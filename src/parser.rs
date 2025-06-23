//! parser.rs - 负责解析由 Scanner 生成的 Token 序列。
//! 这是解释器的语法分析阶段。

use crate::token::{Token, TokenType};

/// Parser 结构体接收一个 Token 序列，并根据 Lox 语言的语法规则进行解析。
pub struct Parser {
    /// 从 Scanner 获取到的 Token 列表。
    tokens: Vec<Token>,
    /// 指向当前正在处理的 Token 的位置。
    current: usize,
}

impl Parser {
    /// 创建一个新的 Parser 实例。
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    /// 开始解析 Token 序列。
    /// 在这个阶段，它会寻找并打印 `true`、`false` 和 `nil`。
    pub fn parse(&mut self) {
        while !self.is_at_end() {
            let token = self.advance();
            match token.token_type {
                TokenType::True => println!("true"),
                TokenType::False => println!("false"),
                TokenType::Nil => println!("nil"),
                // 忽略所有其他 Token。
                _ => ()
            }
        }
    }

    /// 检查是否已经到达 Token 序列的末尾。
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// 查看当前 Token，但不消费它。
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// 消费当前 Token 并向前移动一个位置，返回被消费的 Token。
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// 返回前一个被消费的 Token。
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
