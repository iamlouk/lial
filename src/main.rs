use std::io::Write;
use std::io;
use std::rc::Rc;

mod tokenizer;
mod parser;
mod builtins;
mod interpreter;

fn process(interpreter: &mut interpreter::Interpreter, line: &String) -> Result<Vec<Rc<interpreter::Value>>, String> {
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

fn main() {
    println!("yial: REPL (Ctrl+D to exit)");
	let mut interpreter = interpreter::Interpreter::new();
	loop {
		print!(">_ ");
		io::stdout().flush().unwrap();
		let mut buf = String::new();
		match io::stdin().read_line(&mut buf) {
			Ok(len) => {
				if len == 0 { break; }

				match process(&mut interpreter, &buf) {
					Ok(values) => {
						let mut i = 0;
						for value in values {
							println!("${} = {:?}", i, value);
							i += 1;
						}
					},
					Err(e) => println!("Error: {:?}", e)
				}
			},
			Err(e) => {
				println!("{:?}", e);
				break;
			}
		}
	}
	println!("\nBye!");
}
