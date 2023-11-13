use std::io::Write;

use colored::{ColoredString, Colorize};
fn main() {
    let mut board: BoardState = start_board();
    let mut current_move: PieceMove = PieceMove {
        start_rank: 0,
        start_file: 0,
        end_rank: 0,
        end_file: 0,
    };
    draw_board(&start_board());
    loop {
        let mut line: String = String::default();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.len().eq(&usize::from(u8::from(6))) {
            let bytes: Vec<u8> = line.as_bytes().to_ascii_uppercase();
            let mut success: bool = true;
            if 64 <= bytes[0] && bytes[0] <= 72 {
                current_move.start_file = bytes[0] - 65;
                //println!("{}", bytes[0] - 65);
            } else {
                success = false;
            }
            if 48 < bytes[1] && bytes[1] <= 57 {
                current_move.start_rank = 7 - (bytes[1] - 49);
                //println!("{}", bytes[1] - 49);
            } else {
                success = false;
            }
            if 64 <= bytes[2] && bytes[2] <= 72 {
                current_move.end_file = bytes[2] - 65;
                //println!("{}", bytes[2] - 65);
            } else {
                success = false;
            }
            if 48 < bytes[1] && bytes[1] <= 57 {
                current_move.end_rank = 7-(bytes[3] - 49);
                //println!("{}", bytes[3] - 49);
            } else {
                success = false;
            }
            if success {
                validate_and_play(current_move, &mut board);
            } else {
                println!();
                println!("Invalid move.")
            }
            println!();
            draw_board(&board);
        } else {
            println!("Invalid length for move");
        }
    }
}
fn start_board() -> BoardState {
    let blank_row: TileState = TileState {
        piece: None,
        piece_colour: BoardColours::White,
    };
    let mut state: BoardState = BoardState {
        current_player: BoardColours::White,
        tiles: vec![vec![blank_row; 8]; 8],
    };
    state.tiles[0][0].piece = Some(Pieces::Rook);
    state.tiles[1][0].piece = Some(Pieces::Knight);
    state.tiles[2][0].piece = Some(Pieces::Bishop);
    state.tiles[3][0].piece = Some(Pieces::King);
    state.tiles[4][0].piece = Some(Pieces::Queen);
    state.tiles[5][0].piece = Some(Pieces::Bishop);
    state.tiles[6][0].piece = Some(Pieces::Knight);
    state.tiles[7][0].piece = Some(Pieces::Rook);

    state.tiles[0][1].piece = Some(Pieces::Pawn);
    state.tiles[1][1].piece = Some(Pieces::Pawn);
    state.tiles[2][1].piece = Some(Pieces::Pawn);
    state.tiles[3][1].piece = Some(Pieces::Pawn);
    state.tiles[4][1].piece = Some(Pieces::Pawn);
    state.tiles[5][1].piece = Some(Pieces::Pawn);
    state.tiles[6][1].piece = Some(Pieces::Pawn);
    state.tiles[7][1].piece = Some(Pieces::Pawn);

    state.tiles[0][7].piece = Some(Pieces::Rook);
    state.tiles[1][7].piece = Some(Pieces::Knight);
    state.tiles[2][7].piece = Some(Pieces::Bishop);
    state.tiles[3][7].piece = Some(Pieces::King);
    state.tiles[4][7].piece = Some(Pieces::Queen);
    state.tiles[5][7].piece = Some(Pieces::Bishop);
    state.tiles[6][7].piece = Some(Pieces::Knight);
    state.tiles[7][7].piece = Some(Pieces::Rook);

    state.tiles[0][7].piece_colour = BoardColours::Black;
    state.tiles[1][7].piece_colour = BoardColours::Black;
    state.tiles[2][7].piece_colour = BoardColours::Black;
    state.tiles[3][7].piece_colour = BoardColours::Black;
    state.tiles[4][7].piece_colour = BoardColours::Black;
    state.tiles[5][7].piece_colour = BoardColours::Black;
    state.tiles[6][7].piece_colour = BoardColours::Black;
    state.tiles[7][7].piece_colour = BoardColours::Black;

    state.tiles[0][6].piece = Some(Pieces::Pawn);
    state.tiles[1][6].piece = Some(Pieces::Pawn);
    state.tiles[2][6].piece = Some(Pieces::Pawn);
    state.tiles[3][6].piece = Some(Pieces::Pawn);
    state.tiles[4][6].piece = Some(Pieces::Pawn);
    state.tiles[5][6].piece = Some(Pieces::Pawn);
    state.tiles[6][6].piece = Some(Pieces::Pawn);
    state.tiles[7][6].piece = Some(Pieces::Pawn);

    state.tiles[0][6].piece_colour = BoardColours::Black;
    state.tiles[1][6].piece_colour = BoardColours::Black;
    state.tiles[2][6].piece_colour = BoardColours::Black;
    state.tiles[3][6].piece_colour = BoardColours::Black;
    state.tiles[4][6].piece_colour = BoardColours::Black;
    state.tiles[5][6].piece_colour = BoardColours::Black;
    state.tiles[6][6].piece_colour = BoardColours::Black;
    state.tiles[7][6].piece_colour = BoardColours::Black;
    
    return state;
}

fn validate_and_play(potential_move: PieceMove, board: &mut BoardState) {
    if validate_move(potential_move, board) {
        move_piece(potential_move, board);
        board.current_player = match board.current_player {
            BoardColours::Black => {
                BoardColours::White
            },
            BoardColours::White => {
                BoardColours::Black
            }
        }
    }
}
fn move_piece(intended_move: PieceMove, board: &mut BoardState) {
    println!("Start R: {}", intended_move.start_rank);
    println!("Start F: {}", intended_move.start_file);
    println!("End R: {}", intended_move.end_rank);
    println!("End F: {}", intended_move.end_file);
    board.tiles[usize::from(intended_move.end_file)][usize::from(intended_move.end_rank)] = board.tiles[usize::from(intended_move.start_file)][usize::from(intended_move.start_rank)];
    board.tiles[usize::from(intended_move.start_file)][usize::from(intended_move.start_rank)] = TileState {
        piece: None,
        piece_colour: BoardColours::White,
    };
}
fn validate_move(potential_move: PieceMove, board: &BoardState) -> bool {
    if board.tiles[usize::from(potential_move.start_file)][usize::from(potential_move.start_rank)]
        .piece.is_none() {
            return false; //Checks the starting piece exists
        }
    if board.tiles[usize::from(potential_move.start_file)][usize::from(potential_move.start_rank)].piece_colour != board.current_player {
        return false; //Checks the pieces belong to the player
    }
    if board.tiles[usize::from(potential_move.start_file)][usize::from(potential_move.start_rank)].piece_colour == board.tiles[usize::from(potential_move.end_file)][usize::from(potential_move.end_rank)].piece_colour {
        return false; //Checks the player isn't taking their own pieces
    }



    return true;
}
#[derive(Clone, Copy)]
struct PieceMove {
    start_rank: u8,
    start_file: u8,
    end_rank: u8,
    end_file: u8,
}
#[derive(Clone)]
struct BoardState {
    current_player: BoardColours,
    tiles: Vec<Vec<TileState>>,
}
#[derive(Clone, Copy)]
struct TileState {
    piece: Option<Pieces>,
    piece_colour: BoardColours,
}
#[derive(Clone, Copy, PartialEq)]
enum BoardColours {
    White,
    Black,
}
#[derive(Clone, Copy)]
enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
fn draw_board(board: &BoardState) {
    let mut y: i8 = 0;
    let mut x: i8;
    while y <= 8 {
        x = 0;
        while x <= 8 {
            if x == 0 {
                if y == 8 {
                    print!("  ");
                } else {
                    print!("{} ", 8 - y);
                }
            } else if y == 8 {
                print!("{} ", to_letter(x.unsigned_abs()));
            } else {
                print!(
                    "{}",
                    to_piece_name(
                        &board.tiles[usize::try_from(x.clone() - 1).expect("index out of bounds")]
                            [usize::try_from(y.clone()).expect("index out of bounds")],
                        if (x + y) % 2 == 1 {
                            BoardColours::White
                        } else {
                            BoardColours::Black
                        }
                    )
                )
            }
            x += 1;
        }
        println!();
        y += 1;
    }
    println!();
    println!("{}", "   White's turn   ".black().on_white());
    print!("{}", " White's move:".black().on_white());
    std::io::stdout().flush().unwrap();
}

fn to_piece_name(tile: &TileState, colour: BoardColours) -> ColoredString {
    let mut tile_text: String = match tile.piece {
        Some(_) => match tile.piece_colour {
            BoardColours::White => "w".to_string(),
            BoardColours::Black => "b".to_string(),
        },
        None => " ".to_string(),
    };

    tile_text.push(match &tile.piece {
        Some(piece) => match piece {
            Pieces::Pawn => 'P',
            Pieces::Rook => 'R',
            Pieces::Knight => 'N',
            Pieces::Bishop => 'B',
            Pieces::Queen => 'Q',
            Pieces::King => 'K',
        },
        None => ' ',
    });
    let colored_text: ColoredString;
    return match colour {
        BoardColours::White => {
            colored_text = tile_text.on_white();
            colored_text.black()
        }
        BoardColours::Black => {
            colored_text = tile_text.on_black();
            colored_text.white()
        }
    };
}

fn to_letter(number: u8) -> String {
    let buffer: Vec<u8> = [number + 96].to_vec();
    String::from_utf8(buffer).expect("invalid utf-8 sequence")
}
