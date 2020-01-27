use std::io;
use std::io::Write;
use std::fmt;

extern crate rand;
use rand::Rng;

#[derive(PartialEq)]
struct Location {
    x:f32,
    y:f32,
}

impl Location {
    fn rand() -> Self{
        Location{
            x: rand::thread_rng().gen_range(0_f32, 10_f32).floor(),
            y: rand::thread_rng().gen_range(0_f32, 10_f32).floor(),
        }
    }
    fn from(x:f32, y:f32) -> Self{
        Location{x,y}
    }
    fn dist(&self, other:&Self) -> f32 {

        ( (self.x-other.x).powi(2) + (self.y-other.y).powi(2) ).sqrt()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn main() {
    println!();
    println!(" *******************");
    println!(" *  M U G W U M P  *");
    println!(" *******************");
    println!();
    println!("The object of this game if to find four Mugwumps");
    println!("hidden on a 10 by 10 grid.  Homebase is position 0,0");
    println!("Any guess you make must be two numbers with each");
    println!("number between 0 and 9, inclusive.  First number");
    println!("is distance to right of homebase and second number");
    println!("is distance above homebase.");
    println!();
    println!("You get 10 tries.  After each try, I will tell");
    println!("you how far you are from each Mugwump.");
    println!();

    let x1 = Location::rand();
    let x2 = guess(1);
    println!("{} | {} | {}", x1, x2, x1.dist(&x2));
}

fn guess(turn: u32) -> Location  {
    loop {
        print!("Turn no. {} What is your guess? ", turn);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec: Vec<&str> = input.split(",").collect();
        if vec.len() == 2 {
            if let Ok(x) = vec[0].trim().parse::<f32>() {
                if let Ok(y) = vec[1].trim().parse::<f32>() {
                    return Location::from(x,y)
                }
            }
        }
        println!("Expected digit-comma-digit e.g. 0,9");
    }
}
