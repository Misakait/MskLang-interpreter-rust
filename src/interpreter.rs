use crate::ast::{Expr, Stmt};
use crate::control_flow::ControlFlow;
use crate::environment::Environment;
use crate::msk_value::MskValue;
use crate::token::{Literal, Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::slice;
use log::info;
use pretty_env_logger::env_logger::init_from_env;
use crate::callable::Callable;
use crate::native_fun::ClockNative;
use crate::register_natives;
use crate::user_fun::UserFunction;
#[derive(Debug)]
pub enum RuntimeError {
    Error(String),
    Control(ControlFlow),
}
impl From<String> for RuntimeError {
    fn from(error: String) -> Self {
        RuntimeError::Error(error)
    }
}
pub struct ScopeGuard<'a> {
    pub interpreter: &'a mut Interpreter,
}

impl<'a> ScopeGuard<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
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
    pub env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn interpret(&mut self, stmt: &[Stmt]) -> Result<MskValue, RuntimeError> {
        for stmt in stmt {
            match stmt {
                Stmt::Expression { expression } => {
                    return self.evaluate(&expression)
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
                Stmt::If { name, condition, then_branch, else_branch } => {
                    let condition = self.evaluate(&condition)?;
                    // if let MskValue::Boolean(value) = condition {
                    let value = condition.is_true();
                    if value {
                        let stmt_wrapper = slice::from_ref(&**then_branch);
                        return self.interpret(stmt_wrapper)
                    } else {
                        if let Some(else_branch) = else_branch {
                            let stmt_wrapper = slice::from_ref(&**else_branch);
                            return self.interpret(stmt_wrapper)
                            // let result = self.interpret(stmt_wrapper)?;
                            // return Ok(result);
                        }
                    }
                    // }else{
                    //     return Err(format!("[line {}] Condition must be a boolean.", name.line));
                    // }
                }
                Stmt::While { name, condition, body } => {
                    let stmt_wrapper = slice::from_ref(&**body);
                    while self.evaluate(condition)?.is_true() {
                        match self.interpret(stmt_wrapper) {
                            Ok(_) => {}, // 正常执行
                            Err(RuntimeError::Control(ControlFlow::Break)) => {
                                break; // 遇到 Break 语句，退出循环
                            }
                            Err(RuntimeError::Control(ControlFlow::Continue)) => {
                                continue; // 遇到 Continue 语句，跳过当前循环迭代
                            }
                            Err(e) => return Err(e), // 其他错误直接返回
                        }
                    }
                }
                Stmt::For { name, initializer, condition, increment, body } => {
                    let guard = ScopeGuard::new(self);
                    // let stmt_wrapper = if let Stmt::Block { statements } = &**body {
                    //     statements.as_slice()
                    // } else {
                    //     slice::from_ref(&**body)
                    // };
                    let stmt_wrapper = slice::from_ref(&**body);
                    match initializer.as_ref() {
                        None => {}
                        Some(expr) => {
                            let expr_slice = slice::from_ref(expr.as_ref());
                            guard.interpreter.interpret(expr_slice)?;
                        }
                    }
                    match condition {
                        Some(cond) => {
                            while guard.interpreter.evaluate(cond)?.is_true() {
                                match guard.interpreter.interpret(stmt_wrapper) {
                                    Ok(_) => {}, // 正常执行
                                    Err(RuntimeError::Control(ControlFlow::Break)) => {
                                        break; // 遇到 Break 语句，退出循环
                                    }
                                    Err(RuntimeError::Control(ControlFlow::Continue)) => {
                                        if let Some(increment) = increment.as_ref() {
                                            guard.interpreter.interpret(slice::from_ref(&**increment))?;
                                        }
                                        continue; // 遇到 Continue 语句，跳过当前循环迭代
                                    }
                                    Err(e) => return Err(e), // 其他错误直接返回
                                }
                                if let Some(increment) = increment.as_ref() {
                                    guard.interpreter.interpret(slice::from_ref(&**increment))?;
                                }
                            }
                        }
                        None => {
                            loop {
                                match guard.interpreter.interpret(stmt_wrapper) {
                                    Ok(_) => {}, // 正常执行
                                    Err(RuntimeError::Control(ControlFlow::Break)) => {
                                        break; // 遇到 Break 语句，退出循环
                                    }
                                    Err(RuntimeError::Control(ControlFlow::Continue)) => {
                                        if let Some(increment) = increment.as_ref() {
                                            guard.interpreter.interpret(slice::from_ref(&**increment))?;
                                        }
                                        continue; // 遇到 Continue 语句，跳过当前循环迭代
                                    }
                                    Err(e) => return Err(e), // 其他错误直接返回
                                }
                                if let Some(increment) = increment.as_ref() {
                                    guard.interpreter.interpret(slice::from_ref(&**increment))?;
                                }
                            }
                        } // 如果没有条件，直接进入循环
                    }
                }
                Stmt::Break { .. } => {
                    return Err(RuntimeError::Control(ControlFlow::Break));
                }
                Stmt::Continue { .. } => {
                    return Err(RuntimeError::Control(ControlFlow::Continue));
                }
                Stmt::Function { name, params, body } => {
                    let func = MskValue::Callable(Rc::new(
                        UserFunction {
                            name: name.lexeme.clone(),
                            params: params.clone(),
                            body: (*body).clone(),
                            closure: self.env.clone(),
                        }
                    ));
                    self.env.borrow_mut().define(&name.lexeme, func);
                }
                Stmt::Return { name, value } => {
                    // info!("Returning the value: {:?}", value);
                    return match value {
                        None => {
                            Ok(MskValue::Nil)
                        }
                        Some(value) => {
                            // Err(RuntimeError::Control(ControlFlow::Return(self.evaluate(value)?)))
                            Ok(self.evaluate(value)?)
                        }
                    }
                }
            }
        }
        Ok(MskValue::Nil)
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
        let global_env = Rc::new(RefCell::new(Environment::new()));

        // 使用宏注册所有原生函数
        register_natives!(global_env,
            "clock" => ClockNative,
            // 在这里添加其他原生函数，例如：
            // "sqrt" => SqrtNative,
        );

        Interpreter {
            env: global_env,
        }
    }

    /// 解释并执行给定的 AST 表达式。
    /// 返回一个 Result，包含执行结果或错误信息。
    pub fn evaluate(&mut self, expr: &Expr) -> Result<MskValue, RuntimeError> {
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
                            _ => Err(format!("Unexpected number type for token: {}", value.lexeme).into()),
                        }
                    }
                    TokenType::True => Ok(MskValue::Boolean(true)),
                    TokenType::False => Ok(MskValue::Boolean(false)),
                    TokenType::Nil => Ok(MskValue::Nil),
                    _ => {
                        Err(format!("Unexpected token type: {:?}", value.token_type).into())
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
            Expr::Call { callee, paren, arguments } => {
                let callee_value = self.evaluate(&*callee)?;
                let mut args = Vec::new();
                // info!("Callee: {:?}, Arguments: {:?}", callee_value, arguments);
                for arg in arguments {
                    args.push(self.evaluate(&*arg)?);
                }
                if let MskValue::Callable(func) = callee_value {
                    if args.len() != func.arity() {
                        return Err(format!("[line {}] Expected {} arguments but got {}.", paren.line, func.arity(), args.len()).into());
                    }
                    // func.call(self, args)
                    let result = func.call(self, args);
                    // info!("Result: {:?}",  result);
                    result
                } else {
                    Err(format!("[line {}] Can only call functions and classes.", paren.line).into())
                }
            }
        }
    }
    fn evaluate_binary(&self, operator: &Token, left: MskValue, right: MskValue) -> Result<MskValue, RuntimeError> {
        match operator.token_type {
            TokenType::Plus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l + r)),
                (MskValue::String(l), MskValue::String(r)) => Ok(MskValue::String(format!("{}{}", l, r))),
                _ => Err(format!("[line {}] Operands must be two numbers or two strings for '+' operator.", operator.line).into()),
            },
            TokenType::Minus => match (left, right) {
                (MskValue::Float(l), MskValue::Float(r)) => Ok(MskValue::Float(l - r)),
                _ => Err(format!("[line {}] Operands must be numbers for '-' operator.", operator.line).into()),
            },
            TokenType::Star => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Float(l * r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '*' operator.", operator.line).into())
                }
            },
            TokenType::Slash => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    if r == 0.0 {
                        Err(format!("[line {}] Division by zero is not allowed.", operator.line).into())
                    } else {
                        Ok(MskValue::Float(l / r))
                    }
                } else {
                    Err(format!("[line {}] Operands must be numbers for '/' operator.", operator.line).into())
                }
            },
            TokenType::Greater => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l > r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '>' operator.", operator.line).into())
                }
            },
            TokenType::GreaterEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l >= r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '>=' operator.", operator.line).into())
                }
            },
            TokenType::Less => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l < r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '<' operator.", operator.line).into())
                }
            },
            TokenType::LessEqual => {
                if let (MskValue::Float(l), MskValue::Float(r)) = (left, right) {
                    Ok(MskValue::Boolean(l <= r))
                } else {
                    Err(format!("[line {}] Operands must be numbers for '<=' operator.", operator.line).into())
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
            _ => Err(format!("[line {}] Unsupported binary operator: {:?}", operator.line, operator).into()),
        }
    }
    fn evaluate_unary(&self, operator: &Token, value: MskValue) -> Result<MskValue, RuntimeError> {
        match operator.token_type {
            TokenType::Minus => {
                if let MskValue::Float(n) = value {
                    Ok(MskValue::Float(-n))
                } else {
                    Err(format!("[line {}] Operand must be a number.", operator.line).into())
                }
            }
            TokenType::Bang => {
                Ok(MskValue::Boolean(!value.is_true()))
            }
            _ => Err(format!("[line {}] Unsupported unary operator", operator.line).into())
        }
    }
}