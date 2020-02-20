use super::token::*;
use std::iter::Peekable;

pub fn lex(s: &str) -> (u16, Vec<Token>) {
    let mut t = s.len();
    if s.ends_with("\r\n") {
        t -= 2
    } else if s.ends_with("\r") {
        t -= 1
    } else if s.ends_with("\n") {
        t -= 1
    }
    let l = &mut Lex {
        i: s.chars().take(t).peekable(),
        line_number: 65535,
        remark: false,
        starting: true,
        next_token: None,
    };
    let mut t = l.collect::<Vec<Token>>();
    let n = l.line_number;

    if let Some(Token::Whitespace(_)) = t.last() {
        t.pop();
    }

    if let Some(Token::Unknown(_)) = t.last() {
        if let Some(Token::Unknown(s)) = t.pop() {
            t.push(Token::Unknown(s.trim_end().to_string()));
        }
    }

    (n, t)
}

struct Lex<T: Iterator<Item = char>> {
    i: Peekable<T>,
    line_number: u16,
    remark: bool,
    starting: bool,
    next_token: Option<Token>,
}

impl<T: Iterator<Item = char>> Iterator for Lex<T> {
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
                if !self.is_direct() {
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
                if let Some(Token::Literal(lit)) = tn.as_ref() {
                    let s = match lit {
                        Literal::Integer(s) => s,
                        Literal::Single(s) => s,
                        Literal::Double(s) => s,
                        Literal::String(s) => s,
                    };
                    if s.chars().all(|c| Self::is_basic_digit(c)) {
                        if let Ok(line) = s.parse::<u16>() {
                            if line <= 65529 {
                                self.line_number = line;
                                let n = self.next();
                                if let Some(Token::Whitespace(_)) = n {
                                    return n;
                                }
                                self.next_token = n;
                                return Some(Token::Whitespace(1));
                            }
                        }
                    }
                }
            }
            return tn;
        }
        self.starting = false;

        if Self::is_basic_alphabetic(*p) {
            let r = self.alphabetic();
            if r == Some(Token::Word(Word::Rem1)) {
                self.remark = true;
            }
            if let Some(p) = self.i.peek() {
                if Self::is_basic_alphabetic(*p) {
                    if !self.is_direct() {
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

impl<T: Iterator<Item = char>> Lex<T> {
    fn is_basic_whitespace(c: char) -> bool {
        c == ' ' || c == '\t'
    }

    fn is_basic_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_basic_alphabetic(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn is_direct(&self) -> bool {
        self.line_number == 65535
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
        fn tok(s: &str) -> Token {
            let s = format!("?{}", s);
            let (_, l) = lex(&s);
            let mut i = l.iter();
            i.next();
            i.next().unwrap().clone()
        }

        assert_eq!(
            tok("3.141593"),
            Token::Literal(Literal::Single("3.141593".to_string()))
        );
        assert_eq!(
            lex("3.1415926").1.iter().next().unwrap(),
            &Token::Literal(Literal::Double("3.1415926".to_string()))
        );
        assert_eq!(
            tok("32767"),
            Token::Literal(Literal::Integer("32767".to_string()))
        );
        assert_eq!(
            tok("32768"),
            Token::Literal(Literal::Single("32768".to_string()))
        );
        assert_eq!(
            tok("24e9"),
            Token::Literal(Literal::Single("24E9".to_string()))
        );
    }

    #[test]
    fn test_remark() {
        let (ln, v) = lex(" 100REM A fortunate comment\n");
        assert_eq!(ln, 100);
        let mut x = v.iter();
        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), &Token::Word(Word::Rem1));
        assert_eq!(
            x.next().unwrap(),
            &Token::Unknown(" A fortunate comment".to_string())
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_remark2() {
        let (ln, v) = lex("100  'The comment  \r\n");
        assert_eq!(ln, 100);
        let mut x = v.iter();
        assert_eq!(x.next().unwrap(), &Token::Whitespace(2));
        assert_eq!(x.next().unwrap(), &Token::Word(Word::Rem2));
        assert_eq!(
            x.next().unwrap(),
            &Token::Unknown("The comment".to_string())
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_scanner() {
        let (ln, v) = lex("BANDS\r");
        assert_eq!(ln, 65535);
        let mut x = v.iter();
        assert_eq!(
            x.next().unwrap(),
            &Token::Ident(Ident::Plain("BANDS".to_string()))
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_for_loop() {
        let (ln, v) = lex(" forI%=1to30-10");
        assert_eq!(ln, 65535);
        let mut x = v.iter();

        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), &Token::Word(Word::For));
        assert_eq!(
            x.next().unwrap(),
            &Token::Ident(Ident::Integer("I%".to_string()))
        );
        assert_eq!(x.next().unwrap(), &Token::Operator(Operator::Equals));
        assert_eq!(
            x.next().unwrap(),
            &Token::Literal(Literal::Integer("1".to_string()))
        );
        assert_eq!(x.next().unwrap(), &Token::Word(Word::To));
        assert_eq!(
            x.next().unwrap(),
            &Token::Literal(Literal::Integer("30".to_string()))
        );
        assert_eq!(x.next().unwrap(), &Token::Operator(Operator::Minus));
        assert_eq!(
            x.next().unwrap(),
            &Token::Literal(Literal::Integer("10".to_string()))
        );
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_unk() {
        let (ln, v) = lex("10 PRINT 10");
        assert_eq!(ln, 10);
        let mut x = v.iter();
        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), &Token::Word(Word::Print));
        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
    }

    #[test]
    fn test_unknown() {
        let (ln, v) = lex("10 fOr %woo in 0..4 \n");
        assert_eq!(ln, 10);
        let mut x = v.iter();
        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), &Token::Word(Word::For));
        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
        assert_eq!(x.next().unwrap(), &Token::Unknown("%".to_string()));
        assert_eq!(
            x.next().unwrap(),
            &Token::Ident(Ident::Plain("WOO".to_string()))
        );
        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
        assert_eq!(
            x.next().unwrap(),
            &Token::Ident(Ident::Plain("IN".to_string()))
        );
        assert_eq!(x.next().unwrap(), &Token::Whitespace(1));
        assert_eq!(
            x.next().unwrap(),
            &Token::Literal(Literal::Single("0.".to_string()))
        );
        assert_eq!(
            x.next().unwrap(),
            &Token::Literal(Literal::Single(".4".to_string()))
        );
        assert_eq!(x.next(), None);
    }
}
