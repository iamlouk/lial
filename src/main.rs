use std::io::Write;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::env;
use std::fs::File;

mod tokenizer;
mod parser;
mod builtins;
mod interpreter;


macro_rules! perror(
	($($arg:tt)*) => { {
		writeln!(&mut ::std::io::stderr(), $($arg)*).expect("failed printing to stderr");
	} }
);

fn process_line(interpreter: &mut interpreter::Interpreter, line: &String) -> Result<Vec<Rc<interpreter::Value>>, String> {
	match tokenizer::Tokenizer::new(&line).collect() {
		Ok(tokens) => {
			match parser::Parser::new(tokens).collect() {
				Ok(nodes) => {
					let nodes: Vec<parser::Node> = nodes;
					let mut values: Vec<Rc<interpreter::Value>> = vec![];
					for node in nodes {
						match interpreter.eval(Rc::new(node)) {
							Ok(value) => values.push(value),
							Err(e) => { return Err(e); }
						}
					}
					return Ok(values);
				},
				Err(e) => Err(e)
			}
		},
		Err(e) => Err(e)
	}
}

fn process_file(filename: String) {
	let mut interpreter = interpreter::Interpreter::new();
	let mut file = match File::open(filename) {
		Ok(file) => file,
		Err(e) => {
			perror!("{:?}", e);
			std::process::exit(1);
		}
	};

	let mut buf = String::new();
	match file.read_to_string(&mut buf) {
		Ok(_) => {},
		Err(e) => {
			perror!("{:?}", e);
			std::process::exit(1);
		}
	};

	match tokenizer::Tokenizer::new(&buf).collect() {
		Ok(tokens) => {
			match parser::Parser::new(tokens).collect() {
				Ok(nodes) => {
					let nodes: Vec<parser::Node> = nodes;
					for node in nodes {
						match interpreter.eval(Rc::new(node)) {
							Ok(_) => {},
							Err(e) => {
								perror!("Error: {:?}", e);
								std::process::exit(1);
							}
						}
					}
				},
				Err(e) => {
					perror!("Error: {:?}", e);
					std::process::exit(1);
				}
			}
		},
		Err(e) => {
			perror!("Error: {:?}", e);
			std::process::exit(1);
		}
	}

}

fn repl() {
	println!("yial: REPL (Ctrl+D to exit)");
	let mut interpreter = interpreter::Interpreter::new();
	loop {
		print!(">_ ");
		io::stdout().flush().unwrap();
		let mut buf = String::new();
		match io::stdin().read_line(&mut buf) {
			Ok(len) => {
				if len == 0 { break; }

				match process_line(&mut interpreter, &buf) {
					Ok(values) => {
						let mut i = 0;
						for value in values {
							perror!("${} = {:?}", i, value);
							i += 1;
						}
					},
					Err(e) => perror!("Error: {:?}", e)
				}
			},
			Err(e) => {
				perror!("Error: {:?}", e);
				std::process::exit(1);
			}
		}
	}
	println!("\nBye!");
}

fn main() {
	if let Some(filename) = env::args().nth(1) {
		process_file(filename);
	} else {
		repl();
	}
}
