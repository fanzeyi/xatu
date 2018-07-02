
use std::io::{BufReader, Read};
use std::str;
use std::iter::Iterator;

use super::types::{TokenType, Token};

struct Lexer<'a> {
    input: BufReader<&'a [u8]>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a String) -> Self {
        Self {
            input: BufReader::new(input.as_bytes())
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 1];

        if let Err(_) = self.input.read(&mut buf) {
            Some(Token::new(TokenType::EOF, ""))
        } else {
            let ttype = match buf[0] {
                b'=' => TokenType::ASSIGN,
                b'+' => TokenType::PLUS,
                b'(' => TokenType::LPAREN,
                b')' => TokenType::RPAREN,
                b'{' => TokenType::LBRACE,
                b'}' => TokenType::RBRACE,
                b',' => TokenType::COMMA,
                b';' => TokenType::SEMICOLON,
                0_u8 => TokenType::EOF,
                _ => TokenType::ILLEGAL,
            };
            let literal = str::from_utf8(&buf).ok()?;

            Some(Token::new(ttype, literal))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let answers = vec![
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::EOF, "\u{0}"),
        ];

        let lexer = Lexer::new(&input);

        for (result, answer) in lexer.zip(answers.iter()) {
            assert_eq!(result, *answer);
        }
    }
}
