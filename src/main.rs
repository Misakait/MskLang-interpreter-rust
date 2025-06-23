//! main.rs - 解释器的主入口文件。
//! 负责处理命令行参数、读取文件，并协调 Scanner 和 Parser 的工作。

// 声明并引入项目内的模块，以便在 main.rs 中使用它们。
mod token;
mod scanner;
mod parser;

use std::env;
// 用于处理命令行参数
use std::fs;
// 用于文件系统操作，如读取文件
use std::io::{self, Write};
// 用于 I/O 操作，特别是向 stderr 写入错误信息
use std::process::exit;
// 用于以特定的退出码终止程序

use parser::Parser;
// 从我们自己的模块中导入所需的结构体。
use scanner::Scanner;

/// 程序的主函数。
fn main() {
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

    // 读取指定文件的内容。
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    // 如果文件内容为空，则直接返回。
    if file_contents.is_empty() {
        return;
    }

    // 根据命令执行不同的操作。
    match command.as_str() {
        "tokenize" => {
            // 创建一个新的 Scanner 实例。
            let scanner = Scanner::new(&file_contents);
            // 扫描文件内容以生成 Token。
            let tokens = scanner.scan_tokens();
            // 遍历并打印每个 Token。
            for token in tokens {
                println!("{}", token);
            }
        }
        "parse" => {
            // 创建 Scanner 并生成 Token。
            let scanner = Scanner::new(&file_contents);
            let tokens = scanner.scan_tokens(); // 克隆 Token 以便传递给 Parser
            // 创建一个新的 Parser 实例。
            let mut parser = Parser::new(tokens);
            // 开始解析过程。
            parser.parse();
        }
        _ => {
            // 如果命令未知，则报告错误并以非零状态码退出。
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            exit(65);
        }
    }
}
