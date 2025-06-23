//! scanner.rs - 负责将源代码字符串转换为 Token 序列。
//! 这是解释器的词法分析阶段。

use crate::token::{Literal, Token, TokenType};

/// Scanner 结构体持有扫描过程中的所有状态。
pub struct Scanner {
    /// 完整的源代码字符串。
    source: String,
    /// 已扫描生成的 Token 列表。
    tokens: Vec<Token>,
    /// 当前正在扫描的词素的起始位置。
    start: usize,
    /// 指向源代码中当前正在处理的字符的位置。
    current: usize,
    /// 当前所在的行号，用于错误报告。
    line: usize,
}

impl Scanner {
    /// 创建一个新的 Scanner 实例。
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// 扫描整个源代码，并返回生成的 Token 列表的引用。
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // 在每个新 Token 的开始，我们将 start 和 current 同步。
            self.start = self.current;
            self.scan_token();
        }

        // 扫描结束后，添加一个文件结束符（Eof）Token。
        // 这使得语法分析器可以知道什么时候到达了 Token 序列的末尾。
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        &self.tokens
    }

    /// 检查是否已经处理完所有字符。
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// 扫描并处理单个 Token。
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => { /* Ignore whitespace. */ }
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    // For now, we'll just print an error. A more robust solution is needed.
                    eprintln!("[line {}] Error: Unexpected character.", self.line);
                }
            }
        }
    }

    /// 消费当前字符并向前移动一个位置，返回被消费的字符。
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    /// 添加一个没有字面量的新 Token。
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    /// 根据给定的类型和字面量创建一个新的 Token，并将其添加到列表中。
    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    /// 检查当前字符是否与预期字符匹配。如果匹配，则消费该字符并返回 true。
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    /// 查看当前字符，但不消费它（“预读”）。
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    /// 处理字符串字面量。
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, Some(Literal::String(value)));
    }

    /// 处理数字字面量。
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    /// 查看当前字符的下一个字符，但不消费它。
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    /// 处理标识符和关键字。
    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type);
    }
}
