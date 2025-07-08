use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use crate::msk_value::MskValue;
use crate::token::{Literal, Token, TokenType};
struct ScopeGuard<'a> {
    interpreter: &'a mut Interpreter,
}

impl<'a> ScopeGuard<'a> {
    fn new(interpreter: &'a mut Interpreter) -> Self {
        interpreter.begin_scope();  // 构造时进入作用域
        ScopeGuard { interpreter }
    }
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.interpreter.end_scope();  // 析构时自动退出作用域
    }
}
pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn interpret(&mut self, stmt: &[Stmt]) -> Result<(), String> {
        for stmt in stmt {
            match stmt {
                Stmt::Expression { expression } => {
                    self.evaluate(&expression)?;
                }
                Stmt::Print { expression } => {
                    let value = self.evaluate(&expression)?;
                    println!("{}", value);
                }
                Stmt::Var { name, initializer } => {
                    let value = if let Some(init) = initializer {
                        self.evaluate(&init)?
                    } else {
                        MskValue::Nil  // 如果没有初始化表达式，默认为 nil
                    };
                    self.env.borrow_mut().define(&name.lexeme, value);
                }
                Stmt::Block { statements } => {
                    let guard = ScopeGuard::new(self);
                    guard.interpreter.interpret(statements)?;
                }
                Stmt::If { name, condition,then_branch,else_branch } => {
                    let condition = self.evaluate(&condition)?;
                    // if let MskValue::Boolean(value) = condition {
                    let value = condition.is_true();
                        if value {
                            let stmt_wrapper = if let Stmt::Block { statements } = &**then_branch {
                                statements.as_slice()
                            } else {
                                std::slice::from_ref(&**then_branch)
                            };
                            self.interpret(stmt_wrapper)?
                        }else{
                            if let Some(else_branch) = else_branch {
                                let stmt_wrapper = if let Stmt::Block { statements } = &**else_branch {
                                    statements.as_slice()
                                } else {
                                    std::slice::from_ref(&**else_branch)
                                };
                                self.interpret(stmt_wrapper)?;
                            }
                        }
                    // }else{
                    //     return Err(format!("[line {}] Condition must be a boolean.", name.line));
                    // }
                }
                Stmt::While { name, condition, body } => {
                    let stmt_wrapper = if let Stmt::Block { statements } = &**body {
                        statements.as_slice()
                    } else {
                        std::slice::from_ref(&**body)
                    };
                    while self.evaluate(condition)?.is_true() {
                        self.interpret(stmt_wrapper)?;
                    }
                }
            }
        }
        Ok(())
    }
}
impl Interpreter {
    /// 创建一个新的 Interpreter 实例。
    fn begin_scope(&mut self) {
        self.env = Environment::new_with_parent(self.env.clone());
    }
    fn end_scope(&mut self) {
        let env = if let Some(value) = self.env.borrow().get_parent_env() {
            value
        }else{
            Rc::new(RefCell::new(Environment::new()))
        };
        self.env = env;
    }
}
impl Interpreter {
    /// 创建一个新的 Interpreter 实例。
    pub fn new() -> Self {
        Interpreter {
            env: Rc::new(RefCell::new(Environment::new()))
        }
    }

    /// 解释并执行给定的 AST 表达式。
    /// 返回一个 Result，包含执行结果或错误信息。
    pub fn evaluate(&mut self, expr: &Expr) -> Result<MskValue, String> {
        match expr {
            Expr::Unary { operator, right } => {
                let value = self.evaluate(&*right)?;
                self.evaluate_unary(&*operator, value)
            }
            Expr::Binary { left, operator, right } => {
                let left_value = self.evaluate(&*left)?;
                let right_value = self.evaluate(&*right)?;
                self.evaluate_binary(&operator, left_value, right_value)
            }
            Expr::Grouping { expression } => self.evaluate(&*expression),
            Expr::Literal { value } => {
                match value.token_type {
                    TokenType::String => Ok(MskValue::String(value.literal.as_ref().unwrap().to_string())),
                    TokenType::Number => {
                        match value.literal.as_ref().unwrap() {
                            Literal::Number(n) => Ok(MskValue::Float(*n)),
                            _ => Err(format!("Unexpected number type for token: {}", value.lexeme)),
                        }
                    }
                    TokenType::True => Ok(MskValue::Boolean(true)),
                    TokenType::False => Ok(MskValue::Boolean(false)),
                    TokenType::Nil => Ok(MskValue::Nil),
                    _ => {
                        Err(format!("Unexpected token type: {:?}", value.token_type))
                    }
                }
            },
            Expr::Variable { name } => {
                self.env.borrow().get(&name.lexeme,name.line)
            }
            Expr::Assign { name, value } => {
                let result = self.evaluate(&*value)?;
                self.env.borrow_mut().assign(&name.lexeme,result.clone())?;
                Ok(result)
            }
            Expr::Logical { left, operator, right } => {
                let left_value = self.evaluate(&*left)?;
                if operator.token_type == TokenType::Or {
                    if left_value.is_true() {
                        return Ok(left_value);
                    }
                } else if operator.token_type == TokenType::And {
                    if !left_value.is_true() {
                        return Ok(left_value);
                    }
                }
                let right_value = self.evaluate(&*right)?;
                Ok(right_value)
            }
        }
    }
    fn evaluate_binary(&self, operator: &Token, left: MskValue, right: MskValue) -> Result<MskValue, String> {
        match operator.token_type {
            TokenType::Plus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l + r)),
                (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::String(format!("{}{}", l, r))),
                _ => Err(format!("[line {}] Operands must be two numbers or two strings for '+' operator.", operator.line)),
            },
            TokenType::Minus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l - r)),
                _ => Err(format!("[line {}] Operands must be numbers for '-' operator.", operator.line)),
            },
            TokenType::Star => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Float(l * r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '*' operator.", operator.line))
                }
            },
            TokenType::Slash => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    if r == 0.0 {
                        Err(format!("[line {}] Division by zero is not allowed.", operator.line))
                    } else {
                        Ok(MskValue::Float(l / r))
                    }
                } else {
                    Err(format!("[line {}] Operands must be numbers for '/' operator.", operator.line))
                }
            },
            TokenType::Greater => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l > r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '>' operator.", operator.line))
                }
            },
            TokenType::GreaterEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l >= r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '>=' operator.", operator.line))
                }
            },
            TokenType::Less => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l < r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '<' operator.", operator.line))
                }
            },
            TokenType::LessEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l <= r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '<=' operator.", operator.line))
                }
            }
            TokenType::EqualEqual => {
                match (left,right) { 
                    (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Boolean(l == r)),
                    (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::Boolean(l == r)),
                    (MskValue::Boolean(l), MskValue::Boolean(r)) => Ok(MskValue::Boolean(l == r)),
                    // (MskValue::Nil, MskValue::Nil) => Ok(MskValue::Boolean(true)),
                    _ => Ok(MskValue::Boolean(false)), // 不同类型的比较返回 false
                }
            }
            TokenType::BangEqual => {
                match (left,right) {
                    (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Boolean(l != r)),
                    (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::Boolean(l != r)),
                    (MskValue::Boolean(l), MskValue::Boolean(r)) => Ok(MskValue::Boolean(l != r)),
                    // (MskValue::Nil, MskValue::Nil) => Ok(MskValue::Boolean(true)),
                    _ => Ok(MskValue::Boolean(true)), 
                }
            }
            _ => Err(format!("[line {}] Unsupported binary operator: {:?}", operator.line, operator)),
        }
    }
    fn evaluate_unary(&self, operator: &Token, value: MskValue) -> Result<MskValue, String> {
        match operator.token_type {
            TokenType::Minus => {
                if let MskValue::Float(n) = value {
                    Ok(MskValue::Float(-n))
                } else {
                    Err(format!("[line {}] Operand must be a number.", operator.line))
                }
            }
            TokenType::Bang => {
                Ok(MskValue::Boolean(!value.is_true()))
            }
            _ => Err(format!("[line {}] Unsupported unary operator", operator.line))
        }
    }
    // 其他方法...
}