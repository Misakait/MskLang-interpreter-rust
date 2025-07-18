//! main.rs - 解释器的主入口文件。
//! 负责处理命令行参数、读取文件，并协调 Scanner 和 Parser 的工作。

// 声明并引入项目内的模块，以便在 main.rs 中使用它们。
mod token;
mod scanner;
mod parser;
mod ast;
mod msk_value;
mod interpreter;
mod environment;
mod control_flow;
mod callable;
mod native_fun;
mod user_fun;

use std::env;
// 用于处理命令行参数
use std::fs;
// 用于文件系统操作，如读取文件
use std::io::{self, Write};
// 用于 I/O 操作，特别是向 stderr 写入错误信息
use std::process::exit;
use log::info;
// 用于以特定的退出码终止程序

use crate::interpreter::RuntimeError;
use parser::Parser;
// 从我们自己的模块中导入所需的结构体。
use scanner::Scanner;

/// 程序的主函数。
fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    // 收集命令行参数。
    let args: Vec<String> = env::args().collect();
    // 需要至少两个参数：命令（如 `parse`）和文件名。
    if args.len() < 3 {
        // 如果参数不足，向标准错误输出用法信息。
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return;
    }
    let command = &args[1];
    let filename = &args[2];
    let mut had_error = false;
    let mut interpreter_error = false;
    // 读取指定文件的内容。
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        had_error = true;
        "".to_string()
    });
    // 根据命令执行不同的操作。
    match command.as_str() {
        "tokenize" => {
            // 创建一个新的 Scanner 实例。
            let scanner = Scanner::new(&file_contents);
            // 扫描文件内容以生成 Token。
            let (tokens, error) = scanner.scan_tokens();
            had_error = error;
            // 遍历并打印每个 Token。
            for token in tokens {
                println!("{}", token);
            }
        }
        "parse" => {
            // 1. 扫描阶段
            let scanner = Scanner::new(&file_contents);
            let (tokens, had_scanner_error) = scanner.scan_tokens();

            // 2. 解析阶段
            let mut parser = Parser::new(tokens);
            let (expr_option, had_parser_error) = parser.parse_expr();

            // 检查在任何阶段是否发生了错误
            had_error = had_scanner_error || had_parser_error;

            // 如果没有错误并且成功生成了 AST，则打印它
            if !had_error {
                if let Some(expr) = expr_option {
                    println!("{}", expr.to_string_expr());
                }
            }
        }
        "evaluate" => {
            // 1. 扫描阶段
            let scanner = Scanner::new(&file_contents);
            let (tokens, had_scanner_error) = scanner.scan_tokens();

            // 2. 解析阶段
            let mut parser = Parser::new(tokens);
            let (expr_option, had_parser_error) = parser.parse_expr();

            // 检查在任何阶段是否发生了错误
            had_error = had_scanner_error || had_parser_error;

            // 3. 解释阶段
            if !had_error {
                if let Some(expr) = expr_option {
                    let mut interpreter = interpreter::Interpreter::new();
                    match interpreter.evaluate(&expr) {
                        Ok(value) => println!("{}", value),
                        Err(RuntimeError::Error(e)) => {
                            writeln!(io::stderr(), "Runtime error: {}", e).unwrap();
                            interpreter_error = true;
                        },
                        _=>{}
                    }
                }
            }
        }
        "run" => {
            // 1. 扫描阶段
            let scanner = Scanner::new(&file_contents);
            let (tokens, had_scanner_error) = scanner.scan_tokens();
            // 2. 解析阶段
            let mut parser = Parser::new(tokens);
            let (stmts_option, had_parser_error) = parser.parse();
            // 检查在任何阶段是否发生了错误
            had_error = had_scanner_error || had_parser_error;

            // 3. 执行阶段
            if !had_error {
                if let Some(stmts) = stmts_option {
                    let mut interpreter = interpreter::Interpreter::new();
                    if let Err(RuntimeError::Error(e)) = interpreter.interpret(stmts.as_slice()) {
                        writeln!(io::stderr(), "Runtime error: {}", e).unwrap();
                        interpreter_error = true;
                    }
                }
            }
        }
        _ => {
            // 如果命令未知，则报告错误并以非零状态码退出。
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            exit(65);
        }
    }

    // 如果在任何阶段遇到了错误，则以状态码 65 退出。
    if had_error {
        exit(65);
    }else if interpreter_error {
        exit(70);
    } else {
        // 如果一切正常，则以状态码 0 退出。
        exit(0);
    }
}
