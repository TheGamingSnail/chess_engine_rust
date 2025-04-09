use chess::{ChessMove, Game, Square, Color, Piece};
use std::io;
mod engine;
mod eval;

fn main() {
    println!("Welcome to the Chess Game!");
    println!("This is a simple chess game where you can play against a basic AI.");
    println!("The AI will make random moves, and you can enter your moves in standard algebraic notation.");
    println!("To make a move, enter the starting square and the ending square (e.g., e2e4).");
    println!("To exit the game, type 'exit'.");
    println!("Let's start the game!");
    println!("What color do you wish to play as? (white/black):");
    let mut color = String::new();
    io::stdin()
        .read_line(&mut color)
        .expect("Failed to read line");
    let mut color = color.trim().to_lowercase();
    if color != "white" && color != "black" {
        println!("Invalid color choice. Defaulting to white.");
        color = "white".to_string();
    }
    println!("You are playing as {}.", color);
    println!("The AI will play as {}.", if color == "white" { "black" } else { "white" });
    println!("The game is starting now...");
    let player_color = if color == "white" { Color::White } else { Color::Black };
    start_game(player_color);
}

fn start_game(player_color: Color) {
    let game = Game::new();
    // let testing_board = Board::from_str("3r4/3r4/3k4/8/8/8/4K3/8 w - - 0 1").unwrap();
    // let mut game = Game::new_with_board(testing_board);
    // let board = game.current_position();
    let ai_color = if player_color== Color::White { Color::Black} else { Color::White };
    continue_game(game, player_color, ai_color);
}

fn continue_game(mut game: Game, player_color: Color, ai_color: Color) {
    if player_color == game.side_to_move() {
        println!("your move: ");
        let mut player_move = String::new();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");
        let player_move = player_move.trim();
        let square1 = Square::from_string(player_move[..2].to_string()).expect("square 1 missing");
        let square2 = Square::from_string(player_move[2..4].to_string()).expect("square 2 missing");
        let promotion = if player_move.len() > 4 {
            let promotion_piece = match player_move[4..5].to_lowercase().as_str() {
                "q" => Some(Piece::Queen),
                "r" => Some(Piece::Rook),
                "b" => Some(Piece::Bishop),
                "n" => Some(Piece::Knight),
                _ => None,
            };
            promotion_piece
        } else {
            None
        };
        let chess_move = ChessMove::new(square1, square2, promotion);
        game.make_move(chess_move);
        continue_game(game, player_color, ai_color); 
   }
   else
   {
        let ai_move = engine::engine_move(game.current_position(), ai_color);
        println!("AI's move: {}", ai_move);
        game.make_move(ai_move);
        continue_game(game, player_color, ai_color);
   }
}

// fn evaluate() {

// }

// fn engine_move(board: Board, ai_color: Color) -> ChessMove {
//     return MoveGen::new_legal(&board).next().expect("fuk you");
// }
