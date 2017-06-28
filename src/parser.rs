use std::collections::{LinkedList, HashMap};
use std::iter::Peekable;
use std::vec::IntoIter;
use std::rc::Rc;

use tokenizer::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
	Str(String),
	Int(i64),
	Real(f64),
	Symbol(String),
	Bool(bool),
	Nil,
	List(LinkedList<Rc<Node>>),
	Map(HashMap<String, Rc<Node>>),
	Expr(LinkedList<Rc<Node>>),
}
pub type ParserResult = Result<Node, String>;

pub struct Parser {
	tokens: Peekable<IntoIter<Token>>
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Parser {
		Parser {
			tokens: tokens.into_iter().peekable()
		}
	}

	fn parse_collection(&mut self) -> ParserResult {
		let mut is_map = false;
		let mut is_list = false;
		let mut list: LinkedList<Rc<Node>> = LinkedList::new();
		let mut map: HashMap<String, Rc<Node>> = HashMap::new();
		if self.tokens.peek() == Some(&Token::Colon) {
			self.tokens.next().unwrap();
			if self.tokens.next() == Some(Token::RightCurlyBracket) {
				return Ok(Node::Map(map));
			} else {
				return Err("illegal collection litteral".to_string());
			}
		} else if self.tokens.peek() == Some(&Token::RightCurlyBracket) {
			self.tokens.next().unwrap();
			return Ok(Node::List(list));
		}

		while self.tokens.peek() != Some(&Token::RightCurlyBracket) {
			if is_list {
				if let Some(res) = self.next() {
					match res {
						Ok(node) => list.push_back(Rc::new(node)),
						Err(e) => { return Err(e); }
					}
				} else { return Err("unexpected end of file".to_string()); }
			} else if is_map {
				let key: String = match self.tokens.next() {
					Some(token) => {
						if let Token::Symbol(symbol) = token {
							symbol
						} else { return Err("illegal map litteral".to_string()); }
					},
					None => { return Err("unexpected end of file".to_string()); }
				};

				if self.tokens.next() != Some(Token::Colon) {
					return Err("illegal map litteral".to_string());
				}

				let value: Node = match self.next() {
					Some(res) => {
						match res {
							Ok(node) => node,
							Err(e) => { return Err(e); }
						}
					},
					None => { return Err("unexpected end of file".to_string()); }
				};

				map.insert(key, Rc::new(value));
			} else {
				let item: Node = match self.next() {
					Some(res) => {
						match res {
							Ok(node) => node,
							Err(e) => { return Err(e); }
						}
					},
					None => { return Err("unexpected end of file".to_string()); }
				};

				if self.tokens.peek() == Some(&Token::Colon) {
					self.tokens.next().unwrap();
					is_map = true;
					let key: String = match item {
						Node::Symbol(symbol) => symbol,
						_ => { return Err("illegal collection litteral".to_string()); }
					};

					let value: Node = match self.next() {
						Some(res) => {
							match res {
								Ok(node) => node,
								Err(e) => { return Err(e); }
							}
						},
						None => { return Err("unexpected end of file".to_string()); }
					};

					map.insert(key, Rc::new(value));
				} else {
					is_list = true;
					list.push_back(Rc::new(item));
				}
			}
		}

		if self.tokens.next().is_none() {
			Err("unexpected end of file".to_string())
		} else if is_map {
			Ok(Node::Map(map))
		} else if is_list {
			Ok(Node::List(list))
		} else {
			panic!("parsing collection failed");
		}
	}

	pub fn collect(&mut self) -> Result<Vec<Node>, String> {
		let mut nodes: Vec<Node> = vec![];
		while let Some(res) = self.next() {
			match res {
				Ok(node) => nodes.push(node),
				Err(e) => { return Err(e); }
			}
		}
		Ok(nodes)
	}
}

impl Iterator for Parser {
	type Item = ParserResult;
	fn next(&mut self) -> Option<ParserResult> {
		if let Some(token) = self.tokens.next() {
			match token {
				Token::Str(string) => Some(Ok(Node::Str(string))),
				Token::Int(number) => Some(Ok(Node::Int(number))),
				Token::Real(number) => Some(Ok(Node::Real(number))),
				Token::Bool(value) => Some(Ok(Node::Bool(value))),
				Token::Symbol(string) => Some(Ok(Node::Symbol(string))),
				Token::Nil => Some(Ok(Node::Nil)),
				Token::LeftBracket => {
					let mut items: LinkedList<Rc<Node>> = LinkedList::new();
					while self.tokens.peek() != Some(&Token::RightBracket) {
						if let Some(res) = self.next() {
							match res {
								Ok(node) => items.push_back(Rc::new(node)),
								Err(e) => { return Some(Err(e)); }
							}
						} else { return Some(Err("unexpected end of file".to_string())); }
					}

					self.tokens.next().expect("parsing expression fail: unexpected end of file");
					Some(Ok(Node::Expr(items)))
				},
				Token::LeftCurlyBracket => Some(self.parse_collection()),
				_ => Some(Err("unexpected token".to_string()))
			}
		} else {
			None
		}
	}
}
