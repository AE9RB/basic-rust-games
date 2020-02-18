extern crate basic78;
use basic78::EnumIter;

use std::collections::HashMap;
use std::fmt;

thread_local!(
    static STRING_TO_TOKEN: HashMap<std::string::String, Token> = Token::iter()
        .cloned()
        .chain(Word::iter().map(|x| Token::Word(x.clone())))
        .chain(Operator::iter().map(|x| Token::Operator(x.clone())))
        .map(|d| (d.to_string(), d))
        .collect();
);

#[derive(Debug, PartialEq, Hash, Clone, EnumIter)]
pub enum Token {
    Unknown(String),
    Whitespace(usize),
    Literal(Literal),
    Word(Word),
    Operator(Operator),
    Ident(Ident),
    ParenOpen,
    ParenClose,
    Comma,
    Colon,
}

impl Token {
    pub fn from_string(s: &str) -> Option<Token> {
        STRING_TO_TOKEN.with(|stt| match stt.get(s) {
            Some(t) => Some(t.clone()),
            None => None,
        })
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Unknown(s) => write!(f, "{}", s),
            Token::Whitespace(u) => write!(f, "{s:>w$}", s = "", w = u),
            Token::Literal(s) => write!(f, "{}", s),
            Token::Word(s) => write!(f, "{}", s),
            Token::Operator(s) => write!(f, "{}", s),
            Token::Ident(s) => write!(f, "{}", s),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Literal {
    Single(String),
    Double(String),
    Integer(String),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Single(s) => write!(f, "{}", s),
            Literal::Double(s) => write!(f, "{}", s),
            Literal::Integer(s) => write!(f, "{}", s),
            Literal::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Clone, EnumIter)]
pub enum Word {
    Data,
    Def,
    Dim,
    Else,
    End,
    For,
    GoSub,
    GoTo,
    If,
    Input,
    Let,
    Next,
    On,
    Print,
    Read,
    Rem,
    Rem2,
    Restore,
    Return,
    Stop,
    Then,
    To,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Word::Data => write!(f, "DATA"),
            Word::Def => write!(f, "DEF"),
            Word::Dim => write!(f, "DIM"),
            Word::Else => write!(f, "ELSE"),
            Word::End => write!(f, "END"),
            Word::For => write!(f, "FOR"),
            Word::GoSub => write!(f, "GOSUB"),
            Word::GoTo => write!(f, "GOTO"),
            Word::If => write!(f, "IF"),
            Word::Input => write!(f, "INPUT"),
            Word::Let => write!(f, "LET"),
            Word::Next => write!(f, "NEXT"),
            Word::On => write!(f, "ON"),
            Word::Print => write!(f, "PRINT"),
            Word::Read => write!(f, "READ"),
            Word::Rem => write!(f, "REM"),
            Word::Rem2 => write!(f, "'"),
            Word::Restore => write!(f, "RESTORE"),
            Word::Return => write!(f, "RETURN"),
            Word::Stop => write!(f, "STOP"),
            Word::Then => write!(f, "THEN"),
            Word::To => write!(f, "TO"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Clone, EnumIter)]
pub enum Operator {
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    DivideInt,
    Caret,
    Modulus,
    Not,
    And,
    Or,
    Xor,
    Eqv,
    Imp,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Equals => write!(f, "="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::DivideInt => write!(f, "\\"),
            Operator::Caret => write!(f, "^"),
            Operator::Modulus => write!(f, "MOD"),
            Operator::Not => write!(f, "NOT"),
            Operator::And => write!(f, "AND"),
            Operator::Or => write!(f, "OR"),
            Operator::Xor => write!(f, "XOR"),
            Operator::Eqv => write!(f, "EQV"),
            Operator::Imp => write!(f, "IMP"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Ident {
    Plain(String),
    String(String),
    Single(String),
    Double(String),
    Integer(String),
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ident::Plain(s) => write!(f, "{}", s),
            Ident::String(s) => write!(f, "{}", s),
            Ident::Single(s) => write!(f, "{}", s),
            Ident::Double(s) => write!(f, "{}", s),
            Ident::Integer(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let t = Token::from_string("REM");
        assert_eq!(t, Some(Token::Word(Word::Rem)));
        let t = Token::from_string("PICKLES");
        assert_eq!(t, None);
    }
}
