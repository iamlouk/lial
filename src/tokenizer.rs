use std::str::Chars;
use std::iter::Peekable;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
	LeftBracket,
	RightBracket,
	LeftCurlyBracket,
	RightCurlyBracket,
	Colon,

	Str(String),
	Int(i64),
	Real(f64),
	Bool(bool),
	Symbol(String),
	Nil
}

pub type TokenizerResult = Result<Token, String>;

pub struct Tokenizer<'a> {
	chars: Peekable<Chars<'a>>,
	line: i32,
	reserved: HashMap<String, Token>
}

fn is_symbolchar(c: char) -> bool {
	return !(c == '(' || c == ')' || c == '{' || c == '}' || c == ';' || c == ':') && (c >= '!' && c <= '~');
}

impl<'a> Tokenizer<'a> {
	pub fn new<'b>(string: &'b String) -> Tokenizer<'b> {
		let mut reserved: HashMap<String, Token> = HashMap::new();
		reserved.insert("true".to_string(), Token::Bool(true));
		reserved.insert("false".to_string(), Token::Bool(false));
		reserved.insert("nil".to_string(), Token::Nil);

		Tokenizer {
			chars: string.chars().peekable(),
			line: 0,
			reserved: reserved
		}
	}

	pub fn collect(&mut self) -> Result<Vec<Token>, String> {
		let mut tokens: Vec<Token> = vec![];
		while let Some(res) = self.next() {
			match res {
				Ok(token) => tokens.push(token),
				Err(e) => { return Err(e); }
			}
		}
		Ok(tokens)
	}

	fn read_int(&mut self, base: u32) -> TokenizerResult {
		self.chars.next().unwrap(); // consume 'x', 'o' or 'b'

		let mut buf: Vec<u32> = vec![];
		loop {
			if let Some(peek) = self.chars.peek() {
				if !peek.is_digit(base) {
					break;
				}
			} else {
				break;
			}
			buf.push( self.chars.next().unwrap().to_digit(base).unwrap() );
		}

		if buf.len() == 0 {
			return Err("illegal number litteral".to_string());
		}

		let mut num: i64 = 0;
		let mut i: u32 = 1;
		while i <= (buf.len() as u32) {
			let digit = buf[ buf.len() - (i as usize) ];
			num += (digit * base.pow(i - 1)) as i64;
			i += 1;
		}
		Ok(Token::Int(num))
	}
}

impl<'a> Iterator for Tokenizer<'a> {
	type Item = TokenizerResult;
	fn next(&mut self) -> Option<TokenizerResult> {
		if let Some(ch) = self.chars.next() {
			match ch {
				';' => {
					let mut c = self.chars.next();
					while c.is_some() && c != Some('\n') {
						c = self.chars.next();
					}
					self.line += 1;
					self.next()
				},
				'\n' => {
					self.line += 1;
					self.next()
				},
				'(' => Some(Ok(Token::LeftBracket)),
				')' => Some(Ok(Token::RightBracket)),
				'{' => Some(Ok(Token::LeftCurlyBracket)),
				'}' => Some(Ok(Token::RightCurlyBracket)),
				':' => Some(Ok(Token::Colon)),
				'"' => {
					let mut buf: String = String::new();
					loop {
						if let Some(c) = self.chars.next() {
							match c {
								'"' => {
									break;
								},
								'\\' => {
									let escaped = self.chars.next();
									if escaped.is_none() {
										return Some(Err("unexpected end of string".to_string()));
									}
									match escaped.unwrap() {
										'n' => buf.push('\n'),
										't' => buf.push('\t'),
										'\\' => buf.push('\\'),
										'\"' => buf.push('\"'),
										_ => {
											return Some(Err("unescapeable character in string".to_string()));
										}
									}
								},
								_ => buf.push(c)
							}
						} else {
							return Some(Err("unexpected end of string".to_string()));
						}
					}
					Some(Ok(Token::Str(buf)))
				},
				'0' => {
					let c: char = match self.chars.peek() {
						Some(c) => *c,
						None => {
							return Some(Err("unexpected end of number litteral".to_string()));
						}
					};

					return match c {
						'.' => {
							self.chars.next().unwrap();
							let mut buf: String = "0.".to_string();
							loop {
								if let Some(peek) = self.chars.peek() {
									if !peek.is_digit(10) {
										break;
									}
								} else {
									break;
								}

								buf.push( self.chars.next().unwrap() );
							}
							return match buf.parse::<f64>() {
								Ok(number) => Some(Ok(Token::Real(number))),
								Err(_) => Some(Err("illegal number litteral".to_string()))
							};
						},
						'b' => Some(self.read_int(2)),
						'o' => Some(self.read_int(8)),
						'x' => Some(self.read_int(16)),
						_ => Some(Ok(Token::Int(0))) // Some(Err("illegal number litteral".to_string()))
					};
				},
				'1'...'9' => {
					let mut is_real = false;
					let mut buf: String = ch.to_string();
					loop {
						if let Some(peek) = self.chars.peek() {
							if *peek == '.' && is_real {
								return Some(Err("illegal number litteral".to_string()));
							} else if *peek == '.' {
								is_real = true;
							} else if !peek.is_digit(10) {
								break;
							}
						} else {
							break;
						}

						buf.push( self.chars.next().unwrap() );
					}

					if is_real {
						return match buf.parse::<f64>() {
							Ok(number) => Some(Ok(Token::Real(number))),
							Err(_) => Some(Err("illegal number litteral".to_string()))
						};
					} else {
						return match buf.parse::<i64>() {
							Ok(number) => Some(Ok(Token::Int(number))),
							Err(_) => Some(Err("illegal number litteral".to_string()))
						};
					}
				},
				_ => {
					if ch.is_whitespace() {
						return self.next();
					}

					if !is_symbolchar(ch) {
						return Some(Err("illegal character".to_string()))
					}

					let mut buf: String = ch.to_string();
					loop {
						if let Some(peek) = self.chars.peek() {
							if !is_symbolchar(*peek) {
								break;
							}
						} else {
							break;
						}

						buf.push( self.chars.next().unwrap() );
					}

					if let Some(token) = self.reserved.get(&buf) {
						Some(Ok(token.clone()))
					} else {
						Some(Ok(Token::Symbol(buf)))
					}
				}
			}
		} else {
			None
		}
	}
}
