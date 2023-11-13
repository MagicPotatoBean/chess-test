use std::{clone, io::Read};

use colored::{ColoredString, Colorize};
fn main() {
    draw_board(&start_board());

    //let current_state: EnterMoveState = EnterMoveState::Letter1;
    /*
    loop {
        draw_board(&start_board());
        let byte = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok());
        match byte {
            Some(mut value) => match current_state {
                EnterMoveState::Letter1 => {
                    if 64 < value && value < 90 {
                        value = value - 64
                    } else if 97 < value && value < 122 {
                    }
                }
                EnterMoveState::Number1 => todo!(),
                EnterMoveState::Letter2 => todo!(),
                EnterMoveState::Number2 => todo!(),
            },
            None => {
                println!();
                print!("Encountered an error taking input, sorry!");
            }
        }
        match current_state {
            EnterMoveState::Letter1 => todo!(),
            EnterMoveState::Number1 => todo!(),
            EnterMoveState::Letter2 => todo!(),
            EnterMoveState::Number2 => todo!(),
        }
    }
    */
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

struct BoardState {
    current_player: BoardColours,
    tiles: Vec<Vec<TileState>>,
}
enum EnterMoveState {
    Letter1,
    Number1,
    Letter2,
    Number2,
}
#[derive(Clone, Copy)]
struct TileState {
    piece: Option<Pieces>,
    piece_colour: BoardColours,
}
#[derive(Clone, Copy)]
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
}

fn to_piece_name(tile: &TileState, colour: BoardColours) -> ColoredString {
    let mut tile_text: String = match tile.piece {
        Some(_) => match tile.piece_colour {
            BoardColours::White => "W".to_string(),
            BoardColours::Black => "B".to_string(),
        },
        None => (" ".to_string()),
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
fn from_letter(char: String) -> u8 {
    if char.len().gt(&usize::from(u8::from(1))) {
        panic!("from_letter cannot handle strings of length > 1");
    } else {
        64 + *(char
            .as_bytes()
            .first()
            .expect("from_letter enountered an error converting char to u8"))
    }
}
