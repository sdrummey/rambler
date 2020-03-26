use std::fmt;

// type TokenType = String;
// #[derive(Debug)]
// pub struct Token {
//     token_type: TokenType,
//     literal: String,
// }


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special Types
    ILLEGAL,
    EOF,

    // Identifier
    IDENT(String),

    // Literals
    INT(i32),

    // Operators
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::IDENT(id) => write!(f, "{}", id),
            Token::INT(int) => write!(f, "{}", int),
            Token::ASSIGN => write!(f, "="),
            Token::PLUS => write!(f, "+"),
            Token::COMMA => write!(f, ","),
            Token::SEMICOLON => write!(f, ":"),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::FUNCTION => write!(f, "FUNCTION"),
            Token::LET => write!(f, "LET"),
            token=> write!(f, "{:?}", token)
        }
    }

}
