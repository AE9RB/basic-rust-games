use super::token::*;
use std::str::Chars;

pub struct Lex<'a> {
    i: Chars<'a>,
}

impl<'a> Lex<'a> {
    pub fn new(s: &'a String) -> Lex {
        Lex { i: s.chars() }
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.i.next();
        match n {
            Some(n) => Some(Token::StringIdent(n.to_string())),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let s = String::from("Hello");
        let mut x = Lex::new(&s);
        assert_eq!(x.next(), None);
    }
}
