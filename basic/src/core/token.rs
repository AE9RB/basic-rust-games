extern crate macros;
use macros::EnumIter;

use std::collections::HashMap;
use std::fmt;

thread_local!(
    static STRING_TO_TOKEN: HashMap<std::string::String, Token> = Token::iter()
        .cloned()
        .chain(Statement::iter().map(|x| Token::Statement(x.clone())))
        .chain(Operator::iter().map(|x| Token::Operator(x.clone())))
        .map(|d| (d.to_string(), d))
        .collect();
);

#[derive(Debug, PartialEq, Hash, Clone, EnumIter)]
pub enum Token {
    Unknown(String),
    Whitespace(usize),
    Literal(Literal),
    Statement(Statement),
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
    pub fn scan_string(s: &str) -> (Option<Token>, Option<Token>) {
        STRING_TO_TOKEN.with(|stt| {
            let mut ret: (Option<Token>, Option<Token>) = (None, None);
            let mut ident_len = s.len();
            for (key_str, tok) in stt.iter() {
                if s.ends_with(key_str) {
                    let ident = &s[0..s.len() - key_str.len()];
                    if ident.len() < ident_len {
                        ident_len = ident.len();
                        if ident.len() > 0 {
                            ret = (
                                Some(Token::Ident(Ident::Plain(ident.to_string()))),
                                Some(tok.clone()),
                            );
                        } else {
                            ret = (Some(tok.clone()), None);
                        }
                    }
                }
            }
            return ret;
        })
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Unknown(s) => write!(f, "{}", s),
            Token::Whitespace(u) => write!(f, "{s:>w$}", s = "", w = u),
            Token::Literal(s) => write!(f, "{}", s),
            Token::Statement(s) => write!(f, "{}", s),
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
pub enum Statement {
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
    Restore,
    Return,
    Stop,
    Then,
    To,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Data => write!(f, "DATA"),
            Statement::Def => write!(f, "DEF"),
            Statement::Dim => write!(f, "DIM"),
            Statement::Else => write!(f, "ELSE"),
            Statement::End => write!(f, "END"),
            Statement::For => write!(f, "FOR"),
            Statement::GoSub => write!(f, "GOSUB"),
            Statement::GoTo => write!(f, "GOTO"),
            Statement::If => write!(f, "IF"),
            Statement::Input => write!(f, "INPUT"),
            Statement::Let => write!(f, "LET"),
            Statement::Next => write!(f, "NEXT"),
            Statement::On => write!(f, "ON"),
            Statement::Print => write!(f, "PRINT"),
            Statement::Read => write!(f, "READ"),
            Statement::Rem => write!(f, "REM"),
            Statement::Restore => write!(f, "RESTORE"),
            Statement::Return => write!(f, "RETURN"),
            Statement::Stop => write!(f, "STOP"),
            Statement::Then => write!(f, "THEN"),
            Statement::To => write!(f, "TO"),
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
        assert_eq!(t, Some(Token::Statement(Statement::Rem)));
        let t = Token::from_string("PICKLES");
        assert_eq!(t, None);
    }

    #[test]
    fn test_scan_string() {
        let t = Token::scan_string("BAND");
        println!("{:?}", t);
        assert_eq!(
            t,
            (
                Some(Token::Ident(Ident::Plain("B".to_string()))),
                Some(Token::Operator(Operator::And))
            )
        );
        let t = Token::scan_string("FOR");
        println!("{:?}", t);
        assert_eq!(t, (Some(Token::Statement(Statement::For)), None));
    }
}
