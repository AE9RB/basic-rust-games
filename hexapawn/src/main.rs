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

#[derive(Copy, Clone, PartialEq, Eq)]
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
    fn black_promoted(&self) -> bool {
        for i in 1..=3 {
            if self[i] == Cell::Black {
                return true;
            }
        }
        false
    }
    fn white_promoted(&self) -> bool {
        for i in 7..=9 {
            if self[i] == Cell::White {
                return true;
            }
        }
        false
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
    print!(BOARD!(), 7, 8, 9, 4, 5, 6, 1, 2, 3);
    println!("It's your pawns against mine.");
    println!("The board is numbered like a calculator.");
    println!("You lose when you can no longer make a move.");
    println!("You lose when I reach the third rank.");
    println!("You go first.");
    println!();

    let mut brain = Brain::new();
    let mut black_wins = 0;
    let mut white_wins = 0;
    loop {
        let mut history: Vec<(Board, Move)> = Vec::new();
        let mut board = Board::new();

        loop {
            println!("{}", board);

            let white_moves = board.white_moves();
            if white_moves.is_empty() || board.black_promoted() {
                println!("Black wins!");
                black_wins += 1;
                break;
            }

            let mov = read_move(&white_moves);
            board.do_move(&mov);
            println!("{}", board);

            let black_moves = board.black_moves();
            if black_moves.is_empty() || board.white_promoted() {
                println!("White wins!");
                white_wins += 1;
                loop {
                    let (b, m) = history.pop().unwrap();
                    let mvs = brain.get(&b);
                    mvs.retain(|&mm| mm != m);
                    if !mvs.is_empty() {
                        break
                    }
                }
                break;
            }

            let mov = brain.get(&board).choose(&mut thread_rng()).unwrap();
            history.push((board,*mov));
            board.do_move(&mov);
            println!("I move: {}", mov);
        }

        println!();
        println!("Black has {} wins. You have {} wins.", black_wins, white_wins);
        if black_wins > white_wins {
            println!("The student has become the master.");
        } else {
            println!("Teach me senpai. Let's go again.");
        }
        println!();
    }
}

fn read_move(moves: &Vec<Move>) -> Move {
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
                    if moves.iter().any(|m| mov == *m) {
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
