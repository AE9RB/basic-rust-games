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
        let mut digits = 0;
        let mut decimal = false;
        let mut exp = false;
        loop {
            let mut c = self.i.next().unwrap();
            if c == 'e' {
                c = 'E'
            }
            if c == 'd' {
                c = 'D'
            }
            s.push(c);
            if !exp && Self::is_basic_digit(c) {
                digits += 1;
            }
            if c == '.' {
                decimal = true
            }
            if c == 'D' {
                digits += 8;
            }
            if c == '!' {
                return Some(Token::Single(s));
            }
            if c == '#' {
                return Some(Token::Double(s));
            }
            if let Some(p) = self.i.peek() {
                if c == 'E' || c == 'D' {
                    exp = true;
                    if *p == '+' || *p == '-' {
                        continue;
                    }
                }
                if Self::is_basic_digit(*p) {
                    continue;
                }
                if !decimal && *p == '.' {
                    continue;
                }
                if !exp && *p == 'E' || *p == 'e' || *p == 'D' || *p == 'd' {
                    continue;
                }
                if *p == '!' || *p == '#' {
                    continue;
                }
            }
            if digits > 7 {
                return Some(Token::Double(s));
            }
            if !exp && !decimal {
                if let Ok(_) = s.parse::<i16>() {
                    return Some(Token::Integer(s));
                }
            }
            return Some(Token::Single(s));
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
            if c == '!' {
                return Some(Token::SingleIdent(s));
            }
            if c == '#' {
                return Some(Token::DoubleIdent(s));
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
    fn test_numbers() {
        let s = "3.141593".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Single("3.141593".to_string())
        );
        let s = "3.1415926".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Double("3.1415926".to_string())
        );
        let s = "32767".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Integer("32767".to_string())
        );
        let s = "32768".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Single("32768".to_string())
        );
        let s = "24e9".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Single("24E9".to_string())
        );
    }

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
        assert_eq!(x.next().unwrap(), Token::Single("0.".to_string()));
        assert_eq!(x.next().unwrap(), Token::Single(".4".to_string()));
        assert_eq!(x.next(), None);
    }
}
