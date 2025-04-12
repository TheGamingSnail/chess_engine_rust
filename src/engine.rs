use chess::{Board, ChessMove, Color, MoveGen, Piece};
use crate::eval;
use std::collections::HashMap;

// A simple transposition table entry
struct TTEntry {
    depth: u32,
    eval: i32,
    flag: u8, // 0 = exact, 1 = lower bound, 2 = upper bound
    best_move: Option<ChessMove>,
}

// Add this as a global or pass it as a parameter
type TranspositionTable = HashMap<u64, TTEntry>;

// kill moves and shti
struct KillerMoves {
    moves: Vec<[Option<ChessMove>; 2]>,
}
impl KillerMoves {
    fn new(max_depth: usize) -> Self {
        let mut moves = Vec::with_capacity(max_depth);
        for _ in 0..max_depth {
            moves.push([None, None]);
        }
        KillerMoves { moves }
    }
    
    fn add_killer(&mut self, m: ChessMove, ply: usize) {
        if ply >= self.moves.len() {
            return;
        }
        
        if Some(m) != self.moves[ply][0] {
            self.moves[ply][1] = self.moves[ply][0];
            self.moves[ply][0] = Some(m);
        }
    }
    
    fn is_killer(&self, m: &ChessMove, ply: usize) -> bool {
        if ply >= self.moves.len() {
            return false;
        }
        
        self.moves[ply][0] == Some(*m) || self.moves[ply][1] == Some(*m)
    }
}
pub fn engine_move(board: Board, ai_color: Color) -> ChessMove {
    // return search(board, ai_color, 3);
    let mut killers = KillerMoves::new(10);
    let mut tt: TranspositionTable = HashMap::with_capacity(1_000_000);
        match search(board, ai_color, 6, i32::MIN + 1, i32::MAX - 10000, true, &mut tt, &mut killers).0 {
            Some(best_move) => best_move,
            None => {
                // Fallback to any legal move if no best move found
                println!("No best move found, falling back to any legal move.");
                MoveGen::new_legal(&board).next().expect("no legal moves available")
            }
    }
}
// how the fuck do you add null move pruning
fn search(board: Board, ai_color: Color, depth: u32, mut alpha: i32, mut beta: i32, is_pv_node: bool, tt: &mut TranspositionTable, killers: &mut KillerMoves) -> (Option<ChessMove>, i32) {
    let orig_alpha = alpha;
    let zobrist_hash = board.get_hash();
    // transposition table
    if let Some(tt_entry) = tt.get(&zobrist_hash) {
        if tt_entry.depth >= depth {
            match tt_entry.flag {
                0 => return (tt_entry.best_move, tt_entry.eval), // Exact score
                1 => alpha = alpha.max(tt_entry.eval),           // Lower bound
                2 => beta = beta.min(tt_entry.eval),             // Upper bound
                _ => {}
            }
            
            if alpha >= beta {
                return (tt_entry.best_move, tt_entry.eval);
            }
        }
    }
    if depth == 0 {
        return quiescence_search(board, ai_color, alpha, beta, killers);
    }
    
    // Terminal position checks
    if board.status() == chess::BoardStatus::Checkmate {
        return (None, if board.side_to_move() == ai_color { -100000 } else { 100000 });
    }
    if board.status() == chess::BoardStatus::Stalemate {
        return (None, 0);
    }
    
    // Null move pruning (only in non-PV nodes)
    if !is_pv_node && depth >= 3 && board.checkers().popcnt() == 0 {
        let r = if depth > 6 { 3 } else { 2 }; // Dynamic reduction
        let null_board = board.null_move();
        if null_board.is_some() {
            let (_, null_eval) = search(null_board.unwrap(), ai_color, depth - 1 - r, -beta, -beta + 1, false, tt, killers);
            let check_eval = -null_eval;
            if check_eval >= beta {
                return (None, beta);
            }
        }
    }
    
    let mut best_eval = i32::MIN;
    let mut best_move = None;
    let movegen = MoveGen::new_legal(&board);
    let moves = order_moves(movegen, &board, killers, depth as usize);

    
    let mut search_pv = true; // Flag to indicate if we're searching the first move
    let mut i = 0;
    for m in moves {
        i += 1;
        let mut new_board = Board::default();
        board.make_move(m, &mut new_board);
        
        let mut evaluation;
        let is_quiet = board.checkers().popcnt() == 0 && 
               board.piece_on(m.get_dest()).is_none() && 
               m.get_promotion().is_none();
        let next_depth = if is_quiet && depth >= 3 && i > 3 {
                depth - 2  // Apply Late Move Reduction
            } else {
                depth - 1
            };
        // PVS: first move gets full window, others get zero window
        if search_pv {
            // Full window search for first move or PV node
            (_, evaluation) = search(new_board, ai_color, next_depth, -beta, -alpha, true, tt, killers);
            search_pv = false; // No longer searching first move
        } else {
            // Zero window search for non-first moves
            (_, evaluation) = search(new_board, ai_color, next_depth, -alpha - 1, -alpha, false, tt, killers);
            
            // Re-search with full window if it might improve alpha and we're in a PV node
            if is_pv_node && -evaluation > alpha && -evaluation < beta {
                (_, evaluation) = search(new_board, ai_color, next_depth, -beta, -alpha, true, tt, killers);
            }
        }
        
        let negated_eval = -evaluation;
        
        // Update best move and evaluation
        if negated_eval > best_eval {
            best_eval = negated_eval;
            best_move = Some(m);
            
            if negated_eval > alpha {
                alpha = negated_eval;
                
                if alpha >= beta {
                    // Beta cutoff
                    if board.piece_on(m.get_dest()).is_none() {
                        killers.add_killer(m, depth as usize);
                    }
                    break;
                }
            }
        }
    }
    
    // Handle no legal moves
    if best_move.is_none() {
        return (None, eval::evaluate(board, ai_color));
    }
    let flag = if best_eval <= orig_alpha {
        2 // Upper bound (fail-low)
    } else if best_eval >= beta {
        1 // Lower bound (fail-high)
    } else {
        0 // Exact score
    };
    
    tt.insert(zobrist_hash, TTEntry {
        depth,
        eval: best_eval,
        flag,
        best_move,
    });
    (best_move, best_eval)
}

fn order_moves(moves: MoveGen, board: &Board, killers: &KillerMoves, ply: usize) -> Vec<ChessMove> {
    let mut scored_moves: Vec<(ChessMove, i32)> = Vec::new();
    
    for m in moves {
        let mut move_score_guess = 0;
        let move_piece = board.piece_on(m.get_source()).unwrap();
        let capt_piece = board.piece_on(m.get_dest());

        // Score captures
        if let Some(captured) = capt_piece {
            move_score_guess = 10 * eval::get_piece_value(captured) - eval::get_piece_value(move_piece);
        }
        
        // Score promotions
        if let Some(promotion) = m.get_promotion() {
            move_score_guess += eval::get_piece_value(promotion);
        }
        
        // Add bonus for killer moves
        if killers.is_killer(&m, ply) {
            move_score_guess += 900; // Just below good captures, above quiet moves
        }
        
        scored_moves.push((m, move_score_guess));
    }
    
    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));
    scored_moves.into_iter().map(|(m, _)| m).collect()
}

fn quiescence_search(board: Board, ai_color: Color, mut alpha: i32, beta: i32, killers: &mut KillerMoves) -> (Option<ChessMove>, i32) {
    // Stand-pat score - evaluate current position before looking at captures
    let stand_pat = eval::evaluate(board, ai_color);
    
    // Beta cutoff - position is already too good
    if stand_pat >= beta {
        return (None, beta);
    }
    
    // Update alpha if stand-pat is better than current alpha
    if stand_pat > alpha {
        alpha = stand_pat;
    }
    
    // Check terminal conditions
    if board.status() != chess::BoardStatus::Ongoing {
        if board.status() == chess::BoardStatus::Checkmate {
            return (None, if board.side_to_move() == ai_color { -100000 } else { 100000 });
        }
        return (None, 0); // Stalemate or other draw
    }
    
    let mut best_move = None;
    let mut best_eval = stand_pat; // Initialize with standing pat evaluation
    let mut movegen = MoveGen::new_legal(&board);
    
    // Only consider captures and promotions for quiescence
    let opponent_pieces = board.color_combined(!board.side_to_move());
    let checkers = board.checkers();
    
    // If in check, consider all legal moves
    if checkers.popcnt() > 0 {
        movegen.set_iterator_mask(!chess::BitBoard::new(0)); // All squares
    } else {
        // Otherwise just captures and promotions
        movegen.set_iterator_mask(*opponent_pieces);
    }
    
    let moves = order_moves(movegen, &board, killers, 0);
    
    for m in moves {
        // Skip bad captures using SEE (Static Exchange Evaluation)
        if !checkers.popcnt() == 0 && board.piece_on(m.get_dest()).is_some() {
            let moving_piece = board.piece_on(m.get_source()).unwrap();
            let captured_piece = board.piece_on(m.get_dest()).unwrap();
            
            // Skip clearly bad captures (e.g., queen for pawn)
            if eval::get_piece_value(moving_piece) > eval::get_piece_value(captured_piece) + 100 {
                continue;
            }
        }
        
        let mut new_board = Board::default();
        board.make_move(m, &mut new_board);
        
        let (_, evaluation) = quiescence_search(new_board, ai_color, -beta, -alpha, killers);
        let negated_eval = -evaluation;
        
        if negated_eval > best_eval {
            best_eval = negated_eval;
            best_move = Some(m);
            
            if negated_eval > alpha {
                alpha = negated_eval;
                
                if alpha >= beta {
                    break; // Beta cutoff
                }
            }
        }
    }
    
    return (best_move, best_eval);
}