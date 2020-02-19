mod lang;
use lang::line::*;

fn main() {
    let mut t = Line::from_str(" 10 PRINT 10 \r\n");
    t.ast();
    println!("{:?}", t);
    println!("{}", t);
}
