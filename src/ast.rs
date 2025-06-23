//! ast.rs - 定义抽象语法树（AST）的节点。
//! AST 是解析器将源代码的语法结构进行模型化的方式。

use crate::token::{Literal, Token};

/// Expr 枚举代表了 Lox 语言中所有可能的表达式。
/// 使用 Box<Expr> 来处理递归的枚举类型，避免无限大小的问题。
pub enum Expr {
    /// 分组表达式，例如 `( ... )`
    Grouping {
        expression: Box<Expr>,
    },
    /// 字面量表达式，例如 `123`, `"hello"`, `true`
    Literal {
        value: Token,
    },
}

impl Expr {
    /// 将 AST 节点转换为 S-expression 字符串，用于调试和测试。
    /// 例如，一个 Grouping 节点会变成 `(group ...)`。
    pub fn to_string_sexpr(&self) -> String {
        match self {
            Expr::Grouping { expression } => {
                format!("(group {})", expression.to_string_sexpr())
            }
            Expr::Literal { value } => {
                if let Some(literal) = &value.literal {
                    match literal {
                        Literal::Number(n) => {
                            if n.fract() == 0.0 {
                                format!("{:.1}", n)
                            } else {
                                format!("{}", n)
                            }
                        }
                        Literal::String(s) => s.clone(),
                    }
                } else {
                    // 对于 true, false, nil 等没有字面量值的 Token
                    value.lexeme.clone()
                }
            }
        }
    }
}

