use super::token::{Token, TokenType};

pub struct Scanner {
    code: String,
    line: usize,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(code: String) -> Scanner {
        Scanner {
            code,
            line: 0,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.start = self.current;

        match self.advance() {
            Some(current) => {
                if current.is_ascii_alphabetic() || current == b'_' {
                    return Some(self.identifier());
                }

                if current.is_ascii_digit() {
                    return Some(self.integer());
                }

                Some(match current {
                    b'{' => self.make_token(TokenType::BraceLeft),
                    b'}' => self.make_token(TokenType::BraceRight),
                    b'-' => match self.peek() {
                        Some(b'>') => {
                            self.advance();
                            self.make_token(TokenType::Arrow)
                        }
                        _ => self.make_token(TokenType::Error),
                    },
                    b';' => self.make_token(TokenType::Semicolon),
                    _ => self.make_token(TokenType::Error),
                })
            }
            None => None,
        }
    }

    pub fn get_token_text(&self, token: Token) -> &str {
        &self.code[token.start..token.start + token.length]
    }

    fn advance(&mut self) -> Option<u8> {
        if self.current >= self.code.len() {
            None
        } else {
            self.current += 1;
            Some(self.code.as_bytes()[self.current - 1])
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.current >= self.code.len() {
            None
        } else {
            Some(self.code.as_bytes()[self.current])
        }
    }

    fn peek_next(&self) -> Option<u8> {
        if self.current >= self.code.len() - 1 {
            None
        } else {
            Some(self.code.as_bytes()[self.current + 1])
        }
    }

    fn peek_previous(&self) -> Option<u8> {
        if self.current == 0 {
            None
        } else {
            Some(self.code.as_bytes()[self.current - 1])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b'\t' | b'\r' | b' ' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn identifier(&mut self) -> Token {
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_')
        {
            self.advance();
        }

        let text = &self.code[self.start..self.current];
        match text {
            "return" => self.make_token(TokenType::Return),
            _ => self.make_token(TokenType::Identifier),
        }
    }

    fn integer(&mut self) -> Token {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        self.make_token(TokenType::Integer)
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            start: self.start,
            length: self.current - self.start,
        }
    }
}
