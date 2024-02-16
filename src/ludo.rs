use std::ops::Rem;

use colored::{ColoredString, Colorize};

pub fn main() {}
#[derive(Clone, Default)]
struct TileState {
    piece: Option<(PieceColours, u32)>,
    tile: Option<PieceColours>,
}
#[derive(Clone, PartialEq)]
enum PieceColours {
    Red,
    Blue,
    Yellow,
    Green,
}
impl Iterator for PieceColours {
    type Item = PieceColours;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PieceColours::Red => Some(PieceColours::Blue),
            PieceColours::Blue => Some(PieceColours::Yellow),
            PieceColours::Yellow => Some(PieceColours::Green),
            PieceColours::Green => Some(PieceColours::Red),
        }
    }
}
struct Move {
    piece: u32,
    distance: u32,
}
fn move_piece(potential_move: Move, player: PieceColours) {}
struct BoardState {
    tile_positions: [(u32, u32); 24], // should be 92
    tiles: Vec<TileState>,
}
// impl Default for BoardState {
//     fn default() -> Self {
//         // Self { tile_positions: [
//         //     (6, 2),
//         //     (6, 1),
//         //     (6, 0),
//         //     (7, 0),
//         //     (8, 0),
//         //     (8, 1),
//         //     (8, 2),
//         //     (8, 3),
//         //     (8, 4),
//         //     (8, 5),
//         //     (9, 6),
//         //     (10, 6),
//         //     (11, 6),
//         //     (12, 6),
//         //     (13, 6),
//         //     (14, 6),
//         //     (14, 7),
//         //     (14, 8),
//         //     (13, 8),
//         //     (12, 8),
//         //     (11, 8),
//         //     (10, 8),
//         //     (9, 8),
//         //     (8, 9),
//         // ], tiles: Default::default() }
//     }
// }
impl BoardState {
    fn new() {}
    fn draw_board(&self) {
        let piece_positions = [
            (6, 2),
            (6, 1),
            (6, 0),
            (7, 0),
            (8, 0),
            (8, 1),
            (8, 2),
            (8, 3),
            (8, 4),
            (8, 5),
            (9, 6),
            (10, 6),
            (11, 6),
            (12, 6),
            (13, 6),
            (14, 6),
            (14, 7),
            (14, 8),
            (13, 8),
            (12, 8),
            (11, 8),
            (10, 8),
            (9, 8),
            (8, 9),
        ];
        let mut blank_board = vec![vec![None::<TileState>; 15]; 15];
        for (index, tile) in self.tiles.iter().enumerate() {
            let (x, y) = piece_positions[index];
            *blank_board.get_mut(x).unwrap().get_mut(y).unwrap() = Some(tile.to_owned())
        }
        for (x, row) in blank_board.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let (foreground, background): (Option<PieceColours>, Option<PieceColours>) =
                    (match tile {
                        Some(tile) => match tile.piece.clone() {
                            Some((colour, _)) => Some(colour),
                            None => None,
                        },
                        None => None,
                    }, match tile {
                        Some(tile) => match tile.tile.clone() {
                            Some(colour) => Some(colour),
                            None => None,
                        },
                        None => None,
                    });
            }
        }
    }
}

enum Position {
    StartArea,
    MainPath(u32),
    FinalPath(u32),
    FinalArea,
}
