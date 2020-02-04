use super::token::*;
use core::iter::Peekable;

pub struct Lex<'a> {
    i: Peekable<std::iter::Take<std::str::Chars<'a>>>,
    remark: bool,
}

impl<'a> Lex<'a> {
    pub fn new(s: &'a String) -> Lex {
        let mut t = s.len();
        if s.ends_with("\r\n") {
            t -= 2
        } else if s.ends_with("\r") {
            t -= 1
        } else if s.ends_with("\n") {
            t -= 1
        }
        Lex {
            i: s.chars().take(t).peekable(),
            remark: false,
        }
    }

    fn is_basic_whitespace(c: char) -> bool {
        c == ' ' || c == '\t'
    }

    fn is_basic_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_basic_alphabetic(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn whitespace(&mut self) -> Option<Token> {
        let mut spaces = 0;
        loop {
            self.i.next();
            spaces += 1;
            if let Some(p) = self.i.peek() {
                if Self::is_basic_whitespace(*p) {
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
                if Self::is_basic_digit(*p) {
                    continue;
                }
                if *p == '.' && decimal == false {
                    continue;
                }
                if *p == 'e' || *p == 'E' && exp == false {
                    continue;
                }
            }
            if exp || decimal {
                return Some(Token::Float(s));
            }
            return Some(Token::Integer(s));
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

    fn alphabetic(&mut self) -> Option<Token> {
        let mut s = String::new();
        let mut digit = false;
        loop {
            let c = self.i.next().unwrap().to_ascii_uppercase();
            s.push(c);
            if Self::is_basic_digit(c) {
                digit = true;
            }
            if let Some(t) = str_to_token(&s) {
                return Some(t);
            }
            if c == '$' {
                return Some(Token::StringIdent(s));
            }
            if c == '%' {
                return Some(Token::IntegerIdent(s));
            }
            if let Some(p) = self.i.peek() {
                if Self::is_basic_alphabetic(*p) {
                    if digit {
                        break;
                    }
                    continue;
                }
                if Self::is_basic_digit(*p) || *p == '$' || *p == '%' {
                    continue;
                }
            }
            break;
        }
        return Some(Token::Ident(s));
    }

    fn minutia(&mut self) -> Option<Token> {
        let mut s = String::new();
        loop {
            if let Some(c) = self.i.next() {
                s.push(c);
                if let Some(t) = str_to_token(&s) {
                    return Some(t);
                }
                if let Some(p) = self.i.peek() {
                    if Self::is_basic_alphabetic(*p) {
                        break;
                    }
                    if Self::is_basic_digit(*p) {
                        break;
                    }
                    if Self::is_basic_whitespace(*p) {
                        break;
                    }
                    continue;
                }
                break;
            }
        }
        return Some(Token::Unknown(s));
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.i.peek()?;
        if self.remark {
            let mut s = String::new();
            while let Some(c) = self.i.next() {
                s.push(c);
            }
            return Some(Token::String(s));
        }
        if Self::is_basic_whitespace(*p) {
            return self.whitespace();
        }
        if Self::is_basic_digit(*p) || *p == '.' {
            return self.number();
        }
        if *p == '"' {
            return self.string();
        }
        if Self::is_basic_alphabetic(*p) {
            let r = self.alphabetic();
            if r == Some(Token::Statement(Statement::Rem)) {
                self.remark = true;
            }
            return r;
        }
        return self.minutia();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remark() {
        let s = String::from("100 REM A fortunate comment");
        let mut x = Lex::new(&s);
        assert_eq!(x.next().unwrap(), Token::Integer("100".to_string()));
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Statement(Statement::Rem));
        assert_eq!(
            x.next().unwrap(),
            Token::String(" A fortunate comment".to_string())
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_for_loop() {
        let s = String::from("for i%=1to30-10");
        let mut x = Lex::new(&s);
        assert_eq!(x.next().unwrap(), Token::Statement(Statement::For));
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::IntegerIdent("I%".to_string()));
        assert_eq!(x.next().unwrap(), Token::Operator(Operator::Equals));
        assert_eq!(x.next().unwrap(), Token::Integer("1".to_string()));
        assert_eq!(x.next().unwrap(), Token::Statement(Statement::To));
        assert_eq!(x.next().unwrap(), Token::Integer("30".to_string()));
        assert_eq!(x.next().unwrap(), Token::Operator(Operator::Minus));
        assert_eq!(x.next().unwrap(), Token::Integer("10".to_string()));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_unknown() {
        let s = String::from("10 fOr %woo in 0..4\n");
        let mut x = Lex::new(&s);
        assert_eq!(x.next().unwrap(), Token::Integer("10".to_string()));
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Statement(Statement::For));
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Unknown("%".to_string()));
        assert_eq!(x.next().unwrap(), Token::Ident("WOO".to_string()));
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Ident("IN".to_string()));
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Float("0.".to_string()));
        assert_eq!(x.next().unwrap(), Token::Float(".4".to_string()));
        assert_eq!(x.next(), None);
    }
}
