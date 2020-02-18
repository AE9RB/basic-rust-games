use super::ast::*;
use super::error::Error;
use super::token::*;

type Result = std::result::Result<Ast, Error>;

macro_rules! error {
    // e.g. error!(self, NextWithoutFor)
    ($self:ident, $err:ident) => {
        Err(Error {
            err: $crate::core::error::Basic::$err as u16,
            erl: $self.line,
        })
    };
}

pub fn parse<'a, T: Iterator<Item = &'a Token>>(iter: T) -> Result {
    Parse {
        line: 65535,
        token_stream: iter
            .filter({
                |&_| {
                    //todo filter whitespace
                    true
                }
            } as fn(&&Token) -> bool)
            .peekable(),
    }
    .start()
}

type Line = u16;

struct Parse<'a, T: Iterator<Item = &'a Token>> {
    token_stream: std::iter::Peekable<T>,
    line: Line,
}

impl<'a, T: Iterator<Item = &'a Token>> Parse<'a, T> {
    fn next(&mut self) -> Option<&Token> {
        self.token_stream.next()
    }

    fn peek(&mut self) -> Option<&&Token> {
        self.token_stream.peek()
    }

    fn expect(&mut self, token: &Token) -> Result {
        match self.next() {
            Some(t) => {
                println!("{}", t);
                Ok(Ast {
                    line: None,
                    root: Statement::Data(vec![]),
                })
            }
            None => error!(self, SyntaxError),
        }
    }

    fn start(&mut self) -> Result {
        self.expect(&Token::Comma)?;
        //self.start()

        Ok(Ast {
            line: None,
            root: Statement::Data(vec![]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::lex::*;
    use super::*;

    fn parse_str(s: &str) -> Result {
        parse(Lex::new(s).collect::<Vec<Token>>().iter())
    }

    #[test]
    fn test_foo1() {
        let x = parse_str("for i%=1to30-10").unwrap();
        assert_eq!(
            x,
            Ast {
                line: None,
                root: Statement::Data(vec![])
            }
        );
    }

    #[test]
    fn test_foo2() {
        let x = parse_str("for i%=1to30-10").unwrap();
        assert_eq!(
            x,
            Ast {
                line: None,
                root: Statement::Data(vec![])
            }
        );
    }
}
