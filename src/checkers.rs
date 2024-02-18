use std::{io::Write, ops::Rem};

use colored::{ColoredString, Colorize};
const WHITECHECKERSPRITE: char = '\u{026C0}'; //\u{026C0}
const BLACKCHECKERSPRITE: char = '\u{026C2}'; //\u{026C2}
const WHITEKINGSPRITE: char = '\u{026c1}'; //\u{026c1}
const BLACKKINGSPRITE: char = '\u{026c3}'; //\u{026c3}

pub fn main() {
    let mut board = BoardState::default();
    let mut sequence: Vec<String> = Vec::new();
    let mut allow_move = true;
    let mut current_move: PieceMove = PieceMove {
        start_rank: 0,
        start_file: 0,
        end_rank: 0,
        end_file: 0,
    };
    help_menu();
    loop {
        if !allow_move {
            println!("Type \"{}\" to end your turn.", "stop".red());
            println!();
        }

        draw_board(&board, board.current_player.invert());

        
        let mut line: String = String::default();
        let _ = std::io::stdin().read_line(&mut line);
        line = line.trim().to_owned();
        if line.to_lowercase() == "menu" {
            if confirm() {
                println!("Returning to menu");
                return;
            }
        } else if line.to_lowercase() == "history" {
            println!();
            println!("History: ");
            for line in &sequence {
                println!("{line}");
            }
            println!("End of history.");
            println!();
        } else if line.to_lowercase() == "help" {
            help_menu();
        } else if line.to_lowercase() == "reset" {
            if confirm() {
                sequence = Vec::new();
                board = BoardState::default();
            }
        // } else if line.to_lowercase() == "save" {
        //     println!("Please provide a filepath to save the game status to.");
        //     print!("Path: ");
        //     let _ = std::io::stdout().flush();
        //     let mut path = String::default();
        //     let _ = std::io::stdin().read_line(&mut path);
        //     path = path.trim().to_owned();
        //     let res = std::fs::write(path, board.to_string());
        //     if res.is_err() {
        //         println!("Failed to save game status.");
        //     }
        // } else if line.to_lowercase() == "load" {
        //     println!("Please provide a filepath to load the game status from.");
        //     print!("Path: ");
        //     let _ = std::io::stdout().flush();
        //     let mut path = String::default();
        //     let _ = std::io::stdin().read_line(&mut path);
        //     path = path.trim().to_owned();
        //     let file_data = std::fs::read_to_string(path);
        //     match file_data {
        //         Ok(string_data) => board = BoardState::from_string(string_data),
        //         Err(_) => println!("{}", "Failed to load file.".red()),
        //     }
        } else if line.to_lowercase() == "stop" {
            board.current_player = board.current_player.invert();
        } else if line.len().eq(&usize::from(4u8)) {
            let bytes: Vec<u8> = line.as_bytes().to_ascii_uppercase();
            let mut success: bool = true;
            if 64 <= bytes[0] && bytes[0] <= 72 {
                current_move.start_file = bytes[0] - 65;
                //println!("{}", bytes[0] - 65);
            } else {
                success = false;
            }
            if 49 <= bytes[1] && bytes[1] <= 56 {
                current_move.start_rank = bytes[1] - 49;
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
            if 49 <= bytes[1] && bytes[1] <= 56 {
                current_move.end_rank = bytes[3] - 49;
                //println!("{}", bytes[3] - 49);
            } else {
                success = false;
            }
            if success {
                sequence.push(line.clone());
                println!();
                /*if view_board_as == BoardColours::Black {
                    current_move.rotate_self();
                }*/
                validate_and_move(current_move, &mut board, &mut allow_move);
                promote_pieces(&mut board);
            } else {
                println!();
                println!("Invalid move code");
            }
        } else {
            println!();
            println!("Invalid move code length");
        }
        println!();
    }
}
fn promote_pieces(board: &mut BoardState) {
    for tile in &mut board.tiles {
        if tile.first().expect("Board is incorrect size").is_black() {
            tile.get_mut(0).expect("Board is invalid size").promote();
        }
        if tile.get(7).expect("Board is incorrect size").is_white() {
            tile.get_mut(7).expect("Board is invalid size").promote();
        }
    }
}
fn validate_and_move(potential_move: PieceMove, board: &mut BoardState, allow_move: &mut bool) -> bool {
    if board.tiles[usize::from(potential_move.start_file)][usize::from(potential_move.start_rank)]
        .piece_colour
        != board.current_player
    {
        println!("You cant move your opponent's pieces");
        return false; //Checks the moved piece belong to the player
    }
    if board.tiles[usize::from(potential_move.start_file)][usize::from(potential_move.start_rank)]
        .piece_colour
        == board.tiles[usize::from(potential_move.end_file)][usize::from(potential_move.end_rank)]
            .piece_colour
        && board.tiles[usize::from(potential_move.end_file)]
            [usize::from(potential_move.end_rank)]
        .piece.is_some()
    {
        return false; //Checks the player isn't taking their own pieces (and that piece exists/is en passant)
    } //This also prevents them from not moving(e.g. moving to where they are)

    if match board.get_tile(potential_move.start_file.into(), potential_move.start_rank.into()).piece {
        Some(piece) => match piece {
            Pieces::Checker => {
                let result = validate_checker(potential_move, board, *allow_move);
                *allow_move = result;
                result
            },
            Pieces::King => {
                let result = validate_king(potential_move, board, *allow_move);
                *allow_move = result;
                result
            },
        },
        None => false,
    } {
        board.current_player = board.current_player.invert();
    }

    true
}
fn move_piece(intended_move: PieceMove, board: &mut BoardState) {
    board.tiles[usize::from(intended_move.end_file)][usize::from(intended_move.end_rank)] = board
        .get_owned_tile(
            intended_move.start_file.into(),
            intended_move.start_rank.into(),
        );
    board.tiles[usize::from(intended_move.start_file)][usize::from(intended_move.start_rank)] =
        TileState {
            piece: None,
            piece_colour: BoardColours::Black,
        };
}
fn validate_checker(potential_move: PieceMove, board: &mut BoardState, allow_move: bool) -> bool {
    if potential_move.end_file.abs_diff(potential_move.start_file) == 1
        && ((potential_move.end_rank == potential_move.start_rank + 1 && board.current_player == BoardColours::White) || (potential_move.end_rank == potential_move.start_rank - 1
            && board.current_player == BoardColours::Black))
        
        && allow_move
    {
        if board
            .get_tile(
                potential_move.end_file.into(),
                potential_move.end_rank.into(),
            )
            .piece
            .is_none()
        {
            move_piece(potential_move, board);
            true
        } else {
            false
        }
    } else if potential_move.start_file == potential_move.end_file + 2
        && potential_move.start_rank == potential_move.end_rank + 2
    {
        // Taking down-left
        if board
            .get_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.end_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.end_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else if potential_move.start_file == potential_move.end_file + 2
        && potential_move.start_rank + 2 == potential_move.end_rank
    {
        // Taking up-left
        if board
            .get_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.start_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.start_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else if potential_move.start_file + 2 == potential_move.end_file
        && potential_move.start_rank == potential_move.end_rank + 2
    {
        // Taking down-right
        if board
            .get_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.end_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.end_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else if potential_move.start_file + 2 == potential_move.end_file
        && potential_move.start_rank + 2 == potential_move.end_rank
    {
        // Taking up-right
        if board
            .get_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.start_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.start_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else {
        false
    }
}
fn validate_king(potential_move: PieceMove, board: &mut BoardState, allow_move: bool) -> bool {
    if (potential_move.end_file.abs_diff(potential_move.start_file) == 1
        || potential_move.end_rank.abs_diff(potential_move.start_rank) == 1)
        && allow_move
    {
        if board
            .get_tile(
                potential_move.end_file.into(),
                potential_move.end_rank.into(),
            )
            .piece
            .is_none()
        {
            move_piece(potential_move, board);
            true
        } else {
            false
        }
    } else if potential_move.start_file == potential_move.end_file + 2
        && potential_move.start_rank == potential_move.end_rank + 2
    {
        // Taking down-left
        if board
            .get_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.end_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.end_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else if potential_move.start_file == potential_move.end_file + 2
        && potential_move.start_rank + 2 == potential_move.end_rank
    {
        // Taking up-left
        if board
            .get_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.start_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.end_file + 1).into(),
                (potential_move.start_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else if potential_move.start_file + 2 == potential_move.end_file
        && potential_move.start_rank == potential_move.end_rank + 2
    {
        // Taking down-right
        if board
            .get_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.end_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.end_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else if potential_move.start_file + 2 == potential_move.end_file
        && potential_move.start_rank + 2 == potential_move.end_rank
    {
        // Taking up-right
        if board
            .get_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.start_rank + 1).into(),
            )
            .is_opps(board.current_player)
            && board
                .get_tile(
                    potential_move.end_file.into(),
                    potential_move.end_rank.into(),
                )
                .piece
                .is_none()
        {
            move_piece(potential_move, board);
            board.set_tile(
                (potential_move.start_file + 1).into(),
                (potential_move.start_rank + 1).into(),
                TileState {
                    piece: None,
                    piece_colour: BoardColours::Black,
                },
            );
        }
        false
    } else {
        false
    }
}
#[derive(Clone, Copy, Debug)]
struct PieceMove {
    start_rank: u8,
    start_file: u8,
    end_rank: u8,
    end_file: u8,
}
fn help_menu() {
    println!();
    println!("Type \"{}\" to return to the menu.", "menu".red());
    println!(
        "Type \"{}\" to get the history of the game.",
        "history".red()
    );
    println!("Type \"{}\" to display this menu.", "help".red());
    println!("Type \"{}\" to end your turn (only required when taking a piece)", "stop".red());
    println!("Type \"{}\" to restart the game.", "reset".red());
    // println!("Type \"{}\" to save the state of the board.", "save".red());
    // println!("Type \"{}\" to load the state of the board.", "load".red());
    println!();
}
fn confirm() -> bool {
    print!("Are you sure? [y/n]: ");
    let _ = std::io::stdout().flush();
    let mut line = String::default();
    let _ = std::io::stdin().read_line(&mut line);
    line.trim().eq_ignore_ascii_case("y")
}
#[derive(PartialEq, Clone, Copy, Debug)]
enum BoardColours {
    White,
    Black,
}
impl BoardColours {
    fn invert(&self) -> Self {
        match self {
            BoardColours::White => BoardColours::Black,
            BoardColours::Black => BoardColours::White,
        }
    }
}
#[derive(Clone, Copy, Debug)]
enum Pieces {
    Checker,
    King,
}
#[derive(Clone, Debug)]
struct TileState {
    piece: Option<Pieces>,
    piece_colour: BoardColours,
}
impl TileState {
    fn to_string(&self) -> ColoredString {
        let self_string: ColoredString;
        match self.piece {
            Some(piece) => match piece {
                Pieces::Checker => {
                    
                    let mut sprite = match self.piece_colour {
                        BoardColours::White => WHITECHECKERSPRITE,
                        BoardColours::Black => BLACKCHECKERSPRITE,
                    }.to_string();
                    sprite.push(' ');
                    self_string = sprite.as_str().into();
                }
                Pieces::King => {
                    let mut sprite = match self.piece_colour{
                        BoardColours::White => WHITEKINGSPRITE,
                        BoardColours::Black => BLACKKINGSPRITE,
                    }.to_string();
                    sprite.push(' ');
                    self_string = sprite.as_str().into();
                }
            },
            None => self_string = "  ".into(),
        }
        self_string
    }
    fn is_white(&self) -> bool {
        match self.piece {
            Some(_) => match self.piece_colour {
                BoardColours::White => true,
                BoardColours::Black => false,
            },
            None => false,
        }
    }
    fn is_black(&self) -> bool {
        match self.piece {
            Some(_) => match self.piece_colour {
                BoardColours::White => false,
                BoardColours::Black => true,
            },
            None => false,
        }
    }
    // fn is_ally(&self, as_player: BoardColours) -> bool {
    //     if self.piece.is_some() {
    //         self.piece_colour == as_player
    //     } else {
    //         false // Pieces aren't "Some"
    //     }
    // }
    fn is_opps(&self, as_player: BoardColours) -> bool {
        if self.piece.is_some() {
            self.piece_colour != as_player
        } else {
            false // Pieces aren't "Some"
        }
    }
    fn promote(&mut self) {
        self.piece = self.piece.map(|piece| match piece {
            Pieces::Checker => Pieces::King,
            Pieces::King => Pieces::King,
        });
    }
}
struct BoardState {
    current_player: BoardColours,
    tiles: Vec<Vec<TileState>>,
}
impl BoardState {
    fn get_tile(&self, file: usize, rank: usize) -> &TileState {
        &self.tiles[file][rank]
    }
    fn get_owned_tile(&self, file: usize, rank: usize) -> TileState {
        self.tiles[file][rank].clone()
    }
    fn set_tile(&mut self, file: usize, rank: usize, tile: TileState) {
        self.tiles[file][rank] = tile;
    }
    // fn clone(&self) -> BoardState {
    //     BoardState {
    //         current_player: self.current_player,
    //         tiles: self.tiles.to_owned(),
    //     }
    // }
}
impl Default for BoardState {
    fn default() -> Self {
        let blank_row: TileState = TileState {
            piece: None,
            piece_colour: BoardColours::White,
        };
        let mut state: BoardState = BoardState {
            current_player: BoardColours::White,
            tiles: vec![vec![blank_row; 8]; 8],
        };
        for x in 0..=7 {
            for y in 0..=7 {
                if y <= 2 || y >= 5 {
                    let colour = {
                        if y <= 2 {
                            BoardColours::White
                        } else {
                            BoardColours::Black
                        }
                    };
                    if (x + y).rem(2) == 1 {
                        let tile = state
                            .tiles
                            .get_mut(x as usize)
                            .unwrap()
                            .get_mut(y as usize)
                            .unwrap();
                        tile.piece = Some(Pieces::Checker);
                        tile.piece_colour = colour;
                    } else {
                        state
                            .tiles
                            .get_mut(x as usize)
                            .unwrap()
                            .get_mut(y as usize)
                            .unwrap()
                            .piece = None;
                    }
                }
            }
        }
        state
    }
}

fn draw_board(board: &BoardState, as_player: BoardColours) {
    if as_player == BoardColours::Black {
        let mut y: i8 = 8;
        let mut x: i8;
        while y >= 0 {
            x = 0;
            while x <= 8 {
                if x == 0 {
                    if y == 0 {
                        print!("  ");
                    } else {
                        print!("{y} ");
                    }
                } else if y == 0 {
                    print!("{} ", to_letter((x).unsigned_abs()));
                } else {
                    print!(
                        "{}",
                        to_piece_name(
                            &board.tiles[usize::try_from(x - 1).expect("index out of bounds")]
                                [usize::try_from(y - 1).expect("index out of bounds")],
                            if (x + y) % 2 == 1 {
                                BoardColours::Black
                            } else {
                                BoardColours::White
                            }
                        )
                    );
                }
                x += 1;
            }
            println!();
            y -= 1;
        }
    } else {
        let mut y: i8 = 0;
        let mut x: i8;
        while y <= 8 {
            x = 8;
            while x >= 0 {
                if x == 8 {
                    if y == 8 {
                        print!("  ");
                    } else {
                        print!("{} ", y + 1);
                    }
                } else if y == 8 {
                    print!("{} ", to_letter(x.unsigned_abs() + 1));
                } else {
                    print!(
                        "{}",
                        to_piece_name(
                            &board.tiles[usize::try_from(x).expect("index out of bounds")]
                                [usize::try_from(y).expect("index out of bounds")],
                            if (x + y) % 2 == 1 {
                                BoardColours::Black
                            } else {
                                BoardColours::White
                            }
                        )
                    );
                }
                x -= 1;
            }
            println!();
            y += 1;
        }
    }
    println!();
    match board.current_player {
        BoardColours::Black => {
            println!("{}", "   Black's turn   ".white().on_bright_black());
            print!("{}", " Black's move:".white().on_bright_black());
        }
        BoardColours::White => {
            println!("{}", "   White's turn   ".black().on_white());
            print!("{}", " White's move:".black().on_white());
        }
    }
    let _ = std::io::stdout().flush();
}

fn to_piece_name(tile: &TileState, colour: BoardColours) -> ColoredString {
    match colour {
        BoardColours::White => tile.to_string().on_black().white(),
        BoardColours::Black => tile.to_string().on_white().black(),
    }
}

fn to_letter(number: u8) -> String {
    let buffer: Vec<u8> = [number + 96].to_vec();
    String::from_utf8(buffer).expect("invalid utf-8 sequence")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_opps_test() {
        let opp = TileState {
            piece: Some(Pieces::Checker),
            piece_colour: BoardColours::White,
        };
        let opp2 = TileState {
            piece: None,
            piece_colour: BoardColours::White,
        };
        assert!(opp.is_opps(BoardColours::Black));
        assert!(!opp2.is_opps(BoardColours::Black));
    }
}
