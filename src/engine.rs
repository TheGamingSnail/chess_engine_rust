use chess::{Board, ChessMove, Color, MoveGen, Piece};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::eval;

// Define a struct to hold transposition table entry data
struct TTEntry {
    depth: u32,
    score: i32,
    flag: NodeType,
    best_move: Option<ChessMove>,
}

// Define node types for different search results
#[derive(Clone, Copy)]
enum NodeType {
    Exact,  // Exact score
    Alpha,  // Upper bound (score ≤ alpha)
    Beta,   // Lower bound (score ≥ beta)
}

// Create a lazily initialized global transposition table
lazy_static! {
    static ref TRANSPOSITION_TABLE: Arc<Mutex<HashMap<u64, TTEntry>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn engine_move(board: Board, ai_color: Color) -> ChessMove {
    // Clear the transposition table at the start of a new move calculation
    TRANSPOSITION_TABLE.lock().unwrap().clear();
    
    match search(board, ai_color, 6, i32::MIN + 1, i32::MAX).0 {
        Some(best_move) => best_move,
        None => {
            // Fallback to any legal move if no best move found
            println!("No best move found, falling back to any legal move.");
            MoveGen::new_legal(&board).next().expect("no legal moves available")
        }
    }
}

fn search(board: Board, ai_color: Color, depth: u32, mut alpha: i32, beta: i32) -> (Option<ChessMove>, i32) {
    // Terminal node conditions
    if depth == 0 {
        return search_all_captures(board, ai_color, alpha, beta);
    }
    if board.status() == chess::BoardStatus::Checkmate {
        return (None, if board.side_to_move() == ai_color { 100000 } else { -100000 });
    }
    if board.status() == chess::BoardStatus::Stalemate {
        return (None, 0);
    }
    
    // Get the Zobrist hash of the current board position
    let hash = board.get_hash();
    
    // Check if position is in the transposition table
    let tt_move: Option<ChessMove>;
    {
        let table = TRANSPOSITION_TABLE.lock().unwrap();
        if let Some(entry) = table.get(&hash) {
            if entry.depth >= depth {
                match entry.flag {
                    NodeType::Exact => return (entry.best_move, entry.score),
                    NodeType::Alpha if entry.score <= alpha => return (entry.best_move, alpha),
                    NodeType::Beta if entry.score >= beta => return (entry.best_move, beta),
                    _ => {}
                }
            }
            // Even if depth is insufficient, we can still use the best move for move ordering
            tt_move = entry.best_move;
        } else {
            tt_move = None;
        }
    }
    
    let mut best_eval = i32::MIN;
    let mut best_move = None;
    let movegen = MoveGen::new_legal(&board);
    
    // Order moves - with tt_move first if available
    let moves = order_moves(movegen, &board, tt_move);
    
    // Check if there are any legal moves
    let mut has_moves = false;
    let mut node_type = NodeType::Alpha;
    
    for m in moves {
        has_moves = true;
        let mut new_board = Board::default();
        board.make_move(m, &mut new_board);
        
        if new_board.status() == chess::BoardStatus::Checkmate {
            // If this move leads to checkmate, it's the best move
            return (Some(m), if new_board.side_to_move() == ai_color { 100000 } else { -100000 });
        }
        
        let (_, evaluation) = search(new_board, ai_color, depth - 1, -beta, -alpha);
        let negated_eval = -evaluation;
        
        if negated_eval > best_eval {
            best_eval = negated_eval;
            best_move = Some(m);
            
            if negated_eval > alpha {
                alpha = negated_eval;
                node_type = NodeType::Exact;  // We found an exact score
                
                if alpha >= beta {
                    node_type = NodeType::Beta;  // Score is a lower bound
                    break;  // Beta cutoff
                }
            }
        }
    }
    
    // If no moves were possible, return the original evaluation
    if !has_moves {
        return (None, eval::evaluate(board, ai_color));
    }
    
    // Store result in transposition table
    {
        let mut table = TRANSPOSITION_TABLE.lock().unwrap();
        table.insert(hash, TTEntry {
            depth,
            score: best_eval,
            flag: node_type,
            best_move,
        });
    }
    
    return (best_move, best_eval);
}

fn order_moves(moves: MoveGen, board: &Board, tt_move: Option<ChessMove>) -> Vec<ChessMove> {
    let mut scored_moves: Vec<(ChessMove, i32)> = Vec::new();
    
    for m in moves {
        let mut move_score_guess = 0;
        
        // Prioritize the transposition table move
        if let Some(best_move) = tt_move {
            if m == best_move {
                move_score_guess = 10000;  // Highest priority
            }
        }
        
        let move_piece: Piece = board.piece_on(m.get_source()).unwrap();
        let capt_piece: Option<Piece> = board.piece_on(m.get_dest());

        if capt_piece != None {
            // MVV-LVA (Most Valuable Victim - Least Valuable Aggressor)
            move_score_guess = 10 * eval::get_piece_value(capt_piece.unwrap()) - eval::get_piece_value(move_piece);
        }

        if m.get_promotion() != None {
            move_score_guess += eval::get_piece_value(m.get_promotion().unwrap());
        }

        scored_moves.push((m, move_score_guess));
    }
    
    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Extract just the moves (without scores)
    let ordered_moves: Vec<ChessMove> = scored_moves.into_iter().map(|(m, _)| m).collect();
    
    return ordered_moves;
}

fn search_all_captures(board: Board, ai_color: Color, mut alpha: i32, beta: i32) -> (Option<ChessMove>, i32) {
    // We can also use the transposition table for quiescence search
    let hash = board.get_hash();
    
    {
        let table = TRANSPOSITION_TABLE.lock().unwrap();
        if let Some(entry) = table.get(&hash) {
            // For quiescence search, we don't need to check depth
            match entry.flag {
                NodeType::Exact => return (entry.best_move, entry.score),
                NodeType::Alpha if entry.score <= alpha => return (entry.best_move, alpha),
                NodeType::Beta if entry.score >= beta => return (entry.best_move, beta),
                _ => {}
            }
        }
    }

    let evaluation = eval::evaluate(board, ai_color);
    if evaluation >= beta {
        return (None, evaluation);
    }
    
    alpha = alpha.max(evaluation);
    
    if board.status() == chess::BoardStatus::Stalemate {
        return (None, 0);
    }
    
    let mut best_eval = evaluation;  // Start with the static evaluation
    let mut best_move = None;
    let mut node_type = NodeType::Alpha;
    
    let mut movegen = MoveGen::new_legal(&board);
    let piece_stuff = board.color_combined(!board.side_to_move());
    let pieces_checking = board.checkers();
    let targets = piece_stuff | pieces_checking;
    movegen.set_iterator_mask(targets);
    
    let moves = order_moves(movegen, &board, None);
    
    let mut has_moves = false;
    for m in moves {
        has_moves = true;
        let mut new_board = Board::default();
        board.make_move(m, &mut new_board);
        
        let (_, evaluation) = search_all_captures(new_board, ai_color, -beta, -alpha);
        let negated_eval = -evaluation;
        
        if negated_eval > best_eval {
            best_eval = negated_eval;
            best_move = Some(m);
            
            if negated_eval > alpha {
                alpha = negated_eval;
                node_type = NodeType::Exact;
                
                if alpha >= beta {
                    node_type = NodeType::Beta;
                    break;
                }
            }
        }
    }
    
    // Store result in transposition table
    {
        let mut table = TRANSPOSITION_TABLE.lock().unwrap();
        table.insert(hash, TTEntry {
            depth: 0,  // Depth 0 for quiescence search
            score: best_eval,
            flag: node_type,
            best_move,
        });
    }
    
    if !has_moves {
        return (None, evaluation);
    }
    
    return (best_move, best_eval);
}
