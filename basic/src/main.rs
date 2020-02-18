mod parsing;
use parsing::lex::*;
use parsing::parse::*;
use parsing::token::*;

fn main() {
    let t = parse(Lex::new("10 PRINT 10").collect::<Vec<Token>>().iter());
    println!("{:?}", t);
}
