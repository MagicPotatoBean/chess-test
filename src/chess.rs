use std::io::Write;
use colored::{ColoredString, Colorize};
pub fn main() {
    let mut board: BoardState = start_board(BoardColours::White);
    let mut current_move: PieceMove = PieceMove {
        start_rank: 0,
        start_file: 0,
        end_rank: 0,
        end_file: 0,
    };
    let mut turn_count: i32 = 0;
    println!("Type \"exit\" to leave the game.");
    loop {
        draw_board(&board, board.current_player.invert());

        let mut line: String = String::default();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.to_lowercase() == "exit" {
            println!("Returning to menu");
        }
        if line.len().eq(&usize::from(u8::from(6))) {
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
                println!();
                /*if view_board_as == BoardColours::Black {
                    current_move.rotate_self();
                }*/
                validate_and_play(current_move, &mut board, &turn_count);
                turn_count += 1;
            } else {
                println!();
                println!("Invalid move code")
            }
        } else {
            println!();
            println!("Invalid move code length");
        }
        println!();
    }
}
fn start_board(as_player: BoardColours) -> BoardState {
    let blank_row: TileState = TileState {
        piece: None,
        piece_colour: BoardColours::Black,
    };
    let mut state: BoardState = BoardState {
        current_player: as_player.invert(),
        tiles: vec![vec![blank_row; 8]; 8],
        white_can_castle: false,
        black_can_castle: false,
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

    state.tiles[0][7].piece_colour = BoardColours::White;
    state.tiles[1][7].piece_colour = BoardColours::White;
    state.tiles[2][7].piece_colour = BoardColours::White;
    state.tiles[3][7].piece_colour = BoardColours::White;
    state.tiles[4][7].piece_colour = BoardColours::White;
    state.tiles[5][7].piece_colour = BoardColours::White;
    state.tiles[6][7].piece_colour = BoardColours::White;
    state.tiles[7][7].piece_colour = BoardColours::White;

    state.tiles[0][6].piece = Some(Pieces::Pawn);
    state.tiles[1][6].piece = Some(Pieces::Pawn);
    state.tiles[2][6].piece = Some(Pieces::Pawn);
    state.tiles[3][6].piece = Some(Pieces::Pawn);
    state.tiles[4][6].piece = Some(Pieces::Pawn);
    state.tiles[5][6].piece = Some(Pieces::Pawn);
    state.tiles[6][6].piece = Some(Pieces::Pawn);
    state.tiles[7][6].piece = Some(Pieces::Pawn);

    state.tiles[0][6].piece_colour = BoardColours::White;
    state.tiles[1][6].piece_colour = BoardColours::White;
    state.tiles[2][6].piece_colour = BoardColours::White;
    state.tiles[3][6].piece_colour = BoardColours::White;
    state.tiles[4][6].piece_colour = BoardColours::White;
    state.tiles[5][6].piece_colour = BoardColours::White;
    state.tiles[6][6].piece_colour = BoardColours::White;
    state.tiles[7][6].piece_colour = BoardColours::White;

    return state;
}

fn validate_and_play(potential_move: PieceMove, board: &mut BoardState, turn_count: &i32) {
    if validate_move(potential_move, board, turn_count) {
        move_piece(potential_move, board, turn_count);
        board.current_player = match board.current_player {
            BoardColours::White => BoardColours::Black,
            BoardColours::Black => BoardColours::White,
        }
    }
}
fn move_piece(intended_move: PieceMove, board: &mut BoardState, turn_count: &i32) {
    match board.tiles[usize::from(intended_move.end_file)][usize::from(intended_move.end_rank)]
        .piece
    {
        Some(piece) => match piece {
            Pieces::EnPassant(ep_turn_count) => {
                if ep_turn_count + 1 == *turn_count {
                    match board.current_player {
                        BoardColours::Black => {
                            board.tiles[usize::from(intended_move.end_file)]
                                [usize::from(intended_move.end_rank) + 1]
                                .piece = None;
                        }
                        BoardColours::White => {
                            board.tiles[usize::from(intended_move.end_file)]
                                [usize::from(intended_move.end_rank) - 1]
                                .piece = None;
                        }
                    }
                }
                board.tiles[usize::from(intended_move.end_file)]
                    [usize::from(intended_move.end_rank)]
                .piece = None;
            }
            _default => (),
        },
        None => (),
    }

    board.tiles[usize::from(intended_move.end_file)][usize::from(intended_move.end_rank)] =
        board.tiles[usize::from(intended_move.start_file)][usize::from(intended_move.start_rank)];
    board.tiles[usize::from(intended_move.start_file)][usize::from(intended_move.start_rank)] =
        TileState {
            piece: None,
            piece_colour: BoardColours::Black,
        };
}
fn validate_move(potential_move: PieceMove, board: &mut BoardState, turn_count: &i32) -> bool {
    if !match board.tiles[usize::from(potential_move.start_file)]
        [usize::from(potential_move.start_rank)]
    .piece
    {
        Some(piece) => match piece {
            Pieces::EnPassant(_) => false,
            _default => true,
        },
        None => false,
    } {
        println!("This piece isnt allowed to be moved");
        return false; //Checks the piece is "moveable" (not 'None', or 'EnPassant placeholder')
    }
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
        && match board.tiles[usize::from(potential_move.end_file)]
            [usize::from(potential_move.end_rank)]
        .piece
        {
            Some(piece) => match piece {
                Pieces::EnPassant(_) => false,
                _default => {
                    println!("You cannot take your own pieces");
                    true
                }
            },
            None => false,
        }
    {
        return false; //Checks the player isn't taking their own pieces (and that piece exists/is en passant)
    } //This also prevents them from not moving(e.g. moving to where they are)

    if !match board.tiles[usize::from(potential_move.start_file)]
        [usize::from(potential_move.start_rank)]
    .piece
    {
        Some(piece) => match piece {
            Pieces::Pawn => validate_pawn(potential_move, board, turn_count), //Checks that the chosen piece moves appropriately
            Pieces::Rook => validate_rook(potential_move, board),
            Pieces::Knight => validate_knight(potential_move),
            Pieces::Bishop => validate_bishop(potential_move, board),
            Pieces::Queen => validate_queen(potential_move, board),
            Pieces::King => validate_king(potential_move, board),
            Pieces::EnPassant(_) => false,
        },
        None => false,
    } {
        return false;
    }

    return true;
}
fn validate_king(potential_move: PieceMove, board: &mut BoardState) -> bool {
    if (potential_move.start_file.abs_diff(potential_move.end_file) == 1
        || potential_move.start_file.abs_diff(potential_move.end_file) == 0)
        && (potential_move.start_rank.abs_diff(potential_move.end_rank) == 1
            || potential_move.start_rank.abs_diff(potential_move.end_rank) == 0)
    {
        match board.current_player {
            BoardColours::White => {
                board.white_can_castle = false;
            }
            BoardColours::Black => {
                board.black_can_castle = false;
            }
        }
        true
    } else {
        match board.current_player {
            BoardColours::White => if board.white_can_castle {},
            BoardColours::Black => if board.black_can_castle {},
        };
        println!("You cannot move a king in this way");
        false
    }
}
fn validate_queen(potential_move: PieceMove, board: &BoardState) -> bool {
    if !has_jumped_over(potential_move, board, true, true) {
        true
    } else {
        println!("You cannot move a queen in this way");
        false
    }
}
fn validate_knight(potential_move: PieceMove) -> bool {
    if (potential_move.start_file.abs_diff(potential_move.end_file) == 2
        && potential_move.start_rank.abs_diff(potential_move.end_rank) == 1)
        || (potential_move.start_file.abs_diff(potential_move.end_file) == 1
            && potential_move.start_rank.abs_diff(potential_move.end_rank) == 2)
    {
        true
    } else {
        println!("You cannot move a knight in this way");
        false
    }
}
fn validate_bishop(potential_move: PieceMove, board: &BoardState) -> bool {
    if !has_jumped_over(potential_move, board, false, true) {
        true
    } else {
        println!("You cannot move a bishop in this way");
        false
    }
}
fn validate_rook(potential_move: PieceMove, board: &BoardState) -> bool {
    if !has_jumped_over(potential_move, board, true, false) {
        true
    } else {
        println!("You cannot move a rook in this way");
        false
    }
}
fn umin(a: u8, b: u8) -> u8 {
    if a < b {
        a
    } else {
        b
    }
}
fn umax(a: u8, b: u8) -> u8 {
    if a > b {
        a
    } else {
        b
    }
}

///Returns false regardless of whether the piece jumped, if the move isnt horizontal, vertical or 45* diagonal
fn has_jumped_over(
    potential_move: PieceMove,
    board: &BoardState,
    allow_cardinal: bool,
    allow_diagonal: bool,
) -> bool {
    if potential_move.start_file == potential_move.end_file && allow_cardinal {
        // Vertical
        let lower_y = umin(potential_move.start_rank, potential_move.end_rank) + 1; //Allows taking
        let mut current_y = lower_y.clone();
        let higher_y = umax(potential_move.start_rank, potential_move.end_rank) - 1; //Prevents itself from blocking
        while current_y <= higher_y {
            if board
                .get_tile(potential_move.start_file.into(), current_y.into())
                .is_physical()
            {
                return true;
            }
            current_y += current_y;
        }
        return false;
    } else if potential_move.start_rank == potential_move.end_rank && allow_cardinal {
        // Horizontal
        let lower_x = umin(potential_move.start_file, potential_move.end_file) + 1; //Allows taking
        let mut current_x = lower_x.clone();
        let higher_x = umax(potential_move.start_file, potential_move.end_file) - 1; //Prevents itself from blocking
        while current_x <= higher_x {
            if board
                .get_tile(current_x.into(), potential_move.start_rank.into())
                .is_physical()
            {
                return true;
            }
            current_x += current_x;
        }
        return false;
    } else if (i32::from(potential_move.start_file) - i32::from(potential_move.start_rank)
        == i32::from(potential_move.end_file) - i32::from(potential_move.end_rank)
        || i32::from(potential_move.start_file) + i32::from(potential_move.start_rank)
            == i32::from(potential_move.end_file) + i32::from(potential_move.end_rank))
        && allow_diagonal
    {
        // y=x, or y=-x diagonal
        let x_step: i8;
        let y_step: i8;
        let low_x: i16;
        let high_x: i16;
        if potential_move.start_file < potential_move.end_file {
            x_step = 1;
            low_x = potential_move.start_file.into();
            high_x = potential_move.end_file.into();
        } else {
            x_step = -1;
            high_x = potential_move.start_file.into();
            low_x = potential_move.end_file.into();
        }
        if potential_move.start_rank < potential_move.end_rank {
            y_step = 1;
        } else {
            y_step = -1;
        }
        let mut current_x: i8 = i8::try_from(potential_move.start_file).unwrap() + x_step;
        let mut current_y: i8 = i8::try_from(potential_move.start_rank).unwrap() + y_step;

        while low_x < current_x.into() && high_x > current_x.into() {
            if board
                .get_tile(
                    usize::try_from(current_x).expect("Coordinate was negative"),
                    usize::try_from(current_y).expect("Coordinate was negative"),
                )
                .is_physical()
            {
                return true;
            }
            current_x += x_step;
            current_y += y_step;
        }
        false
    } else {
        true
    }
}

fn validate_pawn(potential_move: PieceMove, board: &mut BoardState, turn_count: &i32) -> bool {
    let is_taking = match board.tiles[usize::from(potential_move.end_file)]
        [usize::from(potential_move.end_rank)]
    .piece
    {
        Some(piece) => match piece {
            Pieces::EnPassant(_) => false,
            _default => true,
        },
        None => false,
    };
    let correct_file: bool;
    let mut correct_rank: bool;
    match is_taking {
        true => correct_file = potential_move.start_file.abs_diff(potential_move.end_file) == 1,
        false => correct_file = potential_move.start_file == potential_move.end_file,
    };
    match board.current_player {
        BoardColours::White => {
            correct_rank =
                i16::from(potential_move.start_rank) - i16::from(potential_move.end_rank) == 1;

            if i16::from(potential_move.start_rank) - i16::from(potential_move.end_rank) == 2
                && potential_move.start_rank == 6
                && board.tiles[usize::from(potential_move.start_file)]
                    [usize::from(potential_move.start_rank - 1)]
                .piece
                .is_none()
            {
                correct_rank = true;
                board.tiles[usize::from(potential_move.start_file)]
                    [usize::from(potential_move.start_rank - 1)]
                .piece = Some(Pieces::EnPassant(*turn_count));
            }
        }
        BoardColours::Black => {
            correct_rank =
                i16::from(potential_move.end_rank) - i16::from(potential_move.start_rank) == 1;

            if i16::from(potential_move.end_rank) - i16::from(potential_move.start_rank) == 2
                && potential_move.start_rank == 1
                && board.tiles[usize::from(potential_move.start_file)]
                    [usize::from(potential_move.start_rank + 1)]
                .piece
                .is_none()
            {
                correct_rank = true;
                board.tiles[usize::from(potential_move.start_file)]
                    [usize::from(potential_move.start_rank + 1)]
                .piece = Some(Pieces::EnPassant(*turn_count));
            }
        }
    };
    if correct_file && correct_rank {
        true
    } else {
        if correct_file {
            println!("You have to move forward one or two spaces with a pawn.");
            false
        } else if correct_rank {
            println!("You can only take diagonally");
            false
        } else {
            println!("You have to move forward one or two spaces with a pawn, can only take diagonally, and move forward normally");
            false
        }
    }
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
    white_can_castle: bool,
    black_can_castle: bool,
}
impl BoardState {
    fn get_tile(&self, x: usize, y: usize) -> &TileState {
        &self.tiles[x][y]
    }
}
#[derive(Clone, Copy)]
struct TileState {
    piece: Option<Pieces>,
    piece_colour: BoardColours,
}
impl TileState {
    ///True for any piece except None, or En Passant
    fn is_physical(&self) -> bool {
        match &self.piece {
            Some(piece) => match piece {
                Pieces::Pawn => true,
                Pieces::Rook => true,
                Pieces::Knight => true,
                Pieces::Bishop => true,
                Pieces::Queen => true,
                Pieces::King => true,
                Pieces::EnPassant(_) => false,
            },
            None => false,
        }
    }
    /*fn is_not_physical(&self) -> bool {
        !&self.is_physical()
    }*/
}
#[derive(Clone, Copy, PartialEq)]
enum BoardColours {
    White,
    Black,
}
impl BoardColours {
    fn invert(&self) -> BoardColours {
        match &self {
            BoardColours::White => BoardColours::Black,
            BoardColours::Black => BoardColours::White,
        }
    }
}
#[derive(Clone, Copy)]
enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    EnPassant(i32),
}
fn draw_board(board: &BoardState, as_player: BoardColours) {
    if as_player == BoardColours::White {
        let mut y: i8 = 8;
        let mut x: i8;
        while y >= 0 {
            x = 0;
            while x <= 8 {
                if x == 0 {
                    if y == 0 {
                        print!("  ");
                    } else {
                        print!("{} ", y);
                    }
                } else if y == 0 {
                    print!("{} ", to_letter((x).unsigned_abs()));
                } else {
                    print!(
                        "{}",
                        to_piece_name(
                            &board.tiles
                                [usize::try_from(x.clone() - 1).expect("index out of bounds")]
                                [usize::try_from(y.clone() - 1).expect("index out of bounds")],
                            if (x + y) % 2 == 1 {
                                BoardColours::Black
                            } else {
                                BoardColours::White
                            }
                        )
                    )
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
                            &board.tiles
                                [usize::try_from(x.clone()).expect("index out of bounds")]
                                [usize::try_from(y.clone()).expect("index out of bounds")],
                            if (x + y) % 2 == 1 {
                                BoardColours::Black
                            } else {
                                BoardColours::White
                            }
                        )
                    )
                }
                x -= 1;
            }
            println!();
            y += 1;
        }
    }
    println!();
    match board.current_player {
        BoardColours::White => {
            println!("{}", "   Black's turn   ".white().on_bright_black());
            print!("{}", " Black's move:".white().on_bright_black());
        }
        BoardColours::Black => {
            println!("{}", "   White's turn   ".black().on_white());
            print!("{}", " White's move:".black().on_white());
        }
    }
    std::io::stdout().flush().unwrap();
}

fn to_piece_name(tile: &TileState, colour: BoardColours) -> ColoredString {
    let mut tile_text: String = match tile.piece {
        Some(piece) => match tile.piece_colour {
            BoardColours::Black => match piece {
                Pieces::EnPassant(_) => " ".to_string(), // 'E' for testing, ' ' for release
                _default => "w".to_string(),
            },
            BoardColours::White => match piece {
                Pieces::EnPassant(_) => " ".to_string(), // 'E' for testing, ' ' for release
                _default => "b".to_string(),
            },
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
            Pieces::EnPassant(_) => ' ', // 'n' for testing, ' ' for release
        },
        None => ' ',
    });
    let colored_text: ColoredString;
    return match colour {
        BoardColours::Black => {
            colored_text = tile_text.on_white();
            colored_text.black()
        }
        BoardColours::White => {
            colored_text = tile_text.on_black();
            colored_text.white()
        }
    };
}

fn to_letter(number: u8) -> String {
    let buffer: Vec<u8> = [number + 96].to_vec();
    String::from_utf8(buffer).expect("invalid utf-8 sequence")
}