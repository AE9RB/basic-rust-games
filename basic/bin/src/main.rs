extern crate core;

use crate::core::lex::*;
use crate::core::parse::*;
use crate::core::token::*;

fn main() {
    let t = parse(Lex::new("10 PRINT 10").collect::<Vec<Token>>().iter());
    println!("{:?}", t);
}
