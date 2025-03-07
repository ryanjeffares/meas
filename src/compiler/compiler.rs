use crate::scanner::scanner::Scanner;
use crate::scanner::token::{Token, TokenType};

use crate::ast::ast_node::{AstNode, BinaryOp};
use anyhow::{Result, anyhow};

pub struct Compiler {
    scanner: Scanner,
    previous: Option<Token>,
    current: Option<Token>,
}

impl Compiler {
    pub fn new(code: String) -> Compiler {
        Compiler {
            scanner: Scanner::new(code),
            previous: None,
            current: None,
        }
    }

    pub fn compile(&mut self) -> Result<AstNode> {
        self.advance();
        let mut declarations = vec![];

        loop {
            if self.current.is_none() {
                break;
            }

            match self.declaration() {
                Ok(node) => declarations.push(node),
                Err(err) => {
                    return Err(anyhow!(err));
                }
            }
        }

        Ok(AstNode::Program { declarations })
    }

    fn advance(&mut self) {
        self.previous = self.current;
        self.current = self.scanner.scan_token();

        if self.current.is_some() {
            println!("{:?}", self.current.unwrap());
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check_token(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check_token(&mut self, token_type: TokenType) -> bool {
        self.current.is_some_and(|t| t.token_type == token_type)
    }

    fn declaration(&mut self) -> Result<AstNode> {
        if self.match_token(TokenType::Identifier) {
            self.function(
                self.scanner
                    .get_token_text(self.previous.unwrap())
                    .to_owned(),
            )
        } else {
            Err(anyhow!("Invalid top-level declaration"))
        }
    }

    fn function(&mut self, name: String) -> Result<AstNode> {
        if self.match_token(TokenType::Arrow) {
            if !self.match_token(TokenType::Identifier) {
                return Err(anyhow!("Expected return type"));
            }
        }

        if !self.match_token(TokenType::BraceLeft) {
            return Err(anyhow!("Expected '{{'"));
        }

        let mut statements = vec![];
        while !self.match_token(TokenType::BraceRight) {
            match self.statement() {
                Ok(statement) => statements.push(statement),
                Err(err) => return Err(err),
            }

            if !self.match_token(TokenType::Semicolon) {
                return Err(anyhow!("Expected ';'"));
            }
        }

        Ok(AstNode::Function { name, statements })
    }

    fn statement(&mut self) -> Result<AstNode> {
        if self.match_token(TokenType::Return) {
            match self.expression() {
                Ok(expression) => Ok(AstNode::ReturnStatement {
                    expression: Box::new(expression),
                }),
                Err(err) => Err(err),
            }
        } else {
            Err(anyhow!("Invalid start of statement"))
        }
    }

    fn expression(&mut self) -> Result<AstNode> {
        self.term()
    }

    fn term(&mut self) -> Result<AstNode> {
        match self.primary() {
            Ok(lhs) => loop {
                if self.match_token(TokenType::Plus) {
                    match self.expression() {
                        Ok(rhs) => {
                            return Ok(AstNode::BinaryOp {
                                op: BinaryOp::Plus,
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                            });
                        }
                        Err(err) => return Err(err),
                    }
                } else if self.match_token(TokenType::Minus) {
                    match self.expression() {
                        Ok(rhs) => {
                            return Ok(AstNode::BinaryOp {
                                op: BinaryOp::Minus,
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                            });
                        }
                        Err(err) => return Err(err),
                    }
                } else {
                    return Ok(lhs);
                }
            },
            Err(err) => Err(err),
        }
    }

    fn primary(&mut self) -> Result<AstNode> {
        if self.match_token(TokenType::Integer) {
            let text = self.scanner.get_token_text(self.previous.unwrap());

            match text.parse::<i32>() {
                Ok(integer) => Ok(AstNode::IntegerLiteral { value: integer }),
                Err(err) => Err(anyhow!("Cannot parse integer from {text}: {}", err)),
            }
        } else if self.match_token(TokenType::Identifier) {
            let callee = self
                .scanner
                .get_token_text(self.previous.unwrap())
                .to_owned();

            if !self.match_token(TokenType::ParenLeft) {
                return Err(anyhow!("Expected '('"));
            }

            if !self.match_token(TokenType::ParenRight) {
                return Err(anyhow!("Expected ')'"));
            }

            Ok(AstNode::Call { callee })
        } else {
            Err(anyhow!("Invalid start of expression"))
        }
    }
}
