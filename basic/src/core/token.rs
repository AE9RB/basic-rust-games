extern crate macros;
use macros::EnumIter;

use std::collections::HashMap;
use std::fmt;

thread_local!(
    static STRING_TO_TOKEN: HashMap<std::string::String, Token> = Token::iter()
        .cloned()
        .chain(Statement::iter().map(|x| Token::Statement(x.clone())))
        .chain(Function::iter().map(|x| Token::Function(x.clone())))
        .chain(Operator::iter().map(|x| Token::Operator(x.clone())))
        .map(|d| (d.to_string(), d))
        .collect();
);

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum Token {
    Whitespace(usize),
    Statement(Statement),
    Function(Function),
    Operator(Operator),
    Ident(String),
    StringIdent(String),
    SingleIdent(String),
    DoubleIdent(String),
    IntegerIdent(String),
    Single(String),
    Double(String),
    Integer(String),
    String(String),
    Unknown(String),
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
            Token::Whitespace(u) => write!(f, "{s:>w$}", s = "", w = u),
            Token::Statement(s) => write!(f, "{}", s),
            Token::Function(s) => write!(f, "{}", s),
            Token::Operator(s) => write!(f, "{}", s),
            Token::Ident(s) => write!(f, "{}", s),
            Token::StringIdent(s) => write!(f, "{}", s),
            Token::SingleIdent(s) => write!(f, "{}", s),
            Token::DoubleIdent(s) => write!(f, "{}", s),
            Token::IntegerIdent(s) => write!(f, "{}", s),
            Token::Single(s) => write!(f, "{}", s),
            Token::Double(s) => write!(f, "{}", s),
            Token::Integer(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Unknown(s) => write!(f, "{}", s),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, EnumIter)]
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

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum Function {
    Abs,
    Asc,
    Atn,
    ChrS,
    Cos,
    Exp,
    Int,
    LeftS,
    Len,
    Log,
    MidS,
    Rnd,
    RightS,
    Sgn,
    Sin,
    Sqr,
    StrS,
    Tab,
    Tan,
    Val,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Abs => write!(f, "ABS"),
            Function::Asc => write!(f, "ASC"),
            Function::Atn => write!(f, "ATN"),
            Function::ChrS => write!(f, "CHR$"),
            Function::Cos => write!(f, "COS"),
            Function::Exp => write!(f, "EXP"),
            Function::Int => write!(f, "INT"),
            Function::LeftS => write!(f, "LEFT$"),
            Function::Len => write!(f, "LEN"),
            Function::Log => write!(f, "LOG"),
            Function::MidS => write!(f, "MID$"),
            Function::Rnd => write!(f, "RND"),
            Function::RightS => write!(f, "RIGHT$"),
            Function::Sgn => write!(f, "SGN"),
            Function::Sin => write!(f, "SIN"),
            Function::Sqr => write!(f, "SQR"),
            Function::StrS => write!(f, "STR$"),
            Function::Tab => write!(f, "TAB"),
            Function::Tan => write!(f, "TAN"),
            Function::Val => write!(f, "VAL"),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum Operator {
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    Caret,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Equals => write!(f, "="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Caret => write!(f, "^"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_token() {
        let t = Token::from_string("REM");
        assert_eq!(t, Some(Token::Statement(Statement::Rem)));
        let t = Token::from_string("PICKLES");
        assert_eq!(t, None);
    }
}
