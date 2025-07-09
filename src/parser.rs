//! parser.rs - 负责解析由 Scanner 生成的 Token 序列，并构建抽象语法树（AST）。
//! 这是解释器的语法分析阶段。

use crate::ast::Stmt::Expression;
use crate::ast::{Expr, Stmt};
use crate::token::{Token, TokenType};
use std::cell::Cell;

/// Parser 结构体接收一个 Token 序列，并根据 Lox 语言的语法规则进行解析。
pub struct Parser {
    /// 从 Scanner 获取到的 Token 列表。
    tokens: Vec<Token>,
    /// 指向当前正在处理的 Token 的位置。
    current: usize,
    /// 记录在解析过程中是否遇到了错误。
    had_error: Cell<bool>,
}

impl Parser {
    /// 创建一个新的 Parser 实例。
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            had_error: Cell::new(false),
        }
    }

    /// 开始解析 Token 序列，尝试构建一个 AST 表达式。
    /// 如果解析成功，返回 `Some(Expr)`；如果遇到错误，则返回 `None`。
    /// 同时返回一个布尔值，表示在解析过程中是否发生了错误。
    pub fn parse(&mut self) -> (Option<Vec<Stmt>>, bool) {
        if self.peek().token_type == TokenType::Eof {
            return (None, self.had_error.get());
        }
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.statement());
        }
        // if !self.is_at_end() {
        //     // self.error(self.peek(), "Expect end of expression.");
        // }
        if self.had_error.get() {
            (None, true)
        } else {
            (Some(stmts), false)
        }
    }
    fn statement(&mut self) -> Stmt {
        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::For]) {
            // 处理 for 循环语句
            return self.for_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            // 处理块语句
            return self.block_statement();
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::Break]) {
            return self.break_statement();
        }
        if self.match_token(&[TokenType::Continue]) {
            return self.continue_statement();
        }
        self.expression_statement()
    }
    fn for_statement(&mut self) -> Stmt {
        let name = self.previous().clone();
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        let initializer = if self.match_token(&[TokenType::Var]) {
            Some(Box::new(self.var_declaration()))
        } else if self.match_token(&[TokenType::Semicolon]) {
            None
        } else {
            Some(Box::new(self.expression_statement()))
        };

        let condition = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression())
        };
        self.consume(TokenType::Semicolon, "Expect ';' after for clauses.");

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(Box::new(self.increment_statement()))
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.");
        let body = Box::new(self.statement());

        Stmt::For {
            name,
            initializer,
            condition,
            increment,
            body,
        }
    }
    fn break_statement(&mut self) -> Stmt {
        let name = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expect ';' after break statement.");
        Stmt::Break { name }
    }
    fn continue_statement(&mut self) -> Stmt {
        let name = self.previous().clone();
        self.consume(TokenType::Semicolon, "Expect ';' after continue statement.");
        Stmt::Continue { name }
    }
    fn while_statement(&mut self) -> Stmt {
        let name = self.previous().clone();
        self.consume(TokenType::LeftParen, "Expect '(' after 'While'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after 'the condition of While statement'.");
        let body = Box::new(self.statement());
        Stmt::While {
            name,
            condition,
            body
        }
    }
    fn if_statement(&mut self) -> Stmt {
        let name = self.previous().clone();
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after 'the condition of if statement'.");
        let then_branch = Box::new(self.statement());
        let mut else_branch = None;
        if self.match_token(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()));
        }
        Stmt::If {
            name,
            condition,
            then_branch,
            else_branch,
        }
    }
    /// 解析变量声明语句
    /// var_declaration -> "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect variable name.").clone();

        let mut initializer = None;
        if self.match_token(&[TokenType::Equal]) {
            initializer = Some(self.expression());
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.");
        Stmt::Var { name, initializer }
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print {
            expression: value,
        }
    }
    fn block_statement(&mut self) -> Stmt {
        let mut stmts: Vec<Stmt> =  Vec::new();
        while !self.match_token(&[TokenType::RightBrace]) {
            if self.is_at_end(){
                self.error(self.peek(), "Expect '}' after block.");
                break;
            }
            stmts.push(self.statement());
        }
        Stmt::Block {
            statements: stmts,
        }
    }
    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Expression { expression: expr }
    }
    fn increment_statement(&mut self) -> Stmt {
        let expr = self.expression();
        Expression { expression: expr }
    }
    pub fn parse_expr(&mut self) -> (Option<Expr>, bool) {
        if self.peek().token_type == TokenType::Eof {
            return (None, self.had_error.get());
        }
        let expr = self.expression();
        if !self.is_at_end() {
            // self.error(self.peek(), "Expect end of expression.");
        }
        if self.had_error.get() {
            (None, true)
        } else {
            (Some(expr), false)
        }
    }
    /// 解析一个表达式。这是解析的入口。
    /// expression -> unary
    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.logic();
        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment(); // 右结合性：递归调用自己

            if let Expr::Variable { name } = expr {
                return Expr::Assign {
                    name,
                    value: Box::new(value),
                };
            }
            self.error(&equals, "Invalid assignment target.");
        }
        expr
    }
    /// 逻辑表达式解析入口。
    fn logic(&mut self) -> Expr {
        let mut expr = self.equality();
        while self.match_token(&[TokenType::Or,TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        if self.match_token(&[
            TokenType::EqualEqual,
            TokenType::BangEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
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
        self.call()
    }
    fn call(&mut self) -> Expr {
        let callee = self.primary();
        if self.match_token(&[TokenType::LeftParen]) {
            let mut arguments = Vec::new();
            while !self.check(&TokenType::RightParen) {
                arguments.push(self.expression());
                if self.check(&TokenType::RightParen){
                    break;
                }
                self.consume(TokenType::Comma, "Expect ',' after argument.");
            }
            if self.match_token(&[TokenType::RightParen]) {
                let paren = self.previous().clone();
                return Expr::Call {
                    callee: Box::new(callee),
                    paren,
                    arguments,
                }
            }else{
                self.error(self.peek(), "Expect ')' after arguments.");
            }
        }
        callee
    }
    /// 解析一个主表达式。
    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
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

        if self.match_token(&[TokenType::Identifier]) {
            return Expr::Variable {
                name: self.previous().clone(),
            };
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        self.error(self.peek(), "Expect expression.");
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
        self.error(self.peek(), message);
        // 返回原始的 peek 结果，因为函数签名要求返回一个引用。
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
    fn error(&self ,token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            eprintln!("[line {}] Error at end: {}", token.line, message);
        } else {
            eprintln!(
                "[line {}] Error at '{}': {}",
                token.line, token.lexeme, message
            );
        }
        self.had_error.set(true);
    }
}
