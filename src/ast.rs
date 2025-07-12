//! ast.rs - 定义抽象语法树（AST）的节点。
//! AST 是解析器将源代码的语法结构进行模型化的方式。

use std::fmt::format;
use std::rc::Rc;
use crate::token::{Literal, Token};

/// Expr 枚举代表了 Lox 语言中所有可能的表达式。
/// 使用 Box<Expr> 来处理递归的枚举类型，避免无限大小的问题。
#[derive(Debug)]
pub enum Expr {
    /// 一元运算表达式，例如 `-5` 或 `!true`
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// 分组表达式，例如 `( ... )`
    Grouping {
        expression: Box<Expr>,
    },
    /// 字面量表达式，例如 `123`, `"hello"`, `true`
    Literal {
        value: Token,
    },
    /// 变量访问表达式，例如 `x` 或 `myVariable`
    Variable {
        name: Token,
    },
    Assign {
        name: Token,  // 被赋值的变量标识符
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token, // 逻辑运算符，例如 `and`, `or`
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },

}

impl Expr {
    /// 将 AST 节点转换为 S-expression 字符串，用于调试和测试。
    /// 例如，一个 Unary 节点会变成 `(! true)`。
    pub fn to_string_expr(&self) -> String {
        match self {
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string_expr())
            }
            Expr::Grouping { expression } => {
                format!("(group {})", expression.to_string_expr())
            }
            Expr::Binary { left, operator, right } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.to_string_expr(),
                    right.to_string_expr())
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
            Expr::Variable { name } => {
                name.lexeme.clone()
            }
            Expr::Assign { name, value } => {
                format!("(assign {} {})", name.lexeme, value.to_string_expr())
            }
            Expr::Logical { left, operator, right } => {
                format!(
                    "({} {} {})",
                    left.to_string_expr(),
                    operator.lexeme,
                    right.to_string_expr())
            }
            Expr::Call { callee,arguments, .. } => {
                format!(
                    "(call {} {})",
                    callee.to_string_expr(),
                    arguments.iter()
                        .map(|arg| arg.to_string_expr())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
        }
    }
}
#[derive(Debug)]
pub enum Stmt{
    Print {
        expression: Expr,
    },
    Expression {
        expression: Expr,
    },
    /// 变量声明语句，例如 `var x = 5;` 或 `var y;`
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Block{
        statements: Vec<Stmt>,
    },
    If{
        name: Token,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        name: Token,
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        name: Token,
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Box<Stmt>>,
        body: Box<Stmt>,
    },
    Break {
        name: Token,
    },
    Continue {
        name: Token,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Rc<Stmt>,
    },
    Return {
        name: Token,
        value: Option<Expr>,
    },
}