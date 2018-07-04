
use std::char;
use std::slice::Iter;
use std::vec::Vec;
use std::iter::Peekable;
use std::iter::Iterator;

use super::types::{TokenType, Token};

macro_rules! combine {
    ( $( $x: expr ), * ) => { {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.append(& mut $x.collect::<Vec<_>>());
         )*
        temp_vec
    }}
}

lazy_static! {
    static ref IDENTIFIER: Vec<u8> = combine!(b'a'..=b'z', b'A'..=b'Z', b'_'..=b'_');
    static ref INTEGERS: Vec<u8> = (b'0'..=b'9').collect();
    static ref KEYWORDS: Vec<&'static str> = vec!["let", "fn"];
}

struct Lexer<'a> {
    input: Peekable<Iter<'a, u8>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a String) -> Self {
        Self {
            input: input.as_bytes().iter().peekable()
        }
    }
}

impl<'a> Lexer<'a> {
    fn read_identifier(&mut self) -> Option<String> {
        let mut result: Vec<u8> = Vec::new();

        loop {
            if let Some(c) = self.input.peek() {
                if IDENTIFIER.contains(c) {
                    result.push(**c);
                } else {
                    break;
                }
            } else {
                break;
            }

            self.input.next();
        }

        String::from_utf8(result).ok()
    }

    fn read_int(&mut self) -> Option<String> {
        let mut result: Vec<u8> = Vec::new();

        loop {
            if let Some(c) = self.input.peek() {
                if INTEGERS.contains(c) {
                    result.push(**c);
                } else {
                    break;
                }
            } else {
                break;
            }

            self.input.next();
        }

        String::from_utf8(result).ok()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let skippable = { 
                if let Some(c) = self.input.peek() {
                    **c != b' ' && **c != b'\n' && **c != 0_u8
                } else {
                   true 
                }
            };

            if skippable {
                break;
            }

            self.input.next();
        }

        let ttype = match self.input.peek()? {
            b'=' => TokenType::ASSIGN,
            b'+' => TokenType::PLUS,
            b'(' => TokenType::LPAREN,
            b')' => TokenType::RPAREN,
            b'{' => TokenType::LBRACE,
            b'}' => TokenType::RBRACE,
            b',' => TokenType::COMMA,
            b';' => TokenType::SEMICOLON,
            0_u8 => TokenType::EOF,
            b'0'...b'9' => TokenType::INT,
            c => {
                if IDENTIFIER.contains(c) {
                    TokenType::IDENT
                } else {
                    TokenType::ILLEGAL
                }
            }
        };

        let literal = if ttype == TokenType::IDENT {
            self.read_identifier()
        } else if ttype == TokenType::INT {
            self.read_int()
        } else {
            self.input.next().map(|c| char::from(*c).to_string())
        }?;

        let ttype = if ttype == TokenType::IDENT && KEYWORDS.contains(&literal.as_str()) {
            match literal.as_str() {
                "let" => TokenType::LET,
                "fn" => TokenType::FUNCTION,
                _ => TokenType::IDENT,
            }
        } else {
            ttype
        };

        Some(Token::new(ttype, literal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);".to_string();
        let answers = vec![
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),

            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::SEMICOLON, ";"),

            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::FUNCTION, "fn"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::SEMICOLON, ";"),

            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "result"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::SEMICOLON, ";"),
        ];

        let lexer = Lexer::new(&input);

        for (result, answer) in lexer.zip(answers.iter()) {
            assert_eq!(result, *answer);
            println!("{:?}", answer);
        }
    }
}
