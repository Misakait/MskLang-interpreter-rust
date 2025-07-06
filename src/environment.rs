//! environment.rs - 管理变量作用域和存储
//! 环境用于存储变量名到值的映射，支持作用域的嵌套

use std::cell::RefCell;
use crate::msk_value::MskValue;
use std::collections::HashMap;
use std::rc::Rc;

/// Environment 结构体管理变量的存储
/// 使用 HashMap 存储变量名到值的映射
#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, MskValue>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// 创建一个新的空环境
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let env = Environment {
            values: HashMap::new(),
            parent: Some(parent),
        };
        Rc::new(RefCell::new(env))
    }

    /// 定义一个新变量
    /// 如果变量已存在，会覆盖原值（Lox 允许重新声明变量）
    pub fn define(&mut self, name: String, value: MskValue) {
        self.values.insert(name, value);
    }

    /// 获取变量的值
    /// 如果变量不存在，返回错误
    pub fn get(&self, name: &str,line: usize) -> Result<MskValue, String> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => {
                match self.get_from_parent(name) {
                    Some(value) => Ok(value),
                    None => {
                        Err(format!("[line {}] Undefined variable '{}'.", line, name))
                    }
                }
            }
        }
    }

    fn get_from_parent(&self, name: &str) -> Option<MskValue> {
        if let Some(parent) = &self.parent {
            match parent.borrow().values.get(name){
                Some(value) => Some(value.clone()),
                None => parent.borrow().get_from_parent(name),
            }
        }else {
            None
        }
    }
    pub fn assign(&mut self, name: &str, value: MskValue) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            match &self.parent{
                None => Err(format!("Undefined variable '{}'.", name)),
                Some(p) => {
                    p.borrow_mut().assign(name, value)
                }
            }
        }
    }
    pub fn get_parent_env(&self) -> Option<Rc<RefCell<Environment>>> {
       match &self.parent{
           None => None,
           Some(env) => {
                Some(env.clone())
           }
       }
    }

}
