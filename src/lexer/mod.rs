use crate::token::Token;


#[derive(Debug, Default)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect::<Vec<char>>(),
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        match self.ch {
            '=' => token = Token::ASSIGN,
            ';' => token = Token::SEMICOLON,
            '(' => token = Token::LPAREN,
            ')' => token = Token::RPAREN,
            ',' => token = Token::COMMA,
            '+' => token = Token::PLUS,
            '{' => token = Token::LBRACE,
            '}' => token = Token::RBRACE,
            '\0' => token = Token::EOF,
            c => {
                if c.is_alphabetic() {
                    token = self.read_identifier();
                } else {
                    token = Token::ILLEGAL;
                }
            }
        }

        self.read_char();
        token
    }

    pub fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_alphabetic() {
            self.read_char();
        }

        let end_pos = self.position;
        self.input[start_pos..end_pos].iter().collect()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_delimiters() {
        let input = "=+(){},;";
        let mut tokens = Lexer::new(input);

        let expected = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

    #[test]
    fn test_tokenize_program() {
        let input = "let five = 5; \
                     let ten = 10; \
                     let add = fn(x, y) { x + y;};";

        let mut tokens = Lexer::new(input);

        let expected = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::EOF,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }
}
