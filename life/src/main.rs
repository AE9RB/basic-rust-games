use std::io;
use std::io::Write;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Occupied,
    Death,
    Birth,
}

struct Board {
    cells: Vec<Cell>,
    width: usize,  // X
    height: usize, // Y
}

impl Board{
    fn new(width: usize, height: usize) -> Self {
        Board {
            cells: vec![Cell::Empty; width * height],
            width,
            height,
        }
    }
    fn evolve(&mut self) {
        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                let mut neighbors = 0;
                for yy in y-1..y+2 {
                    for xx in x-1..x+2 {
                        if x != xx || y != yy {
                            let cell = &self[(xx, yy)];
                            if *cell == Cell::Occupied || *cell == Cell::Death {
                                neighbors += 1;
                            }
                        }
                    }
                }
                let cell = &mut self[(x, y)];
                if neighbors == 3 && *cell == Cell::Empty {
                    *cell = Cell::Birth;
                }
                if (neighbors < 2 || neighbors > 3) && *cell == Cell::Occupied {
                    *cell = Cell::Death;
                }
            }
        }
        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                {
                    let cell = &mut self[(x, y)];
                    if *cell == Cell::Death {
                        *cell = Cell::Empty;
                    }
                    if *cell == Cell::Birth {
                        *cell = Cell::Occupied;
                    }
                }
            }
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;
    fn index(&self, index_xy: (usize, usize)) -> &Self::Output {
        let (x, y) = index_xy;
        &self.cells[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index_xy: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index_xy;
        &mut self.cells[x + y * self.width]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(self.height * self.width + self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self[(x, y)];
                if *cell == Cell::Occupied {
                    s += "*";
                } else {
                    s += " ";
                }
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

fn main() {
    println!();
    println!(" *************");
    println!(" *  L I F E  *");
    println!(" *************");
    println!();

    let mut b = example1();
    loop {
        println!("\n{}", b);
        print!("Enter for more: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        b.evolve();
    }
}

//    *
//   ***
//  ** **
fn example1() -> Board {
    let mut b = Board::new(39, 12);
    b[(19, 5)] = Cell::Occupied;
    b[(18, 6)] = Cell::Occupied;
    b[(19, 6)] = Cell::Occupied;
    b[(20, 6)] = Cell::Occupied;
    b[(17, 7)] = Cell::Occupied;
    b[(18, 7)] = Cell::Occupied;
    b[(20, 7)] = Cell::Occupied;
    b[(21, 7)] = Cell::Occupied;
    b
}
