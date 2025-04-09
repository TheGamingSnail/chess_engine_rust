use chess::{Board, Color, Piece};
use std::cmp;


static PIECE_VALUES: &'static [u32] = &[100, 300, 300, 500, 900];

pub fn evaluate(board: Board, color: Color) -> i32 {
    // let opposite_color = if color == Color::White { Color::Black } else { Color::White };
    // println!("{:?}", new_board.status());
    if board.status() == chess::BoardStatus::Stalemate {
        return 0;
    }
    // piece bitboards (required for eval anyway, but also work for piece counting)
    let white_piece_bitboard = board.color_combined(Color::White);
    let black_piece_bitboard = board.color_combined(Color::Black);
    // opposite color piece count
    let opposite_piece_count: i32 = if color == Color::White {
        black_piece_bitboard.popcnt() as i32
    } else {
        white_piece_bitboard.popcnt() as i32
    };

    let mut score: i32 = 0;
    let pawn_bitboard = board.pieces(Piece::Pawn);
    let knight_bitboard = board.pieces(Piece::Knight);
    let bishop_bitboard = board.pieces(Piece::Bishop);
    let rook_bitboard = board.pieces(Piece::Rook);
    let queen_bitboard = board.pieces(Piece::Queen);
    
    
    let w_pawn_bitboard = pawn_bitboard & white_piece_bitboard;
    let b_pawn_bitboard = pawn_bitboard & black_piece_bitboard;

    let w_knight_bitboard = knight_bitboard & white_piece_bitboard;
    let b_knight_bitboard = knight_bitboard & black_piece_bitboard;

    let w_bishop_bitboard = bishop_bitboard & white_piece_bitboard;
    let b_bishop_bitboard = bishop_bitboard & black_piece_bitboard;

    let w_rook_bitboard = rook_bitboard & white_piece_bitboard;
    let b_rook_bitboard = rook_bitboard & black_piece_bitboard;

    let w_queen_bitboard = queen_bitboard & white_piece_bitboard;
    let b_queen_bitboard = queen_bitboard & black_piece_bitboard;

    score = score.checked_add_unsigned((w_pawn_bitboard.popcnt()) * PIECE_VALUES[0]).expect("score overflow");
    score = score.checked_add_unsigned((w_knight_bitboard.popcnt()) * PIECE_VALUES[1]).expect("score overflow");
    score = score.checked_add_unsigned((w_bishop_bitboard.popcnt()) * PIECE_VALUES[2]).expect("score overflow");
    score = score.checked_add_unsigned((w_rook_bitboard.popcnt()) * PIECE_VALUES[3]).expect("score overflow");
    score = score.checked_add_unsigned((w_queen_bitboard.popcnt()) * PIECE_VALUES[4]).expect("score overflow");
    
    score = score.checked_sub_unsigned((b_pawn_bitboard.popcnt()) * PIECE_VALUES[0]).expect("score overflow");
    score = score.checked_sub_unsigned((b_knight_bitboard.popcnt()) * PIECE_VALUES[1]).expect("score overflow");
    score = score.checked_sub_unsigned((b_bishop_bitboard.popcnt()) * PIECE_VALUES[2]).expect("score overflow");
    score = score.checked_sub_unsigned((b_rook_bitboard.popcnt()) * PIECE_VALUES[3]).expect("score overflow");
    score = score.checked_sub_unsigned((b_queen_bitboard.popcnt()) * PIECE_VALUES[4]).expect("score overflow");
    
    if opposite_piece_count <= 7 {
        score -= force_king_to_corner(board, if color == Color::White { Color::Black } else { Color::White }, opposite_piece_count);
    
    }
    let perspective: i32 = if color == Color::White { 1 } else { -1 };
    score *= perspective;
    // println!("Score: {}", score);
    return score;
}

fn force_king_to_corner(board: Board, color: Color, piece_count: i32) -> i32 {
    let mut eval = 0;
    let opp_king_square = board.king_square(color);
    let opp_king_x = opp_king_square.get_file() as i32;
    let opp_king_y = opp_king_square.get_rank() as i32;

    let opp_king_dist_to_centre_file = cmp::max(3 - opp_king_x, opp_king_x - 4);
    let opp_king_dist_to_centre_rank = cmp::max(3 - opp_king_y, opp_king_y - 4);
    let opp_king_dist_to_centre = opp_king_dist_to_centre_file + opp_king_dist_to_centre_rank;

    eval += opp_king_dist_to_centre;

    // move king to cut off stuff
    let king_square = board.king_square(if color == Color::White { Color::Black } else { Color::White });
    
    let my_king_x = king_square.get_file() as i32;
    let my_king_y = king_square.get_rank() as i32;

    let dist_between_kings = i32::abs(my_king_x - opp_king_x) + i32::abs(my_king_y - opp_king_y);
    eval += 14 - dist_between_kings;
    let pieces_taken = 20 - piece_count;

    return eval * 10 * pieces_taken;
}
// this is for move ordering, not used in evaluation
pub fn get_piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => PIECE_VALUES[0] as i32,
        Piece::Knight => PIECE_VALUES[1] as i32,
        Piece::Bishop => PIECE_VALUES[2] as i32,
        Piece::Rook => PIECE_VALUES[3] as i32,
        Piece::Queen => PIECE_VALUES[4] as i32,
        Piece::King => 0, // King value is not used in evaluation
    }
}