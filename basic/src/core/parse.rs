use super::ast::*;
use super::token::*;

pub fn parse<'a, T: Iterator<Item = &'a Token>>(i: T) -> Ast {
    Parse::parse(i)
}

struct Parse<'a, T: Iterator<Item = &'a Token>> {
    token_stream: std::iter::Peekable<std::iter::Filter<T, fn(&&Token) -> bool>>,
}

impl<'a, T: Iterator<Item = &'a Token>> Parse<'a, T> {
    fn parse(iter: T) -> Ast {
        Parse {
            token_stream: iter
                .filter({
                    |&_| {
                        //todo filter whitespace
                        true
                    }
                } as fn(&&Token) -> bool)
                .peekable(),
        }.start()
    }

    fn next(&mut self) -> Option<&Token> {
        self.token_stream.next()
    }

    fn peek(&mut self) -> Option<&&Token> {
        self.token_stream.peek()
    }

    fn start(&mut self) -> Ast {
        //TODO
        Ast::Statement(Some(10))
    }
}

#[cfg(test)]
mod tests {
    use super::super::lex::*;
    use super::*;

    fn parse_str(s: &str) -> Ast {
        parse(Lex::new(s).collect::<Vec<Token>>().iter())
    }

    #[test]
    fn test_foo() {
        let x = parse_str("for i%=1to30-10");
        assert_eq!(x, Ast::Statement(Some(10)));
    }
}
