use std::rc::Rc;
use interpreter::{Value, EvalResult};

pub fn add(args: Vec<Rc<Value>>) -> EvalResult {
	let mut sum = Value::Int(0);
	for arg in args {
		sum = match (sum, arg.as_ref()) {
			(Value::Int(a), &Value::Int(b)) => Value::Int(a + b),
			(Value::Real(a), &Value::Int(b)) => Value::Real(a + (b as f64)),
			(Value::Int(a), &Value::Real(b)) => Value::Real((a as f64) + b),
			(Value::Real(a), &Value::Real(b)) => Value::Real(a + b),
			_ => {
				return Err("cannot add non-numbers".to_string());
			}
		};
	}
	Ok(Rc::new(sum))
}

pub fn echo(args: Vec<Rc<Value>>) -> EvalResult {
	for arg in args {
		print!("{}", arg.to_string());
	}
	println!();
	Ok(Rc::new(Value::Nil))
}
