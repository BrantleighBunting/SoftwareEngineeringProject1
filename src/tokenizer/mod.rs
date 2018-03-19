/* Set of acceptible language tokens */
use std::collections::HashSet;

use std::iter::Peekable;
use std::ops::Deref;

pub struct Tokenizer;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
	Keyword(String), 		/* handles keywords, variables, and identifiers */
	Printable(String), 		/* Contains the printable output to stdout */
	Assigned(String),		/* Assigned value (right half of expression) */
	Whitespace(String),

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
	Minus,					/* - operator */
	IntDiv,					/* / operator */
	RemainDiv(String),		/* div operator */
}

pub enum Expression {
	Literal(Token),
	Identifier(String),
	Show(String), /* Can either be "" or "<some text here>" */
	Assignment(),
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

			//println!("Tokens Collected: {}", raw.collection.iter().map(|a| a.to_string()).collect::<String>());

			let a = raw.collection.iter().map(|a| a.to_string()).collect::<String>();
			let token = raw.token.clone();

			if a != "" {
				match token.unwrap() {
					Token::Printable(ref c) => {
						result.push(Token::Printable(a.clone()));
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
				println!("Matched an assignment...");
				Some(Token::Assignment(String::new()))
			}
			"rvalue" => {
				println!("Matched an assignment...");
				Some(Token::Assignment(String::new()))
			}
			"push" => {
				println!("Matched an assignment...");
				Some(Token::Assignment(String::new()))
			}
			"label" => {
				println!("Matched a function name...");
				Some(Token::FunctionName(String::new()))
			}
			"call" => {
				println!("Matched a function call with params...");
				Some(Token::FunctionCallWithParams(String::new()))
			}
			"goto" => {
				println!("Matched a function call...");
				Some(Token::FunctionCall(String::new()))
			}
			_ => None,
		};


		match token.clone() {
			Some(new) => {

				// match old_token.clone() {
				// 	Some(old) => {
				// 		if let Token::Printable(pval) = old {
				// 			if let Token::FunctionCallWithParams(cval) = new {
				// 				if *can_update_recognizer {
				// 					*can_update_recognizer = false;
				// 					return Recognize { 
				// 						token: old_token, 
				// 						to_match: true,
				// 						collection: Vec::new()
				// 					};
				// 				}
				// 				/* Early return, handle the 'call' in a printed string */
								
				// 			}
				// 		}
				// 	}
				// 	None => {}
				// }


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


		// let FunctionDescriptorTree = Node::new("main");

		let mut token_buf: Vec<String> = Vec::new();

		/* If we can peek, we peek */
		let mut is_printable: bool = false;
		while let Some(&raw) = iterator.peek() {
			print!("{:?} ", raw);
			
			
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
					println!("Final String: {}", final_str);
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


							println!("Old Token: {:?}", old_token);
							println!("New Token: {:?}", recognizer.token);

							if let Some(Token::Printable(a)) = recognizer.token.clone() {
								is_printable = true;
							}
							// if !can_update_recognizer {
							// 	recognizer.token = old_token;
							// }
							
						}
					}
					token_buf.clear();
				}

				'\n' => {
					is_printable = false;
					println!("New Line Hit...");
					
					//result.push(Token::NewLine);

					Tokenizer::reduce_tokens_to_enum(&mut recognizer, &mut result);

					iterator.next();
				}

				' ' => {
					
					let val = iterator.next().unwrap();
					
					match recognizer.token {
						Some(Token::Printable(ref c)) => {
							
							recognizer.collection.push(val.to_string());						
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
						
						None => { println!("Recognizer wasnt initialized properly."); }
						_ => {}
					}
				}

				':' => {
					let a = iterator.next().unwrap().to_string();
					if *iterator.peek().unwrap() == '=' {
						let b = iterator.next().unwrap().to_string();
						result.push(Token::Assignment([a, b].join("")));
					}
				}
				'0' ... '9' => { /* Match 0-9 characters */

					iterator.next(); /* Consume Next Character */

					let mut num = raw.to_string().parse::<i64>().expect("");

					while let Ok(digit) = iterator.peek().map(|c| c.to_string().parse::<i64>()).unwrap() {
						num = num * 10 + digit;
						iterator.next();
					}

					result.push(Token::Constant(num));
				},
				/* Handle all other cases, debug */
				_ => {

					let val = iterator.next().unwrap();

					match recognizer.token {
						Some(Token::Printable(ref c)) => {
							// can_update_recognizer = false;
							recognizer.collection.push(val.to_string());
						}
						_ => {}
					}
				}
			}
		}
		Ok(result)
	}
}