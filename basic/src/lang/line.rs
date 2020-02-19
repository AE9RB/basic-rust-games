use std::fmt;

use super::ast::*;
use super::error::*;
use super::lex::*;
use super::parse::*;
use super::token::*;

#[derive(Debug, PartialEq)]
pub struct Line {
    number: u16,
    tokens: Vec<Token>,
    ast: Option<Result<Vec<Statement>, Error>>,
}

impl Line {
    pub fn from_str(s: &str) -> Line {
        let lex = &mut Lex::new(s);
        Line {
            tokens: lex.collect(),
            number: lex.line_number(),
            ast: None,
        }
    }

    pub fn is_direct(&self) -> bool {
        self.number == 65535
    }

    pub fn ast(&mut self) -> &Result<Vec<Statement>, Error> {
        if self.ast.is_some() {
            self.ast.as_ref().unwrap()
        } else {
            self.ast = Some(parse(self.tokens.iter()));
            self.ast.as_ref().unwrap()
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.tokens.iter().map(|s| s.to_string()).collect();
        if self.is_direct() {
            write!(f, "{}", s)
        } else {
            write!(f, "{}{}", self.number, s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let _ = Line::from_str("100 fancy");
    }
}
