use std::fmt;
use std::io;
use std::io::Write;

extern crate rand;
use rand::prelude::*;

#[derive(PartialOrd, PartialEq)]
struct Card {
    rank: u8,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.rank {
            11 => write!(f, "[Jack]"),
            12 => write!(f, "[Queen]"),
            13 => write!(f, "[King]"),
            14 => write!(f, "[Ace]"),
            _ => write!(f, "[{}]", self.rank),
        }
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        Deck { cards: vec![] }
    }
    fn reset(&mut self) {
        self.cards.truncate(0);
        for r in 2..=15 {
            for _s in 1..=4 {
                self.cards.push(Card { rank: r });
            }
        }
        self.cards.shuffle(&mut rand::thread_rng());
    }
    fn hand(&mut self) -> [Card; 3] {
        if self.cards.len() < 10 {
            self.reset();
        }
        [
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
            self.cards.pop().unwrap(),
        ]
    }
}

struct Hand {
    cards: [Card; 3],
    revealed: bool,
}

impl Hand {
    fn new(deck: &mut Deck) -> Self {
        Hand {
            cards: deck.hand(),
            revealed: false,
        }
    }
    fn is_winner(&mut self) -> bool {
        self.revealed = true;
        if self.cards[0] < self.cards[1] {
            self.cards[2] > self.cards[0] && self.cards[2] < self.cards[1]
        } else {
            self.cards[2] < self.cards[0] && self.cards[2] > self.cards[1]
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.revealed {
            write!(f, "{} {} {}", self.cards[0], self.cards[1], self.cards[2])
        } else {
            write!(f, "{} {} [???]", self.cards[0], self.cards[1])
        }
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
    let mut deck = Deck::new();
    loop {
        let mut hand = Hand::new(&mut deck);
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
