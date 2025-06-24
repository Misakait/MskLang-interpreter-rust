//! parser.rs - 负责解析由 Scanner 生成的 Token 序列，并构建抽象语法树（AST）。
//! 这是解释器的语法分析阶段。

use crate::ast::Expr;
use crate::token::{Token, TokenType};

/// Parser 结构体接收一个 Token 序列，并根据 Lox 语言的语法规则进行解析。
pub struct Parser {
    /// 从 Scanner 获取到的 Token 列表。
    tokens: Vec<Token>,
    /// 指向当前正在处理的 Token 的位置。
    current: usize,
    /// 记录在解析过程中是否遇到了错误。
    had_error: bool,
}

impl Parser {
    /// 创建一个新的 Parser 实例。
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    /// 开始解析 Token 序列，尝试构建一个 AST 表达式。
    /// 如果解析成功，返回 `Some(Expr)`；如果遇到错误，则返回 `None`。
    /// 同时返回一个布尔值，表示在解析过程中是否发生了错误。
    pub fn parse(&mut self) -> (Option<Expr>, bool) {
        if self.peek().token_type == TokenType::Eof {
            return (None, self.had_error);
        }
        let expr = self.expression();
        if !self.is_at_end() {
            // self.error(self.peek(), "Expect end of expression.");
        }
        if self.had_error {
            (None, true)
        } else {
            (Some(expr), false)
        }
    }

    /// 解析一个表达式。这是解析的入口。
    /// expression -> unary
    fn expression(&mut self) -> Expr {
        self.comparison()
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// 解析一元表达式。
    /// unary -> ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        // 如果不是一元运算符，则继续解析主表达式。
        self.primary()
    }

    /// 解析一个主表达式。
    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Expr {

        if self.match_token(&[
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::String,
            TokenType::Number,
        ]) {
            return Expr::Literal {
                value: self.previous().clone(),
            };
        }


        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        // self.error(self.peek(), "Expect expression.");
        Expr::Literal {
            value: self.peek().clone(),
        }
    }

    /// 检查当前 Token 是否是预期类型之一。如果是，则消耗它并返回 true。
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// 消耗一个特定类型的 Token。如果当前 Token 不是预期类型，则报告错误。
    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(&token_type) {
            return self.advance();
        }
        // self.error(self.peek(), message);
        self.peek()
    }

    /// 检查当前 Token 的类型是否与预期匹配，但不消耗它。
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    /// 消费当前 Token 并向前移动一个位置，返回被消费的 Token。
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// 检查是否已经到达 Token 序列的末尾（即下一个 Token 是 EOF）。
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// 查看当前 Token，但不消费它。
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// 返回前一个被消费的 Token。
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// 报告一个解析错误。
    fn error(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            eprintln!("[line {}] Error at end: {}", token.line, message);
        } else {
            eprintln!(
                "[line {}] Error at '{}': {}",
                token.line, token.lexeme, message
            );
        }
        self.had_error = true;
    }
}
