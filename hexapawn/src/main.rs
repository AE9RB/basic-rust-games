use std::fmt;
use std::ops::{Index, IndexMut};

const MOTION: [[u8; 2]; 14] = [
    [1, 4],
    [1, 5],
    [2, 4],
    [2, 5],
    [2, 6],
    [3, 5],
    [3, 6],
    [4, 7],
    [4, 8],
    [5, 7],
    [5, 8],
    [5, 9],
    [6, 8],
    [6, 9],
];

#[derive(Debug)]
struct Move {
    from: u8,
    to: u8,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.from, self.to)
    }
}

impl Move {
    fn is_forward(&self) -> bool {
        if self.from > self.to {
            self.from - self.to == 3
        } else {
            self.to - self.from == 3
        }
    }
    fn all_white_moves() -> Vec<Move> {
        MOTION
            .iter()
            .map(|m| Move {
                from: m[0],
                to: m[1],
            })
            .collect()
    }
    fn all_black_moves() -> Vec<Move> {
        MOTION
            .iter()
            .map(|m| Move {
                from: 10 - m[0],
                to: 10 - m[1],
            })
            .collect()
    }
}

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    White,
    Black,
}

struct Board {
    cells: [Cell; 9],
}

impl Index<u8> for Board {
    type Output = Cell;
    fn index(&self, index: u8) -> &Self::Output {
        &self.cells[(index-1) as usize]
    }
}

impl IndexMut<u8> for Board {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.cells[(index-1) as usize]
    }
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [
                Cell::White,
                Cell::White,
                Cell::White,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Black,
                Cell::Black,
                Cell::Black,
            ],
        }
    }
    fn white_moves(&self) -> Vec<Move> {
        let mut moves = Move::all_white_moves();
        moves.retain(|m| {
            if self[m.from] != Cell::White {
                return false;
            }
            if m.is_forward() {
                if self[m.to] == Cell::Black {
                    return false;
                }
            } else { // is_capture
                if self[m.to] != Cell::Black {
                    return false;
                }
            }
            true
        });
        moves
    }
    fn black_moves(&self) -> Vec<Move> {
        let mut moves = Move::all_black_moves();
        moves.retain(|m| {
            if self[m.from] != Cell::Black {
                return false;
            }
            if m.is_forward() {
                if self[m.to] == Cell::White {
                    return false;
                }
            } else { // is_capture
                if self[m.to] != Cell::White {
                    return false;
                }
            }
            true
        });
        moves
    }
}

fn main() {
    println!();
    println!(" *********************");
    println!(" *  H E X A P A W N  *");
    println!(" *********************");
    println!();

    let board = Board::new();
    println!("{:?}", board.white_moves());
    println!("{:?}", board.black_moves());
}
