use std::{fmt::Display, io::Write, ops::Rem};

use colored::Colorize;
use rand::Rng;

use crate::readline;

pub fn main() {
    let mut board = BoardState::default();
    let mut sequence: Vec<String> = Vec::new();
    let mut dice: (Option<u32>, Option<u32>) = (None, None);
    let mut current_move = PieceMove::default();
    help_menu();
    loop {
        draw_board(&board);
        if dice.0.is_none() && dice.1.is_none() {
            dice.0 = Some(rand::thread_rng().gen_range(0..=6));
            dice.1 = Some(rand::thread_rng().gen_range(0..=6));
            board.current_player.end_turn();
        }
        let mut dice_roll = 0;
        if let Some(dice1) = dice.0 {
            println!("Dice 2: {}", dice1);
            dice_roll = dice1;
        }
        if let Some(dice2) = dice.1 {
            println!("Dice 1: {}", dice2);
            dice_roll = dice2;
        }
        println!("Move {} Spaces: ", dice_roll);
        if let Some(mut line) = readline!() {
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
                    println!("{}", line);
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
            } else {
                if let Ok(piece) = prse::try_parse!(line, "{}") {
                    current_move.piece = piece;
                    current_move.piece = current_move.piece - 1;
                    current_move.distance = dice_roll;
                    sequence.push(line.to_owned());
                    println!();
                    // validate_and_play(current_move, &mut board);
                } else {
                    println!();
                    println!("Invalid move code")
                }
            }
            println!();
        }
    }
}
// fn validate_and_play(current_move: PieceMove, board: &mut BoardState) {
//     let from_space = &board.spaces[current_move.piece as usize];
//     if current_move.piece + current_move.distance <
// }
#[derive(Default)]
struct PieceMove {
    piece: u32,
    distance: u32,
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
#[derive(Default)]
enum BoardColour {
    Yellow,
    #[default]
    Red,
}
#[derive(Default)]
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
            write!(f, "{: >2}", coloured)
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
fn draw_board(board: &BoardState) {
    println!();
    match board.current_player {
        CheckerColour::Black => {
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
        CheckerColour::White => {
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
    println!();
}
