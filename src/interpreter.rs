use std::collections::{LinkedList, HashMap};
use std::rc::Rc;
use std::fmt::{Debug, Formatter};
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
		let mut i: usize = self.size - 1;
		while i >= 0 {
			if let Some(value) = self.scope[i].get(key) {
				return Some(value.clone());
			}
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

				let mut returnValue: Rc<Value> = Rc::new( Value::Nil );
				for node in nodes {
					returnValue = match self.eval(node.clone()) {
						Ok(value) => value,
						Err(e) => { return Err(e); }
					}
				}
				Ok(returnValue)
			},
			_ => Err("cannot evaluate expression".to_string())
		}
	}

	fn eval_fn(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		unimplemented!();
	}

	fn eval_def(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		unimplemented!();
	}

	fn eval_if(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		unimplemented!();
	}

	fn eval_and(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		unimplemented!();
	}

	fn eval_or(&mut self, mut iter: IntoIter<Rc<Node>>) -> EvalResult {
		unimplemented!();
	}

}
