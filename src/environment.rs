//! environment.rs - 管理变量作用域和存储
//! 环境用于存储变量名到值的映射，支持作用域的嵌套

use crate::msk_value::MskValue;
use std::collections::HashMap;

/// Environment 结构体管理变量的存储
/// 使用 HashMap 存储变量名到值的映射
pub struct Environment {
    /// 存储变量名到值的映射
    values: HashMap<String, MskValue>,
}

impl Environment {
    /// 创建一个新的空环境
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    /// 定义一个新变量
    /// 如果变量已存在，会覆盖原值（Lox 允许重新声明变量）
    pub fn define(&mut self, name: String, value: MskValue) {
        self.values.insert(name, value);
    }

    /// 获取变量的值
    /// 如果变量不存在，返回错误
    pub fn get(&self, name: &str) -> Result<MskValue, String> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Undefined variable '{}'.", name)),
        }
    }
}
