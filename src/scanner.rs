//! scanner.rs - 负责将源代码字符串转换为 Token 序列。
//! 这是解释器的词法分析阶段。

use crate::token::{Literal, Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

/// Scanner 结构体持有扫描过程中的所有状态。
/// 它使用一个可预读的迭代器来处理源字符串，以获得高效的性能。
pub struct Scanner<'a> {
    /// 一个可预读的字符迭代器，用于查看下一个字符而不消耗它。
    chars: Peekable<Chars<'a>>,
    /// 已扫描生成的 Token 列表。
    tokens: Vec<Token>,
    /// 当前所在的行号，用于错误报告。
    line: usize,
    /// 记录在扫描过程中是否遇到了错误。
    had_error: bool,
}

impl<'a> Scanner<'a> {
    /// 创建一个新的 Scanner 实例。
    /// 它接收对源字符串的引用，并创建一个内部迭代器。
    pub fn new(source: &'a str) -> Self {
        Scanner {
            chars: source.chars().peekable(),
            tokens: Vec::new(),
            line: 1,
            had_error: false,
        }
    }

    /// 扫描整个源代码，并返回生成的 Token 列表以及是否发生错误的标志。
    /// 此方法会消耗 Scanner 实例。
    pub fn scan_tokens(mut self) -> (Vec<Token>, bool) {
        // 主扫描循环，只要还有字符就继续。
        while let Some(c) = self.advance() {
            self.scan_token(c);
        }

        // 扫描结束后，添加一个文件结束符（Eof）Token。
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        (self.tokens, self.had_error)
    }

    /// 根据当前字符扫描并处理单个 Token。
    fn scan_token(&mut self, c: char) {
        match c {
            '(' => self.add_chars_token(TokenType::LeftParen, "("),
            ')' => self.add_chars_token(TokenType::RightParen, ")"),
            '{' => self.add_chars_token(TokenType::LeftBrace, "{"),
            '}' => self.add_chars_token(TokenType::RightBrace, "}"),
            ',' => self.add_chars_token(TokenType::Comma, ","),
            '.' => self.add_chars_token(TokenType::Dot, "."),
            '-' => self.add_chars_token(TokenType::Minus, "-"),
            '+' => self.add_chars_token(TokenType::Plus, "+"),
            ';' => self.add_chars_token(TokenType::Semicolon, ";"),
            '*' => self.add_chars_token(TokenType::Star, "*"),

            // 处理可能为双字符的 Token
            '!' => {
                let (ty, lexeme) = if self.match_char('=') { (TokenType::BangEqual, "!=") } else { (TokenType::Bang, "!") };
                self.add_chars_token(ty, lexeme);
            },
            '=' => {
                let (ty, lexeme) = if self.match_char('=') { (TokenType::EqualEqual, "==") } else { (TokenType::Equal, "=") };
                self.add_chars_token(ty, lexeme);
            },
            '<' => {
                let (ty, lexeme) = if self.match_char('=') { (TokenType::LessEqual, "<=") } else { (TokenType::Less, "<") };
                self.add_chars_token(ty, lexeme);
            },
            '>' => {
                let (ty, lexeme) = if self.match_char('=') { (TokenType::GreaterEqual, ">=") } else { (TokenType::Greater, ">") };
                self.add_chars_token(ty, lexeme);
            },

            '/' => {
                if self.match_char('/') {
                    // 注释会一直持续到行尾，我们直接忽略它。
                    while let Some(pc) = self.peek() {
                        if pc == '\n' { break; }
                        self.advance();
                    }
                } else {
                    self.add_chars_token(TokenType::Slash, "/");
                }
            }

            // 忽略空白字符
            ' ' | '\r' | '\t' => (),

            // 换行符，增加行号
            '\n' => self.line += 1,

            // 字符串字面量
            '"' => self.string(),

            // 数字字面量
            c if c.is_ascii_digit() => self.number(c),
            // 标识符或关键字
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(c),

            // 未知字符
            _ => {
                eprintln!("[line {}] Error: Unexpected character.", self.line);
                self.had_error = true;
            }
        }
    }

    /// 消费迭代器中的下一个字符并返回它。
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// 查看迭代器中的下一个字符，但不消耗它。
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    /// 如果下一个字符与 `expected` 匹配，则消耗它并返回 `true`。
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// 添加一个简单的单字符（或双字符）Token。
    fn add_chars_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens.push(Token::new(token_type, lexeme.to_string(), None, self.line));
    }

    /// 添加一个带有字面量值的 Token。
    fn add_literal_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<Literal>) {
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line));
    }

    /// 处理字符串字面量。
    fn string(&mut self) {
        let mut value = String::new();
        while let Some(c) = self.peek() {
            if c == '"' { break; }
            if c == '\n' { self.line += 1; }
            value.push(self.advance().unwrap());
        }

        if self.peek().is_none() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.had_error = true;
            return;
        }

        // 消耗结尾的 "
        self.advance();

        // 完整的词素包括引号
        let lexeme = format!("\"{}\"", value);
        self.add_literal_token(TokenType::String, lexeme, Some(Literal::String(value)));
    }

    /// 处理数字字面量。
    fn number(&mut self, first_char: char) {
        let mut lexeme = String::new();
        lexeme.push(first_char);

        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() { break; }
            lexeme.push(self.advance().unwrap());
        }

        if self.peek() == Some('.') {
            let mut ahead = self.chars.clone();
            ahead.next(); // 跳过 '.'
            if let Some(next_char) = ahead.peek() {
                if next_char.is_ascii_digit() {
                    lexeme.push(self.advance().unwrap()); // 消耗 '.'
                    while let Some(c) = self.peek() {
                        if !c.is_ascii_digit() { break; }
                        lexeme.push(self.advance().unwrap());
                    }
                }
            }
        }

        let value: f64 = lexeme.parse().unwrap();
        self.add_literal_token(TokenType::Number, lexeme, Some(Literal::Number(value)));
    }

    /// 处理标识符和关键字。
    fn identifier(&mut self, first_char: char) {
        let mut lexeme = String::new();
        lexeme.push(first_char);

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                lexeme.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let token_type = match lexeme.as_str() {
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
        self.add_chars_token(token_type, &lexeme);
    }
}
