use std::collections::HashMap;

//TODO remove #[allow(dead_code)] after lexer

thread_local!(
    static TOKEN_TO_STRING: HashMap<Token, &'static str> = [
        (Token::Eol, "\n"),
        (Token::Statement(Statement::Data), "DATA"),
        (Token::Statement(Statement::Rem), "REM"),
    ]
    .iter()
    .cloned()
    .collect();

    static STRING_TO_TOKEN: HashMap<&'static str, Token> =
        TOKEN_TO_STRING.with(|tts| tts.into_iter().map(|(t, &s)| (s, t.clone())).collect());
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

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Eol,
    Statement(Statement),
    Function(Function),
    NumberIdent(String),
    StringIdent(String),
    Number(String),
    String(String),
    ParenOpen,
    ParenClose,
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
pub enum Statement {
    Data,
    Def,
    Dim,
    End,
    For,
    To,
    GoSub,
    GoTo,
    If,
    Then,
    Else,
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
}
