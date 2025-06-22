use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let mut is_error =  false;
    let mut line: u64 = 1;
    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut chars = file_contents.chars().peekable();
                while let Some(char) = chars.next() {
                    match char {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        '{' => println!("LEFT_BRACE {{ null"),
                        '}' => println!("RIGHT_BRACE }} null"),
                        '*' => println!("STAR * null"),
                        '.' => println!("DOT . null"),
                        ',' => println!("COMMA , null"),
                        '+' => println!("PLUS + null"),
                        '-' => println!("MINUS - null"),
                        '/' => {
                            if chars.peek() == Some(&'/') {
                                chars.next(); // Consume the second slash
                                while let Some(c) = chars.next()  {
                                    if c == '\n' {
                                        line += 1;
                                        break; // Stop at the end of the line
                                    }
                                }
                            } else {
                                println!("SLASH / null");
                            }
                        },
                        ';' => println!("SEMICOLON ; null"),
                        '=' => {
                            if chars.peek() == Some(&'=') {
                                chars.next();
                                println!("EQUAL_EQUAL == null");
                            } else {
                                println!("EQUAL = null");
                            }
                        },
                        '!' => {
                            if chars.peek() == Some(&'=') {
                                chars.next();
                                println!("BANG_EQUAL != null");
                            } else {
                                println!("BANG ! null");
                            }
                        },
                        '>' => {
                            if chars.peek() == Some(&'=') {
                                chars.next();
                                println!("GREATER_EQUAL >= null");
                            } else {
                                println!("GREATER > null");
                            }
                        },
                        '<' => {
                            if chars.peek() == Some(&'=') {
                                chars.next();
                                println!("LESS_EQUAL <= null");
                            } else {
                                println!("LESS < null");
                            }
                        },
                        '\t' | ' ' | '\r' => {
                            // Ignore whitespace
                        },
                        '\n' => line += 1,
                        unknow => {
                            eprintln!("[line {}] Error: Unexpected character: {}",line ,unknow);
                            is_error = true;
                        },
                    }
                }
            }
            println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    if is_error {
        exit(65);
    }
    exit(0);
}
