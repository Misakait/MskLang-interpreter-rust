//! token.rs - 定义解释器中的 Token、Token 类型和字面量。
//! 这是词法分析器和语法分析器的基础数据结构。

use std::fmt::{self, Display};

/// TokenType 枚举定义了 Lox 语言中所有可能的 Token 类型。
/// 这包括单字符、多字符、字面量和关键字。
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // 单字符 Token。
    LeftParen, RightParen, LeftBrace, RightBrace, // ( ) { }
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, // , . - + ; / *

    // 一个或两个字符的 Token。
    Bang, BangEqual,     // ! !=
    Equal, EqualEqual,   // = ==
    Greater, GreaterEqual, // > >=
    Less, LessEqual,     // < <=

    // 字面量。
    Identifier, // 标识符
    String,     // 字符串
    Number,     // 数字

    // 关键字。
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof // 文件结束符
}

/// Token 结构体代表一个由词法分析器从源代码中生成的最小单元。
#[derive(Debug, Clone)]
pub struct Token {
    /// Token 的类型，来自 TokenType 枚举。
    pub token_type: TokenType,
    /// 词素：在源代码中原始的文本，例如 `var` 或 `123`。
    pub lexeme: String,
    /// 如果 Token 是一个字面量（如字符串或数字），这里会存储它的值。
    pub literal: Option<Literal>,
    /// Token 所在的行号，用于错误报告。
    pub line: usize,
}

/// Literal 枚举表示字面量的值。
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_type_str = format!("{:?}", self.token_type).to_uppercase();
        let token_type: String = match token_type_str.as_str() {
            "LEFTPAREN" => "LEFT_PAREN".to_string(),
            "RIGHTPAREN" => "RIGHT_PAREN".to_string(),
            "LEFTBRACE" => "LEFT_BRACE".to_string(),
            "RIGHTBRACE" => "RIGHT_BRACE".to_string(),
            "EQUALEQUAL" => "EQUAL_EQUAL".to_string(),
            "BANGEQUAL" => "BANG_EQUAL".to_string(),
            "GREATEREQUAL" => "GREATER_EQUAL".to_string(),
            "LESSEQUAL" => "LESS_EQUAL".to_string(),
            s => s.to_string(),
        };
        let lexeme = &self.lexeme;
        let literal = match &self.literal {
            Some(Literal::Number(n)) => {
                if n.fract() == 0.0 {
                    format!("{0:.1}", n)
                } else {
                    format!("{}", n)
                }
            },
            Some(Literal::String(s)) => s.clone(),
            None => "null".to_string(),
        };
        write!(f, "{} {} {}", token_type, lexeme, literal)
    }
}

impl Token {
    /// 创建一个新的 Token 实例。
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
