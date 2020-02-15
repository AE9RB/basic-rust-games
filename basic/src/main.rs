mod core;

use crate::core::ast::*;
use crate::core::lex::*;
use crate::core::parse::*;
use crate::core::token::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", Token::from_string("DATA"));
    println!("{:?}", Lex::new(&"Hello".to_string()).next());
    println!("{:?}", Token::iter());
}
