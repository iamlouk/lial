use std::collections::{LinkedList, HashMap};
use std::rc::Rc;
use std::collections::linked_list::IntoIter;

use parser::Node;
use builtins;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
	Str(String),
	Int(i64),
	Real(f64),
	Bool(bool),
	Nil,
	List(LinkedList<Rc<Value>>),
	Map(HashMap<String, Rc<Value>>),
	Func(Vec<String>, Vec<Rc<Node>>),
	ExternalFn(fn(Vec<Rc<Value>>) -> EvalResult)
}

impl Value {
	pub fn to_bool(&self) -> bool {
		match self {
			&Value::Bool(value) => value,
			&Value::Nil => false,
			&Value::Int(i) => i != 0,
			&Value::Real(r) => r != 0.0,
			_ => false
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			&Value::Str(ref value) => value.clone(),
			&Value::Int(value) => value.to_string(),
			&Value::Real(value) => value.to_string(),
			&Value::Bool(value) => value.to_string(),
			&Value::Nil => "<Nil>".to_string(),
			&Value::List(ref list) => {
				let mut string = "{ ".to_string();
				for item in list {
					string.push_str(item.to_string().as_str());
					string.push(' ');
				}
				string.push('}');
				string
			},
			&Value::Map(ref map) => {
				if map.len() == 0 {
					return "{:}".to_string();
				}
				let mut string = "{ ".to_string();
				for (key, value) in map {
					string.push_str(key.as_str());
					string.push_str(": ");
					string.push_str(value.to_string().as_str());
					string.push(' ');
				}
				string.push('}');
				string
			},
			&Value::Func(_, _) => "<Fn::Internal>".to_string(),
			&Value::ExternalFn(_) => "<Fn::External>".to_string()
		}
	}
}

pub type EvalResult = Result<Rc<Value>, String>;


pub struct Env {
	size: usize,
	scope: Vec<HashMap<String, Rc<Value>>>
}
impl Env {
	pub fn new() -> Env {
		Env { scope: vec![], size: 0 }
	}

	pub fn exit(&mut self) {
		self.size -= 1;
		self.scope.pop().expect("");
	}

	pub fn enter(&mut self) {
		self.size += 1;
		self.scope.push( HashMap::new() );
	}

	pub fn define(&mut self, key: String, value: Rc<Value>) {
		self.scope[self.size - 1].insert(key, value);
	}

	pub fn define_global(&mut self, key: String, value: Rc<Value>) {
		self.scope[0].insert(key, value);
	}

	pub fn lookup(&self, key: &String) -> Option<Rc<Value>> {
		let mut i: i32 = self.size as i32 - 1;
		while i >= 0 {
			if let Some(value) = self.scope[i as usize].get(key) {
				return Some(value.clone());
			}
			i -= 1;
		}
		None
	}
}

pub struct Interpreter {
	env: Env
}

impl Interpreter {
	pub fn new() -> Interpreter {
		let mut interpreter = Interpreter {
			env: Env::new()
		};
		interpreter.env.enter();
		interpreter.expose_external_func("+", builtins::add);
		interpreter.expose_external_func("echo", builtins::echo);
		interpreter
	}

	pub fn expose_external_func(&mut self, name: &'static str, func: fn(Vec<Rc<Value>>) -> EvalResult) {
		self.env.define(name.to_string(), Rc::new( Value::ExternalFn(func) ));
	}

	pub fn eval(&mut self, node: Rc<Node>) -> EvalResult {
		match *node {
			Node::Str(ref value) => Ok(Rc::new( Value::Str( value.clone() ) )),
			Node::Int(value) => Ok(Rc::new( Value::Int(value) )),
			Node::Real(value) => Ok(Rc::new( Value::Real(value) )),
			Node::Bool(value) => Ok(Rc::new( Value::Bool(value) )),
			Node::Nil => Ok(Rc::new(Value::Nil)),
			Node::List(ref nodes) => {
				let mut list: LinkedList<Rc<Value>> = LinkedList::new();
				for node in nodes {
					match self.eval(node.clone()) {
						Ok(value) => list.push_back(value),
						Err(e) => {
							return Err(e);
						}
					}
				}
				Ok(Rc::new( Value::List(list) ))
			},
			Node::Map(ref nodes) => {
				let mut map: HashMap<String, Rc<Value>> = HashMap::new();
				for (key, node) in nodes {
					match self.eval(node.clone()) {
						Ok(value) => {
							map.insert(key.clone(), value);
						},
						Err(e) => {
							return Err(e);
						}
					}
				}
				Ok(Rc::new( Value::Map(map) ))
			},
			Node::Symbol(ref symbol) => {
				if let Some(value) = self.env.lookup(symbol) {
					Ok(value)
				} else {
					Err("unkown symbol".to_string())
				}
			},
			Node::Expr(ref args) => self.eval_expr(args)
		}
	}

	fn eval_expr(&mut self, args: &LinkedList<Rc<Node>>) -> EvalResult {
		let mut iter: IntoIter<Rc<Node>> = args.clone().into_iter();
		if let Some(op) = iter.next() {
			match *(op.clone()) {
				Node::Symbol(ref symname) => {
					match symname.as_str() {
						"fn" => self.eval_fn(iter),
						"def" => self.eval_def(iter),
						"if" => self.eval_if(iter),
						"and" => self.eval_and(iter),
						"or" => self.eval_or(iter),
						_ => {
							if let Some(value) = self.env.lookup(symname) {
								self.eval_value(value, iter)
							} else {
								Err("unkown symbol".to_string())
							}
						}
					}
				},
				_ => {
					match self.eval(op) {
						Ok(value) => self.eval_value(value, iter),
						Err(e) => Err(e)
					}
				}
			}
		} else {
			Err("cannot evaluate empty expression".to_string())
		}
	}

	fn eval_value(&mut self, value: Rc<Value>, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		match *value {
			Value::ExternalFn(func) => {
				let mut args: Vec<Rc<Value>> = vec![];
				while let Some(node) = iter.next() {
					match self.eval(node) {
						Ok(value) => args.push(value),
						Err(e) => {
							return Err(e);
						}
					}
				}
				func(args)
			},
			Value::Func(ref argnames, ref nodes) => {
				self.env.enter();
				let mut i = 0;
				while let Some(argnode) = iter.next() {
					match self.eval(argnode) {
						Ok(value) => self.env.define(argnames[i].clone(), value),
						Err(e) => { return Err(e); }
					}
					i += 1;
				}

				let mut return_value: Rc<Value> = Rc::new( Value::Nil );
				for node in nodes {
					return_value = match self.eval(node.clone()) {
						Ok(value) => value,
						Err(e) => { return Err(e); }
					}
				}
				self.env.exit();
				Ok(return_value)
			},
			_ => Err("cannot evaluate expression".to_string())
		}
	}

	fn eval_fn(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		let mut args: Vec<String> = vec![];
		if let Some(node) = iter.next() {
			match *node {
				Node::List(ref list) => {
					for node in list {
						match *node.clone() {
							Node::Symbol(ref sym) => { args.push(sym.clone()); },
							_ => { return Err("illegal fn syntax".to_string()); }
						}
					}
				},
				_ => { return Err("illegal fn syntax".to_string()); }
			}
		} else { return Err("illegal fn syntax".to_string()); }
		Ok( Rc::new( Value::Func(args, iter.collect()) ) )
	}

	fn eval_def(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		let key: String;
		if let Some(node) = iter.next() {
			match *node {
				Node::Symbol(ref sym) => { key = sym.clone(); },
				_ => { return Err("illegal def syntax".to_string()); }
			}
		} else { return Err("illegal def syntax".to_string()); }

		let value: Rc<Value>;
		if let Some(node) = iter.next() {
			match self.eval(node) {
				Ok(val) => { value = val; },
				Err(e) => { return Err(e); }
			}
		} else { return Err("illegal def syntax".to_string()); }

		if iter.count() != 0 {
			return Err("illegal def syntax".to_string());
		}

		self.env.define_global(key, value.clone());
		Ok(value)
	}

	fn eval_if(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		let cond: bool;
		if let Some(node) = iter.next() {
			match self.eval(node) {
				Ok(value) => { cond = value.to_bool(); },
				Err(e) => { return Err(e); }
			}
		} else { return Err("illegal if syntax".to_string()); }

		if let Some(if_true) = iter.next() {
			if cond {
				self.eval(if_true)
			} else if let Some(if_false) = iter.next() {
				self.eval(if_false)
			} else {
				Ok(Rc::new(Value::Nil))
			}
		} else {
			Err("illegal if syntax".to_string())
		}
	}

	fn eval_and(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		while let Some(node) = iter.next() {
			match self.eval(node) {
				Ok(value) => {
					if !value.to_bool() {
						return Ok(Rc::new(Value::Bool(false)));
					}
				},
				Err(e) => { return Err(e); }
			}
		}
		Ok(Rc::new(Value::Bool(true)))
	}

	fn eval_or(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		while let Some(node) = iter.next() {
			match self.eval(node) {
				Ok(value) => {
					if value.to_bool() {
						return Ok(Rc::new(Value::Bool(true)));
					}
				},
				Err(e) => { return Err(e); }
			}
		}
		Ok(Rc::new(Value::Bool(false)))
	}

}
