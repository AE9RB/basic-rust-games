use std::collections::HashMap;
use std::fmt;

//TODO remove #[allow(dead_code)] after lexer

thread_local!(
    static TOKEN_TO_STRING: HashMap<Token, &'static str> = [
        // This would be better to #derive using fmt::Display
        // where completeness is checked by match.
        (Token::Statement(Statement::Data), "DATA"),
        (Token::Statement(Statement::Def), "DEF"),
        (Token::Statement(Statement::Dim), "DIM"),
        (Token::Statement(Statement::Else), "ELSE"),
        (Token::Statement(Statement::End), "END"),
        (Token::Statement(Statement::For), "FOR"),
        (Token::Statement(Statement::GoSub), "GOSUB"),
        (Token::Statement(Statement::GoTo), "GOTO"),
        (Token::Statement(Statement::If), "IF"),
        (Token::Statement(Statement::Input), "INPUT"),
        (Token::Statement(Statement::Let), "LET"),
        (Token::Statement(Statement::Next), "NEXT"),
        (Token::Statement(Statement::On), "ON"),
        (Token::Statement(Statement::Print), "PRINT"),
        (Token::Statement(Statement::Read), "READ"),
        (Token::Statement(Statement::Rem), "REM"),
        (Token::Statement(Statement::Restore), "RESTORE"),
        (Token::Statement(Statement::Return), "RETURN"),
        (Token::Statement(Statement::Stop), "STOP"),
        (Token::Statement(Statement::Then), "THEN"),
        (Token::Statement(Statement::To), "TO"),
        (Token::Function(Function::Abs), "ABS"),
        (Token::Function(Function::Asc), "ASC"),
        (Token::Function(Function::Atn), "ATN"),
        (Token::Function(Function::ChrStr), "CHR$"),
        (Token::Function(Function::Cos), "COS"),
        (Token::Function(Function::Exp), "EXP"),
        (Token::Function(Function::Int), "INT"),
        (Token::Function(Function::LeftStr), "LEFT$"),
        (Token::Function(Function::Len), "LEN"),
        (Token::Function(Function::Log), "LOG"),
        (Token::Function(Function::MidStr), "MID$"),
        (Token::Function(Function::Rnd), "RND"),
        (Token::Function(Function::RightStr), "RIGHT$"),
        (Token::Function(Function::Sgn), "SGN"),
        (Token::Function(Function::Sin), "SIN"),
        (Token::Function(Function::Sqr), "SQR"),
        (Token::Function(Function::StrStr), "STR$"),
        (Token::Function(Function::Tab), "TAB"),
        (Token::Function(Function::Tan), "TAN"),
        (Token::Function(Function::Val), "VAL"),
    ]
    .iter()
    .cloned()
    .collect();

    static STRING_TO_TOKEN: HashMap<&'static str, Token> =
        TOKEN_TO_STRING.with(|tts| tts.into_iter().map(|(t, &s)| (s, t.clone())).collect());

    static MAX_TOKEN_LEN: usize =
        TOKEN_TO_STRING.with(|tts| tts.into_iter().map(|(_, &s)| s.len()).max().unwrap());
);

pub fn token_to_str(t: &Token) -> Option<&str> {
    TOKEN_TO_STRING.with(|tts| match tts.get(t) {
        Some(s) => Some(*s),
        None => None,
    })
}

pub fn str_to_token(s: &str) -> Option<Token> {
    STRING_TO_TOKEN.with(|stt| match stt.get(s) {
        Some(t) => Some(t.clone()),
        None => None,
    })
}

pub fn max_token_len() -> usize {
    MAX_TOKEN_LEN.with(|mts| *mts)
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Eol,
    Whitespace(usize),
    Statement(Statement),
    Function(Function),
    Ident(String),
    StringIdent(String),
    IntegerIdent(String),
    Number(String),
    String(String),
    ParenOpen,
    ParenClose,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Eol => write!(f, "\n"),
            Token::Whitespace(u) => {
                write!(f, "{}", std::iter::repeat(" ").take(*u).collect::<String>())
            }
            Token::Statement(s) => write!(f, "{}", s),
            Token::Function(s) => write!(f, "{}", s),
            Token::Ident(s) => write!(f, "{}", s),
            Token::StringIdent(s) => write!(f, "{}$", s),
            Token::IntegerIdent(s) => write!(f, "{}%", s),
            Token::Number(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
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
        let t = Token::Statement(self.clone());
        TOKEN_TO_STRING.with(|tts| match tts.get(&t) {
            Some(s) => write!(f, "{}", *s),
            None => panic!("No string for Statement::{:?}", self),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
pub enum Function {
    Abs,
    Asc,
    Atn,
    ChrStr,
    Cos,
    Exp,
    Int,
    LeftStr,
    Len,
    Log,
    MidStr,
    Rnd,
    RightStr,
    Sgn,
    Sin,
    Sqr,
    StrStr,
    Tab,
    Tan,
    Val,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = Token::Function(self.clone());
        TOKEN_TO_STRING.with(|tts| match tts.get(&t) {
            Some(s) => write!(f, "{}", *s),
            None => panic!("No string for Function::{:?}", self),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_str() {
        let s = token_to_str(&Token::Statement(Statement::Rem));
        assert_eq!(s, Some("REM"));
        let t = Token::StringIdent("A$".to_string());
        let s = token_to_str(&t);
        assert_eq!(s, None);
    }

    #[test]
    fn test_str_to_token() {
        let t = str_to_token("REM");
        assert_eq!(t, Some(Token::Statement(Statement::Rem)));
        let t = str_to_token("PICKLES");
        assert_eq!(t, None);
    }

    #[test]
    fn test_max_token_len() {
        assert_eq!(max_token_len(), 7); // RESTORE is longest
    }
}
