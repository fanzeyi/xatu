
#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET
}

#[derive(Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    literal: String,
}

impl Token {
    pub fn new<I: ToString>(ttype: TokenType, literal: I) -> Self {
        Self {
            ttype: ttype,
            literal: literal.to_string(),
        }
    }
}
