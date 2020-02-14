mod core;

use crate::core::token::*;
use crate::core::lex::*;
use crate::core::ast::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", Token::from_string("DATA"));
    println!("{:?}", Lex::new(&"Hello".to_string()).next());
    println!("{:?}", Token::iter());
}
