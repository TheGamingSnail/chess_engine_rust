use chess::{Board, ChessMove, Color, MoveGen, Piece};
use std::collections::HashMap;
use crate::eval;


pub fn engine_move(board: Board, ai_color: Color) -> ChessMove {
    // return search(board, ai_color, 3);
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
    if depth == 0 {
        return search_all_captures(board, ai_color, alpha, beta);
    }
    if board.status() == chess::BoardStatus::Checkmate {
        return (None, if board.side_to_move() == ai_color { -100000 } else { 100000 });
    }
    if board.status() == chess::BoardStatus::Stalemate {
        return (None, 0);
    }
    
    let mut best_eval = i32::MIN;
    let mut best_move = None;
    let movegen = MoveGen::new_legal(&board);
    let moves = order_moves(movegen, &board);
    
    // Check if there are any legal moves
    let mut has_moves = false;
    for m in moves {
        has_moves = true;
        let mut new_board = Board::default();
        board.make_move(m, &mut new_board);
        if new_board.status() == chess::BoardStatus::Checkmate {
            // If this move leads to checkmate, it's the best move
            // println!("Checkmate move found: {:?}", m);
            return (Some(m), if new_board.side_to_move() == ai_color { -100000 } else { 100000 });
        }
        // let hash = new_board.get_hash();
        // let mut negated_eval = 0;
        // let mut table = TRANSPOSITION_TABLE.lock().unwrap();
        // if table.contains_key(&hash.to_string()) {
        //     let eval = *table.get(&hash.to_string()).unwrap();
        //     negated_eval = -eval;
        // } else {
        //     let (_, evaluation) = search(new_board, ai_color, depth - 1, -beta, -alpha);
        //     negated_eval = -evaluation;
        //     table.insert(hash.to_string(), evaluation);
        // }
        let (_, evaluation) = search(new_board, ai_color, depth - 1, -beta, -alpha);
        let negated_eval = -evaluation;
        
        if negated_eval > best_eval {
            best_eval = negated_eval;
            best_move = Some(m);
        }
        
        if negated_eval > alpha {
            alpha = negated_eval;
        }
        
        if alpha >= beta {
            // Beta cutoff - the position is too good, opponent won't allow this
            break;
        }
    }
    
    // If no moves were possible, return the original evaluation
    if !has_moves {
        // This should never happen since we check for checkmate/stalemate above
        return (None, eval::evaluate(board, ai_color));
    }
    
    return (best_move, best_eval);
}

fn order_moves(moves: MoveGen, board: &Board) -> Vec<ChessMove>{
    let mut scored_moves: Vec<(ChessMove, i32)> = Vec::new();
    for m in moves {
        let mut move_score_guess = 0;
        let move_piece: Piece = board.piece_on(m.get_source()).unwrap();
        let capt_piece: Option<Piece> = board.piece_on(m.get_dest());

        if capt_piece != None {
            move_score_guess = 10 * eval::get_piece_value(capt_piece.unwrap()) - eval::get_piece_value(move_piece);  // Capture
        }

        if m.get_promotion() != None {
            move_score_guess += eval::get_piece_value(m.get_promotion().unwrap());
        }

        scored_moves.push((m, move_score_guess));

    }
    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Extract just the moves (without scores)
    let ordered_moves: Vec<ChessMove> = scored_moves.into_iter().map(|(m, _)| m).collect();
    
    return ordered_moves

}

fn search_all_captures(board: Board, ai_color: Color, mut alpha: i32, beta: i32) -> (Option<ChessMove>, i32) {

    let evaluation = eval::evaluate(board, ai_color);
    if evaluation >= beta {
        return (None, evaluation);
    }
    alpha = alpha.max(evaluation);
    // if board.status() == chess::BoardStatus::Checkmate {
    //     return (None, if board.side_to_move() == ai_color { -100000 } else { 100000 });
    // }
    if board.status() == chess::BoardStatus::Stalemate {
        return (None, 0);
    }
    
    let mut best_eval = i32::MIN;
    let mut best_move = None;
    let mut movegen = MoveGen::new_legal(&board);
    let piece_stuff = board.color_combined(!board.side_to_move());
    let pieces_checking = board.checkers();
    let targets = piece_stuff  | pieces_checking;
    movegen.set_iterator_mask(targets);
    let moves = order_moves(movegen, &board);
    
    // Check if there are any legal moves
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
        }
        
        if negated_eval > alpha {
            alpha = negated_eval;
        }
        
        if alpha >= beta {
            // Beta cutoff - the position is too good, opponent won't allow this
            break;
        }
    }
    
    // If no moves were possible, return the original evaluation
    if !has_moves {
        // This should never happen since we check for checkmate/stalemate above
        return (None, eval::evaluate(board, ai_color));
    }
    
    return (best_move, best_eval);
}