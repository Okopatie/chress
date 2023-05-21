use colored::*;
use std::fmt;
#[derive(Copy, Clone)]
struct Piece {
    ptype: Pieces,
    side: Side,
}
#[derive(Copy, Clone)]
enum Pieces {
    Pawn,
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
}
#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    White,
    Black,
}
pub struct GameState {
    board: [Option<Piece>; 64],
}

impl Piece {
    fn new(ptype: Pieces, side: Side) -> Self {
        Self { ptype, side }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.ptype {
            Pieces::Pawn => "P",
            Pieces::Bishop => "B",
            Pieces::King => "K",
            Pieces::Knight => "N",
            Pieces::Queen => "Q",
            Pieces::Rook => "R",
        };
        let output = match self.side {
            Side::Black => symbol.blue(),
            Side::White => symbol.green(),
        };
        write!(f, "{output}")
    }
}
// Board setup
impl GameState {
    pub fn new() -> Self {
        Self { board: [None; 64] }
    }
    pub fn setup_board(&mut self) {
        for i in 1..9 {
            self.set_square(i, 2, Some(Piece::new(Pieces::Pawn, Side::White)));
            self.set_square(i, 7, Some(Piece::new(Pieces::Pawn, Side::Black)));
        }
        for i in 0..2 {
            let piece_rank = 7 * i + 1;
            let piece_colour = if i == 0 { Side::White } else { Side::Black };
            self.set_square(1, piece_rank, Some(Piece::new(Pieces::Rook, piece_colour)));
            self.set_square(1, piece_rank, Some(Piece::new(Pieces::Rook, piece_colour)));
            self.set_square(
                2,
                piece_rank,
                Some(Piece::new(Pieces::Knight, piece_colour)),
            );
            self.set_square(
                2,
                piece_rank,
                Some(Piece::new(Pieces::Knight, piece_colour)),
            );
            self.set_square(
                3,
                piece_rank,
                Some(Piece::new(Pieces::Bishop, piece_colour)),
            );
            self.set_square(
                6,
                piece_rank,
                Some(Piece::new(Pieces::Bishop, piece_colour)),
            );
            self.set_square(4, piece_rank, Some(Piece::new(Pieces::Queen, piece_colour)));
            self.set_square(5, piece_rank, Some(Piece::new(Pieces::King, piece_colour)));
        }
    }
    pub fn print_board(&self) {
        for i in 1..9 {
            for j in 1..9 {
                let square = self.get_square(j, 9 - i);
                let to_print = if let Some(piece) = &square {
                    piece.to_string()
                } else {
                    String::from(" ")
                };
                let to_print = if i % 2 == j % 2 {
                    to_print.on_white()
                } else {
                    to_print.on_black()
                };
                print!("{to_print}");
            }
            println!("\n")
        }
    }
    fn get_square(&self, file: usize, rank: usize) -> &Option<Piece> {
        let index: usize = ((rank - 1) * 8) + file - 1;
        &self.board[index]
    }
    fn set_square(&mut self, file: usize, rank: usize, piece: Option<Piece>) {
        let index: usize = ((rank - 1) * 8) + file - 1;
        self.board[index] = piece;
    }
}

// Piece movements
impl GameState {
    // TODO: implement en passant
    pub fn pawn_move(&self, file: usize, rank: usize, side: Side) -> Vec<(usize, usize)> {
        let mut potential_moves: Vec<(usize, usize)> = Vec::new();
        let starting_rank = match side {
            Side::White => 2,
            Side::Black => 7,
        };
        let direction: isize = match side {
            Side::White => 1,
            Side::Black => -1,
        };
        let on_starting_rank = rank == starting_rank;
        if self
            .get_square(file, (rank as isize + direction) as usize)
            .is_none()
        {
            potential_moves.push((file, (rank as isize + direction) as usize));
            if on_starting_rank {
                if self
                    .get_square(file, (rank as isize + direction * 2) as usize)
                    .is_none()
                {
                    potential_moves.push((file, (rank as isize + direction * 2) as usize));
                }
            }
        }
        if let Some(piece) = self.get_square(file + 1, (rank as isize + direction) as usize) {
            if file != 8 && piece.side != side {
                potential_moves.push((file + 1, (rank as isize + direction) as usize))
            }
        }
        if let Some(piece) = self.get_square(file - 1, (rank as isize + direction) as usize) {
            if file != 1 && piece.side != side {
                potential_moves.push((file + 1, (rank as isize + direction) as usize))
            }
        }

        potential_moves
    }
    pub fn knight_move(&self, file: usize, rank: usize, side: Side) -> Vec<(usize, usize)> {
        let potential_moves: Vec<(usize, usize)> = Vec::new();
        
        potential_moves
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
