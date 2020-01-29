use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::io::Write;

extern crate rand;
use rand::Rng;

struct Card {
    rank: u8,
    revealed: bool,
}

impl Card {
    fn new(revealed: bool) -> Self {
        Card {
            rank: rand::thread_rng().gen_range(2, 15),
            revealed,
        }
    }
    fn turn_over(&mut self) {
        self.revealed = !self.revealed;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.revealed {
            match self.rank {
                11 => write!(f, "[Jack]"),
                12 => write!(f, "[Queen]"),
                13 => write!(f, "[King]"),
                14 => write!(f, "[Ace]"),
                _ => write!(f, "[{}]", self.rank),
            }
        } else {
            write!(f, "[???]")
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}

struct Hand {
    cards: [Card; 3],
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: [Card::new(true), Card::new(true), Card::new(false)],
        }
    }
    fn is_winner(&mut self) -> bool {
        self.cards[2].turn_over();
        if self.cards[0] < self.cards[1] {
            self.cards[2] > self.cards[0] && self.cards[2] < self.cards[1]
        } else {
            self.cards[2] < self.cards[0] && self.cards[2] > self.cards[1]
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.cards[0], self.cards[1], self.cards[2])
    }
}

fn main() {
    println!();
    println!(" ***************************");
    println!(" *  A C E Y   D E U C E Y  *");
    println!(" ***************************");
    println!();
    println!("Two cards are dealt face up and a third face down.");
    println!("You win if the rank of the third card is between the first two.");

    game();
}

fn game() {
    let mut wallet: u32 = 100;
    loop {
        let mut hand = Hand::new();
        println!();
        println!("You have {} coins.", wallet);
        println!("The deal: {}", hand);

        loop {
            let bet = bet();
            if bet > wallet {
                println!("You don't have that much.");
                continue;
            }
            if bet == 0 {
                println!("Chicken.");
                break;
            }
            if hand.is_winner() {
                println!("You win!: {}", hand);
                wallet += bet;
            } else {
                println!("You lose: {}", hand);
                wallet -= bet;
            }
            break;
        }

        if wallet <= 0 {
            println!("You are out of coins.");
            println!("Goodbye.");
            println!();
            break;
        }
    }
}

fn bet() -> u32 {
    loop {
        print!("Your Bet? ");
        io::stdout().flush().unwrap();
        let mut bet = String::new();
        io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");
        let bet = bet.trim().parse::<u32>();
        match bet {
            Ok(bet) => return bet,
            Err(why) => println!("{:?}", why),
        }
    }
}
