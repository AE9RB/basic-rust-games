mod token;
use token::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", str_to_token("DATA"));
    println!("{:?}", token_to_str(&Token::Statement(Statement::Rem)));
}
