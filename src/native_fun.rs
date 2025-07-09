use std::time::{SystemTime, UNIX_EPOCH};
use crate::callable::Callable;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::msk_value::MskValue;

pub struct ClockNative;
impl Callable for ClockNative {
    fn arity(&self) -> usize { 0 }
    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<MskValue>) -> Result<MskValue, RuntimeError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64().round();
        Ok(MskValue::Float(now))
    }
}
impl Default for ClockNative {
    fn default() -> Self {
        ClockNative {}
    }
}
/// 宏：将原生函数注册到环境中
///
/// # 参数
/// - `$env`: 一个表达式，其类型为 Rc<RefCell<Environment>>，表示要注册函数的环境。
/// - `$name`: 函数的字符串名称。
/// - `$ty`: 实现 MskCallable trait 的函数结构体类型。
///
#[macro_export]
macro_rules! register_natives {
    ($env:expr, $( $name:expr => $ty:ty ),* $(,)? ) => {
        $(
            $env.borrow_mut().define(
                $name,
                $crate::msk_value::MskValue::Callable(Rc::new(<$ty>::default()))
            );
        )*
    };
}