use super::token::*;
use std::iter::Peekable;

pub struct Lex<'a> {
    i: Peekable<std::iter::Take<std::str::Chars<'a>>>,
    remark: bool,
    starting: bool,
    direct: bool,
    next_token: Option<Token>,
}

impl<'a> Iterator for Lex<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_token.is_some() {
            return self.next_token.take();
        }
        let p = self.i.peek()?;
        if self.remark {
            return Some(Token::Unknown(self.i.by_ref().collect::<String>()));
        }

        if Self::is_basic_whitespace(*p) {
            let tw = self.whitespace();
            if self.starting {
                let tn = self.next();
                if !self.direct {
                    return tn;
                }
                self.next_token = tn;
            }
            return tw;
        }
        if Self::is_basic_digit(*p) || *p == '.' {
            let tn = self.number();
            if self.starting {
                self.starting = false;
                if let Some(Token::Literal(Literal::Integer(_))) = tn {
                    self.direct = false;
                }
            }
            return tn;
        }
        self.starting = false;

        if Self::is_basic_alphabetic(*p) {
            let r = self.alphabetic();
            if r == Some(Token::Word(Word::Rem)) {
                self.remark = true;
            }
            if let Some(p) = self.i.peek() {
                if Self::is_basic_alphabetic(*p) {
                    if !self.direct {
                        self.next_token = Some(Token::Whitespace(1));
                    }
                }
            }
            return r;
        }
        if *p == '"' {
            return self.string();
        }
        let r = self.minutia();
        if r == Some(Token::Word(Word::Rem2)) {
            self.remark = true;
        }
        return r;
    }
}

impl<'a> Lex<'a> {
    pub fn new(s: &'a str) -> Lex {
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
            starting: true,
            direct: true,
            next_token: None,
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
                return Some(Token::Literal(Literal::Single(s)));
            }
            if c == '#' {
                return Some(Token::Literal(Literal::Double(s)));
            }
            if c == '%' {
                return Some(Token::Literal(Literal::Integer(s)));
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
                return Some(Token::Literal(Literal::Double(s)));
            }
            if !exp && !decimal {
                if let Ok(_) = s.parse::<i16>() {
                    return Some(Token::Literal(Literal::Integer(s)));
                }
            }
            return Some(Token::Literal(Literal::Single(s)));
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
            return Some(Token::Literal(Literal::String(s)));
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
            if let Some(t) = Token::from_string(&s) {
                return Some(t);
            }
            if c == '$' {
                return Some(Token::Ident(Ident::String(s)));
            }
            if c == '!' {
                return Some(Token::Ident(Ident::Single(s)));
            }
            if c == '#' {
                return Some(Token::Ident(Ident::Double(s)));
            }
            if c == '%' {
                return Some(Token::Ident(Ident::Integer(s)));
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
        return Some(Token::Ident(Ident::Plain(s)));
    }

    fn minutia(&mut self) -> Option<Token> {
        let mut s = String::new();
        loop {
            if let Some(c) = self.i.next() {
                s.push(c);
                if let Some(t) = Token::from_string(&s) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        let s = "3.141593".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Literal(Literal::Single("3.141593".to_string()))
        );
        let s = "3.1415926".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Literal(Literal::Double("3.1415926".to_string()))
        );
        let s = "32767".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Literal(Literal::Integer("32767".to_string()))
        );
        let s = "32768".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Literal(Literal::Single("32768".to_string()))
        );
        let s = "24e9".to_string();
        assert_eq!(
            Lex::new(&s).next().unwrap(),
            Token::Literal(Literal::Single("24E9".to_string()))
        );
    }

    #[test]
    fn test_remark() {
        let mut x = Lex::new(" 100 REM A fortunate comment");
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Integer("100".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Word(Word::Rem));
        assert_eq!(
            x.next().unwrap(),
            Token::Unknown(" A fortunate comment".to_string())
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_remark2() {
        let mut x = Lex::new("100 'The comment");
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Integer("100".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Word(Word::Rem2));
        assert_eq!(x.next().unwrap(), Token::Unknown("The comment".to_string()));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_scanner() {
        let mut x = Lex::new("BANDS");
        assert_eq!(
            x.next().unwrap(),
            Token::Ident(Ident::Plain("BANDS".to_string()))
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_for_loop() {
        let mut x = Lex::new(" forI%=1to30-10");
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Word(Word::For));
        assert_eq!(
            x.next().unwrap(),
            Token::Ident(Ident::Integer("I%".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Operator(Operator::Equals));
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Integer("1".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Word(Word::To));
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Integer("30".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Operator(Operator::Minus));
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Integer("10".to_string()))
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_unknown() {
        let mut x = Lex::new("10 fOr %woo in 0..4\n");
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Integer("10".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Word(Word::For));
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), Token::Unknown("%".to_string()));
        assert_eq!(
            x.next().unwrap(),
            Token::Ident(Ident::Plain("WOO".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(
            x.next().unwrap(),
            Token::Ident(Ident::Plain("IN".to_string()))
        );
        assert_eq!(x.next().unwrap(), Token::Whitespace(1));
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Single("0.".to_string()))
        );
        assert_eq!(
            x.next().unwrap(),
            Token::Literal(Literal::Single(".4".to_string()))
        );
        assert_eq!(x.next(), None);
    }
}
