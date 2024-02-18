use std::{
    fmt::Display,
    io::Write,
    ops::{Index, IndexMut, Rem},
    u32,
};

use colored::Colorize;

use crate::{dice::Dice, readline};

pub fn main() {
    let mut board = BoardState::default();
    let mut dice: (Option<Dice>, Option<Dice>) = (None, None);
    let mut current_move = PieceMove::default();
    help_menu();
    loop {
        if dice.0.is_none() && dice.1.is_none() {
            dice.0 = Some(Dice::default());
            dice.1 = Some(Dice::default());
            board.end_turn();
        }
        draw_board(&board);
        println!(
            "White : Black = {} : {}",
            board.scores[CheckerColour::White],
            board.scores[CheckerColour::Black]
        );
        if board.scores[CheckerColour::White] == 15 {
            println!("White wins!");
            break;
        }
        if board.scores[CheckerColour::Black] == 15 {
            println!("Black wins!");
            break;
        }
        let mut dice_roll = Dice::default();
        print!("Dice: ");
        if let Some(dice2) = dice.1 {
            print!("{dice2}");
            dice_roll = dice2;
        }
        if let Some(dice1) = dice.0 {
            println!("{dice1}");
            dice_roll = dice1;
        } else {
            println!();
        }
        println!("Select a piece to move {dice_roll} Spaces: ");
        match board.current_player {
            CheckerColour::Black => {
                println!("{}", "   Black's turn   ".white().on_bright_black());
                print!("{}", " Black's move:".white().on_bright_black());
            }
            CheckerColour::White => {
                println!("{}", "   White's turn   ".black().on_white());
                print!("{}", " White's move:".black().on_white());
            }
        }
        std::io::stdout().flush().unwrap();
        if let Some(mut line) = readline!() {
            line = line.trim().to_owned();
            if line.to_lowercase() == "menu" {
                if confirm() {
                    println!("Returning to menu");
                    return;
                }
            } else if line.to_lowercase() == "help" {
                help_menu();
            } else if line.to_lowercase() == "reset" {
                if confirm() {
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
                if dice.0.is_some() {
                    dice.0 = None;
                } else if dice.1.is_some() {
                    dice.1 = None;
                }
            } else if let Ok(piece) = prse::try_parse!(line, "{}") {
                current_move.piece = piece;
                current_move.piece -= 1;
                current_move.distance = i32::from(u8::from(dice_roll));
                println!();
                if !validate_and_play(current_move, &mut board) {
                    println!("Invalid move");
                } else if dice.0.is_some() {
                    dice.0 = None;
                } else if dice.1.is_some() {
                    dice.1 = None;
                }
            } else {
                println!();
                println!("Invalid move code");
            }
            println!();
        }
    }
}
fn validate_and_play(current_move: PieceMove, board: &mut BoardState) -> bool {
    if match board.current_player {
        CheckerColour::Black => current_move.piece + current_move.distance == 24,
        CheckerColour::White => current_move.piece - current_move.distance == -1,
    } {
        match board.spaces.get_mut(current_move.piece as usize) {
            Some(from_move) => {
                from_move.count -= 1;
                board.scores[board.current_player] += 1;
                true
            }
            None => false,
        }
    } else {
        match board.current_player {
            CheckerColour::Black => {
                if is_valid_move(current_move, board) {
                    (match board.spaces.get_mut(current_move.piece as usize) {
                        Some(from_move) => {
                            from_move.count -= 1;
                            true
                        }
                        None => false,
                    }) && (match board
                        .spaces
                        .get_mut((current_move.piece + current_move.distance) as usize)
                    {
                        Some(to_move) => {
                            if to_move.checker == board.current_player {
                                to_move.count += 1;
                                true
                            } else {
                                to_move.count = 1;
                                to_move.checker = board.current_player;
                                true
                            }
                        }
                        None => panic!("Unstable game state, aborting."),
                    })
                } else {
                    false
                }
            }
            CheckerColour::White => {
                if is_valid_move(current_move, board) {
                    (match board.spaces.get_mut(current_move.piece as usize) {
                        Some(from_move) => {
                            from_move.count -= 1;
                            true
                        }
                        None => false,
                    }) && (match board
                        .spaces
                        .get_mut((current_move.piece - current_move.distance) as usize)
                    {
                        Some(to_move) => {
                            if to_move.checker == board.current_player {
                                to_move.count += 1;
                                true
                            } else {
                                to_move.count = 1;
                                to_move.checker = board.current_player;
                                true
                            }
                        }
                        None => panic!("Unstable game state, aborting."),
                    })
                } else {
                    false
                }
            }
        }
    }
}
fn is_valid_move(current_move: PieceMove, board: &mut BoardState) -> bool {
    if match board.current_player {
        CheckerColour::Black => {
            current_move.piece + current_move.distance > 24
                || current_move.piece + current_move.distance < 0
        }
        CheckerColour::White => {
            current_move.piece - current_move.distance > 24
                || current_move.piece - current_move.distance < 0
        }
    } {
        return false;
    }
    (match board.spaces.get(current_move.piece as usize) {
        Some(from_move) => from_move.checker == board.current_player && from_move.count > 0,
        None => false,
    }) && (match board.spaces.get(
        (match board.current_player {
            CheckerColour::Black => current_move.piece + current_move.distance,
            CheckerColour::White => current_move.piece - current_move.distance,
        }) as usize,
    ) {
        Some(to_move) => {
            if to_move.checker == board.current_player {
                true
            } else {
                to_move.count <= 1
            }
        }
        None => match board.current_player {
            CheckerColour::Black => current_move.piece + current_move.distance == 24,
            CheckerColour::White => current_move.piece - current_move.distance == -1,
        },
    })
}
#[derive(Default, Copy, Clone)]
struct PieceMove {
    piece: i32,
    distance: i32,
}
fn help_menu() {
    println!();
    println!("Type \"{}\" to return to the menu.", "menu".red());
    println!(
        "Type \"{}\" to get the history of the game.",
        "history".red()
    );
    println!("Type \"{}\" to show this menu.", "help".red());
    println!("Type \"{}\" to restart the game.", "reset".red());
    println!("Type \"{}\" to save the state of the board.", "save".red());
    println!("Type \"{}\" to load the state of the board.", "load".red());
    println!();
}
fn confirm() -> bool {
    print!("Are you sure? [y/n]: ");
    let _ = std::io::stdout().flush();
    let mut line = String::default();
    let _ = std::io::stdin().read_line(&mut line);
    line.trim().eq_ignore_ascii_case("y")
}
#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum CheckerColour {
    Black,
    #[default]
    White,
}
impl CheckerColour {
    fn end_turn(&mut self) {
        *self = match self {
            CheckerColour::Black => CheckerColour::White,
            CheckerColour::White => CheckerColour::Black,
        }
    }
}
#[derive(Default)]
struct SpaceState {
    checker: CheckerColour,
    count: u32,
}
impl SpaceState {
    fn new(colour: CheckerColour, count: u32) -> Self {
        Self {
            checker: colour,
            count,
        }
    }
}
// #[derive(Default)]
struct BoardState {
    spaces: [SpaceState; 24],
    current_player: CheckerColour,
    scores: Scores,
}
impl Default for BoardState {
    fn default() -> Self {
        Self {
            spaces: [
                SpaceState::new(CheckerColour::Black, 2),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::new(CheckerColour::White, 5),
                SpaceState::default(),
                SpaceState::new(CheckerColour::White, 3),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::new(CheckerColour::Black, 5),
                SpaceState::new(CheckerColour::White, 5),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::new(CheckerColour::Black, 3),
                SpaceState::default(),
                SpaceState::new(CheckerColour::Black, 5),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::default(),
                SpaceState::new(CheckerColour::White, 2),
            ],
            current_player: Default::default(),
            scores: Default::default(),
        }
    }
}
impl Display for SpaceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.count > 0 {
            let uncoloured = format!("{: >2}", self.count);
            let coloured = match self.checker {
                CheckerColour::Black => uncoloured.black(),
                CheckerColour::White => uncoloured.bright_white(),
            };
            write!(f, "{coloured: >2}")
        } else {
            write!(f, "  ")
        }
    }
}
impl BoardState {
    fn end_turn(&mut self) {
        self.current_player.end_turn();
    }
}
#[derive(Default)]
struct Scores {
    white: u32,
    black: u32,
}
impl Index<CheckerColour> for Scores {
    type Output = u32;

    fn index(&self, index: CheckerColour) -> &Self::Output {
        match index {
            CheckerColour::Black => &self.black,
            CheckerColour::White => &self.white,
        }
    }
}
impl IndexMut<CheckerColour> for Scores {
    fn index_mut(&mut self, index: CheckerColour) -> &mut Self::Output {
        match index {
            CheckerColour::Black => &mut self.black,
            CheckerColour::White => &mut self.white,
        }
    }
}
fn draw_board(board: &BoardState) {
    println!();
    match board.current_player {
        CheckerColour::White => {
            println!("{: >24}", "╭Black─home╮");
            for y in 0..14 {
                for x in 0..12 {
                    if y == 0 {
                        if x.rem(2) == 0 {
                            print!("{}", format!("{: >2}", 13 + x).on_red());
                        } else {
                            print!("{}", format!("{: >2}", 13 + x).on_yellow());
                        }
                    } else if y == 13 {
                        if x.rem(2) == 0 {
                            print!("{}", format!("{: >2}", 12 - x).on_red());
                        } else {
                            print!("{}", format!("{: >2}", 12 - x).on_yellow());
                        }
                    } else if y == 2 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB56}".bright_red().on_black());
                            print!("{}", "\u{1FB61}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB56}".bright_yellow().on_black());
                            print!("{}", "\u{1FB61}".bright_yellow().on_black());
                        }
                    } else if y == 3 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB66}".bright_red().on_black());
                            print!("{}", "\u{1FB5B}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB66}".bright_yellow().on_black());
                            print!("{}", "\u{1FB5B}".bright_yellow().on_black());
                        }
                    } else if y == 11 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB45}".bright_red().on_black());
                            print!("{}", "\u{1FB50}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB45}".bright_yellow().on_black());
                            print!("{}", "\u{1FB50}".bright_yellow().on_black());
                        }
                    } else if y == 10 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB4B}".bright_red().on_black());
                            print!("{}", "\u{1FB40}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB4B}".bright_yellow().on_black());
                            print!("{}", "\u{1FB40}".bright_yellow().on_black());
                        }
                    } else if 10 > y && y > 3 {
                        print!("{}", "  ".on_black());
                    } else if y == 12 {
                        if x.rem(2) == 0 {
                            print!(
                                "{: >2}",
                                format!("{}", board.spaces[11 - x]).on_bright_red()
                            );
                        } else {
                            print!(
                                "{: >2}",
                                format!("{}", board.spaces[11 - x]).on_bright_yellow()
                            );
                        }
                    } else if y == 1 {
                        if x.rem(2) == 0 {
                            print!(
                                "{: >2}",
                                format!("{}", board.spaces[12 + x]).on_bright_red()
                            );
                        } else {
                            print!(
                                "{: >2}",
                                format!("{}", board.spaces[12 + x]).on_bright_yellow()
                            );
                        }
                    }
                }
                println!();
            }
            println!("{: >24}", "╰White─home╯");
        }
        CheckerColour::Black => {
            println!("{: <24}", "╭White─home╮");
            for y in 0..14 {
                for x in 0..12 {
                    if y == 0 {
                        if x.rem(2) == 0 {
                            print!("{}", format!("{: >2}", 1 + x).on_red());
                        } else {
                            print!("{}", format!("{: >2}", 1 + x).on_yellow());
                        }
                    } else if y == 13 {
                        if x.rem(2) == 0 {
                            print!("{}", format!("{: >2}", 24 - x).on_red());
                        } else {
                            print!("{}", format!("{: >2}", 24 - x).on_yellow());
                        }
                    } else if y == 2 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB56}".bright_red().on_black());
                            print!("{}", "\u{1FB61}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB56}".bright_yellow().on_black());
                            print!("{}", "\u{1FB61}".bright_yellow().on_black());
                        }
                    } else if y == 3 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB66}".bright_red().on_black());
                            print!("{}", "\u{1FB5B}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB66}".bright_yellow().on_black());
                            print!("{}", "\u{1FB5B}".bright_yellow().on_black());
                        }
                    } else if y == 11 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB45}".bright_red().on_black());
                            print!("{}", "\u{1FB50}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB45}".bright_yellow().on_black());
                            print!("{}", "\u{1FB50}".bright_yellow().on_black());
                        }
                    } else if y == 10 {
                        if x.rem(2) == 0 {
                            print!("{}", "\u{1FB4B}".bright_red().on_black());
                            print!("{}", "\u{1FB40}".bright_red().on_black());
                        } else {
                            print!("{}", "\u{1FB4B}".bright_yellow().on_black());
                            print!("{}", "\u{1FB40}".bright_yellow().on_black());
                        }
                    } else if 10 > y && y > 3 {
                        print!("{}", "  ".on_black());
                    } else if y == 1 {
                        if x.rem(2) == 0 {
                            print!("{: >2}", format!("{}", board.spaces[x]).on_bright_red());
                        } else {
                            print!("{: >2}", format!("{}", board.spaces[x]).on_bright_yellow());
                        }
                    } else if y == 12 {
                        if x.rem(2) == 0 {
                            print!(
                                "{: >2}",
                                format!("{}", board.spaces[23 - x]).on_bright_red()
                            );
                        } else {
                            print!(
                                "{: >2}",
                                format!("{}", board.spaces[23 - x]).on_bright_yellow()
                            );
                        }
                    }
                }
                println!();
            }
            println!("{: <24}", "╰Black─home╯");
        }
    }
}

