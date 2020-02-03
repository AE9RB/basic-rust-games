use super::token::*;
use core::iter::Peekable;
use std::str::Chars;

pub struct Lex<'a> {
    i: Peekable<Chars<'a>>,
}

impl<'a> Lex<'a> {
    pub fn new(s: &'a String) -> Lex {
        Lex {
            i: s.chars().peekable(),
        }
    }

    fn ascii_whitespace(&mut self) -> Option<Token> {
        let mut spaces = 0;
        loop {
            self.i.next();
            spaces += 1;
            if let Some(p) = self.i.peek() {
                if p.is_ascii_whitespace() {
                    continue;
                }
            }
            return Some(Token::Whitespace(spaces));
        }
    }

    fn number(&mut self) -> Option<Token> {
        let mut s = String::new();
        let mut decimal = false;
        let mut exp = false;
        loop {
            let mut c = self.i.next().unwrap();
            if c == 'e' {
                c = 'E'
            }
            s.push(c);
            if let Some(p) = self.i.peek() {
                if c == '.' {
                    decimal = true
                }
                if c == 'E' {
                    exp = true;
                    if *p == '+' || *p == '-' {
                        continue;
                    }
                }
                if p.is_ascii_digit() {
                    continue;
                }
                if *p == '.' && decimal == false {
                    continue;
                }
                if *p == 'e' || *p == 'E' && exp == false {
                    continue;
                }
            }
            return Some(Token::Number(s));
        }
    }

    fn string(&mut self) -> Option<Token> {
        let mut s = String::new();
        self.i.next();
        loop {
            if let Some(c) = self.i.next() {
                if c != '"' {
                    s.push(c);
                    continue;
                }
            }
            return Some(Token::String(s));
        }
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.i.peek();
        if p == None {
            panic!("todo");
        }
        let p = p.unwrap();
        if p.is_ascii_whitespace() {
            return self.ascii_whitespace();
        }
        if p.is_ascii_digit() || *p == '.' {
            return self.number();
        }
        if *p == '"' {
            return self.string();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        println!("{:?}", "876245e+12".to_string().parse::<f32>());
        let s = String::from("10  2.9e-3\"abc");
        let mut x = Lex::new(&s);
        assert_eq!(x.next().unwrap(), Token::Number("10".to_string()));
        assert_eq!(x.next().unwrap(), Token::Whitespace(2));
        assert_eq!(x.next().unwrap(), Token::Number("2.9E-3".to_string()));
        assert_eq!(x.next().unwrap(), Token::String("abc".to_string()));
    }
}
