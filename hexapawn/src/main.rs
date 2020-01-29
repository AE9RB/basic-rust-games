use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Write;
use std::ops::{Index, IndexMut};

extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

macro_rules! BOARD {
    () => {
        "+---+---+---+\n\
         | {} | {} | {} |\n\
         +---+---+---+\n\
         | {} | {} | {} |\n\
         +---+---+---+\n\
         | {} | {} | {} |\n\
         +---+---+---+\n"
    };
}

#[derive(PartialEq, Eq)]
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

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    White,
    Black,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::White => write!(f, "O"),
            Cell::Black => write!(f, "X"),
        }
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
struct Board {
    cells: [Cell; 9],
}

impl Index<u8> for Board {
    type Output = Cell;
    fn index(&self, index: u8) -> &Self::Output {
        &self.cells[(index - 1) as usize]
    }
}

impl IndexMut<u8> for Board {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.cells[(index - 1) as usize]
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
            } else {
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
            } else {
                if self[m.to] != Cell::White {
                    return false;
                }
            }
            true
        });
        moves
    }
    fn do_move(&mut self, mov: &Move) {
        self[mov.to] = self[mov.from];
        self[mov.from] = Cell::Empty;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            BOARD!(),
            self[7], self[8], self[9], self[4], self[5], self[6], self[1], self[2], self[3]
        )
    }
}

struct Brain {
    brain: HashMap<Board, Vec<Move>>,
}

impl Brain {
    fn new() -> Self {
        Brain {
            brain: HashMap::new(),
        }
    }
    fn get(&mut self, board: &Board) -> &mut Vec<Move> {
        match self.brain.entry(*board) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(board.black_moves()),
        }
    }
}

fn main() {
    println!();
    println!(" *********************");
    println!(" *  H E X A P A W N  *");
    println!(" *********************");
    println!();
    println!(BOARD!(), 7, 8, 9, 4, 5, 6, 1, 2, 3);

    let mut brain = Brain::new();
    let mut board = Board::new();
    println!("{}", board);

    let mov = read_move(&board);
    board.do_move(&mov);
    println!("{}", board);

    let mov = brain.get(&board).choose(&mut thread_rng()).unwrap();
    board.do_move(&mov);
    println!("I move: {}\n{}", mov, board);
}

fn read_move(board: &Board) -> Move {
    println!();
    let white_moves = board.white_moves();
    loop {
        print!("Your move? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let vec: Vec<&str> = input.split(",").collect();
        if vec.len() == 2 {
            if let Ok(f) = vec[0].trim().parse::<u8>() {
                if let Ok(t) = vec[1].trim().parse::<u8>() {
                    let mov = Move { from: f, to: t };
                    if white_moves.iter().any(|m| mov == *m) {
                        return mov;
                    }
                    println!("Invalid move");
                    continue;
                }
            }
        }
        println!("Expected digit-comma-digit e.g. 0,9");
    }
}
