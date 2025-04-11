use chess::{Board, Color, Piece, ALL_SQUARES, Square};
use std::cmp;


static PIECE_VALUES: &'static [u32] = &[100, 300, 300, 500, 900];

static BLACK_PAWN_TABLE: &'static [i32] = &[ 
 0,  0,  0,  0,  0,  0,  0,  0,
 50, 50, 50, 50, 50, 50, 50, 50,
 10, 10, 20, 30, 30, 20, 10, 10,
 5,  5, 10, 25, 25, 10,  5,  5,
 0,  0,  0, 20, 20,  0,  0,  0,
 5, -5,-10,  0,  0,-10, -5,  5,
 5, 10, 10,-20,-20, 10, 10,  5,
 0,  0,  0,  0,  0,  0,  0,  0
];
static BLACK_KNIGHT_TABLE: &'static [i32] = &[
-50,-40,-30,-30,-30,-30,-40,-50,
-40,-20,  0,  0,  0,  0,-20,-40,
-30,  0, 10, 15, 15, 10,  0,-30,
-30,  5, 15, 20, 20, 15,  5,-30,
-30,  0, 15, 20, 20, 15,  0,-30,
-30,  5, 10, 15, 15, 10,  5,-30,
-40,-20,  0,  5,  5,  0,-20,-40,
-50,-40,-30,-30,-30,-30,-40,-50,
];
static BLACK_BISHOP_TABLE: &'static [i32] = &[
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];
static BLACK_ROOK_TABLE: &'static [i32] = &[
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];
static BLACK_QUEEN_TABLE: &'static [i32] = &[
    -20,-10,-10, -5, -5,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5,  5,  5,  5,  0,-10,
 -5,  0,  5,  5,  5,  5,  0, -5,
  0,  0,  5,  5,  5,  5,  0, -5,
-10,  5,  5,  5,  5,  5,  0,-10,
-10,  0,  5,  0,  0,  0,  0,-10,
-20,-10,-10, -5, -5,-10,-10,-20
];
static BLACK_KING_TABLE_MID: &'static [i32] = &[
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-20,-30,-30,-40,-40,-30,-30,-20,
-10,-20,-20,-20,-20,-20,-20,-10,
 20, 20,  0,  0,  0,  0, 20, 20,
 20, 30, 10,  0,  0, 10, 30, 20
];
static WHITE_PAWN_TABLE: &'static [i32] = &[
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10,-20,-20, 10, 10,  5,
    5, -5,-10,  0,  0,-10, -5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5,  5, 10, 25, 25, 10,  5,  5,
   10, 10, 20, 30, 30, 20, 10, 10,
   50, 50, 50, 50, 50, 50, 50, 50,
    0,  0,  0,  0,  0,  0,  0,  0
];

static WHITE_KNIGHT_TABLE: &'static [i32] = &[
   -50,-40,-30,-30,-30,-30,-40,-50,
   -40,-20,  0,  5,  5,  0,-20,-40,
   -30,  5, 10, 15, 15, 10,  5,-30,
   -30,  0, 15, 20, 20, 15,  0,-30,
   -30,  5, 15, 20, 20, 15,  5,-30,
   -30,  0, 10, 15, 15, 10,  0,-30,
   -40,-20,  0,  0,  0,  0,-20,-40,
   -50,-40,-30,-30,-30,-30,-40,-50
];

static WHITE_BISHOP_TABLE: &'static [i32] = &[
   -20,-10,-10,-10,-10,-10,-10,-20,
   -10,  5,  0,  0,  0,  0,  5,-10,
   -10, 10, 10, 10, 10, 10, 10,-10,
   -10,  0, 10, 10, 10, 10,  0,-10,
   -10,  5,  5, 10, 10,  5,  5,-10,
   -10,  0,  5, 10, 10,  5,  0,-10,
   -10,  0,  0,  0,  0,  0,  0,-10,
   -20,-10,-10,-10,-10,-10,-10,-20
];

static WHITE_ROOK_TABLE: &'static [i32] = &[
    0,  0,  0,  5,  5,  0,  0,  0,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    5, 10, 10, 10, 10, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

static WHITE_QUEEN_TABLE: &'static [i32] = &[
   -20,-10,-10, -5, -5,-10,-10,-20,
   -10,  0,  5,  0,  0,  0,  0,-10,
   -10,  5,  5,  5,  5,  5,  0,-10,
     0,  0,  5,  5,  5,  5,  0, -5,
    -5,  0,  5,  5,  5,  5,  0, -5,
   -10,  0,  5,  5,  5,  5,  0,-10,
   -10,  0,  0,  0,  0,  0,  0,-10,
   -20,-10,-10, -5, -5,-10,-10,-20
];

static WHITE_KING_TABLE_MID: &'static [i32] = &[
    20, 30, 10,  0,  0, 10, 30, 20,
    20, 20,  0,  0,  0,  0, 20, 20,
   -10,-20,-20,-20,-20,-20,-20,-10,
   -20,-30,-30,-40,-40,-30,-30,-20,
   -30,-40,-40,-50,-50,-40,-40,-30,
   -30,-40,-40,-50,-50,-40,-40,-30,
   -30,-40,-40,-50,-50,-40,-40,-30,
   -30,-40,-40,-50,-50,-40,-40,-30
];

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

    score += (w_pawn_bitboard.popcnt() as i32) * PIECE_VALUES[0] as i32;
    score += (w_knight_bitboard.popcnt() as i32) * PIECE_VALUES[1] as i32;
    score += (w_bishop_bitboard.popcnt() as i32) * PIECE_VALUES[2] as i32;
    score += (w_rook_bitboard.popcnt() as i32) * PIECE_VALUES[3] as i32;
    score += (w_queen_bitboard.popcnt() as i32) * PIECE_VALUES[4] as i32;
    
    score -= (b_pawn_bitboard.popcnt() as i32) * PIECE_VALUES[0] as i32;
    score -= (b_knight_bitboard.popcnt() as i32) * PIECE_VALUES[1] as i32;
    score -= (b_bishop_bitboard.popcnt() as i32) * PIECE_VALUES[2] as i32;
    score -= (b_rook_bitboard.popcnt() as i32) * PIECE_VALUES[3] as i32;
    score -= (b_queen_bitboard.popcnt() as i32) * PIECE_VALUES[4] as i32;
    
    if opposite_piece_count <= 7 {
        score -= force_king_to_corner(board, if color == Color::White { Color::Black } else { Color::White }, opposite_piece_count);
    }
    score += piece_square_table_eval(&board, opposite_piece_count);
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

fn piece_square_table_eval(board: &Board, opposite_piece_count: i32) -> i32 {
    let mut eval = 0;
    for square in ALL_SQUARES.iter() {
        let piece_opt = board.piece_on(*square);
        if piece_opt.is_some()
        {
            let piece = piece_opt.unwrap();
            if board.color_on(*square).unwrap() == Color::White {
                match piece
                {
                    Piece::Pawn => {
                        eval += WHITE_PAWN_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Knight => {
                        eval += WHITE_KNIGHT_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Bishop => {
                        eval += WHITE_BISHOP_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Rook => {
                        eval += WHITE_ROOK_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Queen => {
                        eval += WHITE_QUEEN_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::King => {
                        if opposite_piece_count >= 7 {
                            eval += WHITE_KING_TABLE_MID[square.to_index() as usize] as i32;
                        }
                    }
                    _ => {}
                }
            }
            else
            {
                match piece
                {
                    Piece::Pawn => {
                        eval -= BLACK_PAWN_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Knight => {
                        eval -= BLACK_KNIGHT_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Bishop => {
                        eval -= BLACK_BISHOP_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Rook => {
                        eval -= BLACK_ROOK_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::Queen => {
                        eval -= BLACK_QUEEN_TABLE[square.to_index() as usize] as i32;
                    }
                    Piece::King => {
                        if opposite_piece_count >= 7 {
                            eval -= BLACK_KING_TABLE_MID[square.to_index() as usize] as i32;
                        }
                    }
                    _ => {}
                }
            }
        }
    }  
    return eval; 
}