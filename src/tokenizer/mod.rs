/* Set of acceptible language tokens */
use std::collections::HashSet;
use std::ops::Deref;
pub struct Tokenizer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
	Keyword(String), 		/* handles keywords, variables, and identifiers */
	Printable(String), 		/* Contains the printable output to stdout */
	Assigned(String),		/* Assigned value (right half of expression) */
	Whitespace(String),
	GotoLabel(String),
	FunctionCall(String),   /* Represents a function call */
	FunctionCallWithParams(String), /* function call w params */
	FunctionName(String),	/* Represents a function name */
	NewLine,				/* New line for parsing purposes */
	Assignment(String),		/* := keyword */
	Constant(i64), 			/* Use 64-bit for constant integers */
	Equivalent,				/* <> operator */
	LessThanEqual,			/* <= operator */
	GreaterThanEqual,		/* >= operator */
	LessThan, 				/* < operator */
	GreaterThan,			/* > operator */
	Equal,					/* = operator */
	And,					/* & operator */
	Or,						/* | operator */
	Not,					/* ! operator */
	Plus,					/* + operator */
	Multiply,				/* * operator */
	Minus,					/* - operator */
	IntDiv,					/* / operator */
	RemainDiv(String),		/* div operator */
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recognize {
	token: Option<Token>,
	to_match: bool,
	collection: Vec<String>,
}

impl Tokenizer {
	pub fn reduce_tokens_to_enum(raw: &mut Recognize, result: &mut Vec<Token>) {
		if raw.to_match {
			raw.to_match = false;

			let a = raw.collection.iter().map(|a| a.to_string()).collect::<String>();
			let token = raw.token.clone();

			if a != "" {
				match token.unwrap() {
					Token::Printable(ref c) => {
						let mut b = a.clone();
						b.push_str(&" ".to_string());
						result.push(Token::Printable(b.clone()));
					}
					Token::FunctionCall(ref c) => {
						result.push(Token::FunctionCall(a.clone()));
					}
					Token::FunctionCallWithParams(ref c) => {
						result.push(Token::FunctionCallWithParams(a.clone()));
					}
					Token::FunctionName(ref c) => {
						result.push(Token::FunctionName(a.clone()));
					}
					Token::Assignment(ref c) => {
						result.push(Token::Assignment(a.clone()));
					}
					Token::GotoLabel(ref c) => {
						result.push(Token::GotoLabel(a.clone()));
					}
					_ => {/* Sink */}
				}
			}
			raw.collection.clear()					
		}
	}

	fn build_recognizer(
		final_str: &str, 
		old_token: Option<Token>, 
		can_update_recognizer: &mut bool
	) -> Recognize {
		let token = match final_str { 
			"show" => {
				Some(Token::Printable(String::new()))
			}
			"lvalue" => {
				Some(Token::Assignment(String::new()))
			}
			"rvalue" => {
				Some(Token::Assignment(String::new()))
			}
			"push" => {
				Some(Token::Assignment(String::new()))
			}
			"label" => {
				Some(Token::FunctionName(String::new()))
			}
			"call" => {
				Some(Token::FunctionCallWithParams(String::new()))
			}
			"goto" => {
				Some(Token::FunctionCall(String::new()))
			}
			"gofalse" => {
				Some(Token::GotoLabel(String::new()))
			}
			_ => None,
		};
		match token.clone() {
			Some(new) => {
				return Recognize { 
					token: token, 
					to_match: true,
					collection: Vec::new()
				};
			}
			None => {
				return Recognize {
					token: None,
					to_match: false,
					collection: Vec::new()
				}
			}
		}
	}

	pub fn lex(input: &String) -> Result<Vec<Token>, String>
	{
		/* This allows us to recognize any token, and collect its subsequent values */
		let mut recognizer = Recognize { 
			token: None, 
			to_match: false,
			collection: Vec::new()
		};

		/* Populate HashSet with allowable keywords */
		let grammar: HashSet<&str> = [
			"push", "rvalue", "lvalue", "pop",
			":=", "copy", "label", "goto", "gofalse",
			"gotrue", "halt", "+", "-", "/", "div",
			"&", "!", "|", "<>", "<=", ">=", "<", ">",
			"=", "print", "show", "begin", "end",
			"return", "call",
		].iter().cloned().collect();

		let mut result: Vec<Token> = Vec::new();
		let mut iterator = input.chars().peekable();
		let mut token_buf: Vec<String> = Vec::new();

		/* If we can peek, we peek */
		let mut is_printable: bool = false;
		while let Some(&raw) = iterator.peek() {
			match raw {
				'a' ... 'z' => { /* Match a-z characters */

					while iterator.peek().unwrap().is_alphabetic() {
						let val = iterator.next().unwrap().to_string();

						if recognizer.to_match {
							recognizer.collection.push(val.to_string());
						}

						token_buf.push(val);
					}

					let final_str: String = token_buf.join("");
					/* Empty out token buffer */
					
					if !is_printable {
						if grammar.contains(final_str.deref()) {
							result.push(Token::Keyword(final_str.clone()));

							let old_token = recognizer.token;
							recognizer = Tokenizer::build_recognizer(
								&*final_str,
								old_token.clone(),
								&mut true
							);
							if let Some(Token::Printable(a)) = recognizer.token.clone() {
								is_printable = true;
							}							
						}
					}
					token_buf.clear();
				}

				'\n' => {
					is_printable = false;
					Tokenizer::reduce_tokens_to_enum(&mut recognizer, &mut result);
					recognizer.token = None;
					iterator.next();
				}

				'\r' => {
					// println!("HIT...");
					let val = iterator.next().unwrap();
					
					match recognizer.token {
						Some(Token::Printable(ref c)) => {
							if val != '\r' || val != '\n' {
								recognizer.collection.push(val.to_string());
							}						
						}
						Some(Token::Assignment(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::FunctionCall(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::FunctionCallWithParams(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}					
						}
						Some(Token::FunctionName(ref c)) => {
							if val != ' ' { /* Can further validate syntax here */
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::GotoLabel(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						None => { /* Recognizer did not handle Token type */ }
						_ => {}
					}
				}

				'\n' => {
					let val = iterator.next().unwrap();
					match recognizer.token {
						Some(Token::Printable(ref c)) => {
							if val != '\r' || val != '\n' {
								recognizer.collection.push(val.to_string());
							}					
						}
						Some(Token::Assignment(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::FunctionCall(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::FunctionCallWithParams(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}					
						}
						Some(Token::FunctionName(ref c)) => {
							if val != ' ' { 
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::GotoLabel(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						None => { /* Recognizer did not handle Token type */  }
						_ => {}
					}
				}

				' ' => {
					let val = iterator.next().unwrap();
					match recognizer.token {
						Some(Token::Printable(ref c)) => {
							if val != '\r' || val != '\n' {
								recognizer.collection.push(val.to_string());
							}					
						}
						Some(Token::Assignment(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::FunctionCall(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::FunctionCallWithParams(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}					
						}
						Some(Token::FunctionName(ref c)) => {
							if val != ' ' { 
								recognizer.collection.push(val.to_string());
							}
						}
						Some(Token::GotoLabel(ref c)) => {
							if val != ' ' {
								recognizer.collection.push(val.to_string());
							}
						}
						None => { /* Recognizer did not handle Token type */ }
						_ => {}
					}
				}
				'+' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {}
						}
					} else {
						iterator.next();
						result.push(Token::Plus);
					}
				}
				'-' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();
						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {
								result.push(Token::Minus);
							}
						}
					} else {
						/* Otherwise match the operators token value */
						match iterator.clone().peek() {
							Some(peeked) => {
								if *peeked != '-' {
									println!("Not Peeked...");
									result.push(Token::Minus);
									iterator.next();
								} else {
									iterator.next();
								}
							}
							None => {
								println!("Peeked...");
							}
						}
					}
				}
				'/' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();
						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {
								result.push(Token::IntDiv);
							}
						}
					} else {
						iterator.next();
						result.push(Token::IntDiv);
					}
				}
				'&' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {
								result.push(Token::And);
							}
						}
					} else {
						iterator.next();
						result.push(Token::And);
					}
				}
				'!' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {
								result.push(Token::Not);
							}
						}
					} else {
						iterator.next();
						result.push(Token::Not);
					}
				}
				'*' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {
								result.push(Token::Multiply);
							}
						}
					} else {
						iterator.next();
						result.push(Token::Multiply);
					}
				}
				'|' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {
								result.push(Token::Or);
							}
						}
					} else {
						iterator.next();
						result.push(Token::Or);
					}
				}
				'<' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {}
						}
					} else {
						match iterator.clone().peek() {
							Some(peeked) => {
								if *peeked == '>' {
									result.push(Token::Equivalent);
									iterator.next();
								} else if *peeked == '=' {
									result.push(Token::LessThanEqual);
									iterator.next();
								} else {
									result.push(Token::LessThan);
									iterator.next();
								}
							}
							None => {
								println!("Couldnt peek.");
							}
						}
					}
				}
				'>' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();
						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {}
						}
					} else {
						match iterator.clone().peek() {
							Some(peeked) => {
								if *peeked == '=' {
									result.push(Token::GreaterThanEqual);
									iterator.next();
								} else {
									result.push(Token::GreaterThan);
									iterator.next();
								}
							}

							None => {
								println!("Couldnt Peek.");
							}
						}
					}
				}
				'=' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();
						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								if val != '\r' || val != '\n' {
									recognizer.collection.push(val.to_string());
								}
							}
							_ => {}
						}
					} else {
						result.push(Token::Equal);
						iterator.next();
					}
				}
				':' => {
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								recognizer.collection.push(val.to_string());
							}
							_ => {}
						}
					} else {
						let a = iterator.next().unwrap().to_string();
						if *iterator.peek().unwrap() == '=' {
							let b = iterator.next().unwrap().to_string();
							result.push(Token::Assignment([a, b].join("")));
						}
					}
				}
				'0' ... '9' => { /* Match 0-9 characters */
					if let Some(Token::Printable(ref apple)) = recognizer.token {
						/* If we are matching printable tokens, ignore operators */
						let val = iterator.next().unwrap();

						match recognizer.token {
							Some(Token::Printable(ref c)) => {
								// can_update_recognizer = false;
								recognizer.collection.push(val.to_string());
							}
							_ => {}
						}
					} else {
						iterator.next(); /* Consume Next Character */

						let mut num = raw.to_string().parse::<i64>().expect("");

						while let Ok(digit) = iterator.peek().map(|c| c.to_string().parse::<i64>()).unwrap() {
							num = num * 10 + digit;
							iterator.next();
						}

						result.push(Token::Constant(num));
					}
				},
				/* Handle all other cases, debug */
				_ => {

					let val = iterator.next().unwrap();

					match recognizer.token {
						Some(Token::Printable(ref c)) => {
							// can_update_recognizer = false;
							if val != '\r' || val != '\n' {
								recognizer.collection.push(val.to_string());
							}
						}
						_ => {}
					}
				}
			}
		}
		Ok(result)
	}
}