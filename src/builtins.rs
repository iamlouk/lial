use std::rc::Rc;
use interpreter::{Value, EvalResult};

pub fn add(args: Vec<Rc<Value>>) -> EvalResult {
	let mut res = Value::Int(0);
	for arg in args {
		res = match (res, arg.as_ref()) {
			(Value::Int(a), &Value::Int(b)) => Value::Int(a + b),
			(Value::Real(a), &Value::Int(b)) => Value::Real(a + (b as f64)),
			(Value::Int(a), &Value::Real(b)) => Value::Real((a as f64) + b),
			(Value::Real(a), &Value::Real(b)) => Value::Real(a + b),
			_ => {
				return Err("`+` takes arguments of type int or real".to_string());
			}
		};
	}
	Ok(Rc::new(res))
}

pub fn mul(args: Vec<Rc<Value>>) -> EvalResult {
	let mut res = Value::Int(1);
	for arg in args {
		res = match (res, arg.as_ref()) {
			(Value::Int(a), &Value::Int(b)) => Value::Int(a * b),
			(Value::Real(a), &Value::Int(b)) => Value::Real(a * (b as f64)),
			(Value::Int(a), &Value::Real(b)) => Value::Real((a as f64) * b),
			(Value::Real(a), &Value::Real(b)) => Value::Real(a * b),
			_ => {
				return Err("`*` takes arguments of type int or real".to_string());
			}
		};
	}
	Ok(Rc::new(res))
}

pub fn sub(args: Vec<Rc<Value>>) -> EvalResult {
	if args.len() == 1 {
		return match args[0].as_ref() {
			&Value::Int(i) => Ok(Rc::new(Value::Int(-i))),
			&Value::Real(r) => Ok(Rc::new(Value::Real(-r))),
			_ => { return Err("`-` takes arguments of type int or real".to_string()); }
		};
	} else if args.len() > 1 {
		let mut iter = args.into_iter();
		let mut res = match iter.next().unwrap().as_ref() {
			&Value::Int(i) => Value::Int(i),
			&Value::Real(r) => Value::Real(r),
			_ => { return Err("`-` takes arguments of type int or real".to_string()); }
		};
		while let Some(arg) = iter.next() {
			res = match (res, arg.as_ref()) {
				(Value::Int(a), &Value::Int(b)) => Value::Int(a - b),
				(Value::Real(a), &Value::Int(b)) => Value::Real(a - (b as f64)),
				(Value::Int(a), &Value::Real(b)) => Value::Real((a as f64) - b),
				(Value::Real(a), &Value::Real(b)) => Value::Real(a - b),
				_ => {
					return Err("`-` takes arguments of type int or real".to_string());
				}
			};
		}
		return Ok(Rc::new(res));
	} else {
		return Err("`-` takes min. one argument of type int or real".to_string());
	}
}

pub fn equals(args: Vec<Rc<Value>>) -> EvalResult {
	let mut iter = args.iter();
	if let Some(arg0) = iter.next() {
		while let Some(arg) = iter.next() {
			if !(arg0.as_ref() == arg.as_ref()) {
				return Ok(Rc::new(Value::Bool(false)));
			}
		}
		Ok(Rc::new(Value::Bool(true)))
	} else {
		Err("`=` takes min. one argument".to_string())
	}
}

pub fn echo(args: Vec<Rc<Value>>) -> EvalResult {
	for arg in args {
		print!("{}", arg.to_string());
	}
	println!();
	Ok(Rc::new(Value::Nil))
}

pub fn hex(args: Vec<Rc<Value>>) -> EvalResult {
	if args.len() != 1 {
		return Err("hex takes only one argument".to_string());
	}

	match *args[0] {
		Value::Int(i) => Ok(Rc::new( Value::Str( format!("{:X}", i) ) )),
		_ => Err("hex only takes int as argument".to_string())
	}
}
