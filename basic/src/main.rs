mod core;

use crate::core::lex::*;
use crate::core::token::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", str_to_token("DATA"));
    println!("{:?}", token_to_str(&Token::Statement(Statement::Rem)));
    println!("{:?}", Lex::new(&"Hello".to_string()).next());
    println!("{:?}", Token::iter());
}
