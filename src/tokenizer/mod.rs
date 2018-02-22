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

			println!("Tokens Collected: {}", raw.collection.iter().map(|a| a.to_string()).collect::<String>());

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
					Token::Assignment(ref c) => {
						result.push(Token::Assignment(a.clone()));
					}
					_ => {/* Sink */}
				}
			}
			raw.collection.clear()					
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
		while let Some(&raw) = iterator.peek() {
			print!("{:?} ", raw);

			// if collect_printable_tokens {
			// 	printable.push(raw.to_string());
			// }
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

					// println!("Parsed String: {}", final_str);
					/* Empty out token buffer */


					if grammar.contains(final_str.deref()) {
						println!("Grammar Keyword Found: {:?}", final_str);

						result.push(Token::Keyword(final_str.clone()));

						match &*final_str { /* String -> &str for comparisons */
							"show" => {
								recognizer = Recognize {
									token: Some(Token::Printable(String::new())),
									to_match: true,
									collection: Vec::new()
								}

							}
							"print" => {
								println!("Matched a print statement...");
							}
							"lvalue" | "rvalue" | "push" => {
								println!("Matched an assignment...");
								recognizer = Recognize {
									token: Some(Token::Assignment(String::new())),
									to_match: true,
									collection: Vec::new()
								}

							}

							"goto" => {
								println!("Matched a function call...");
								recognizer = Recognize {
									token: Some(Token::FunctionCall(String::new())),
									to_match: true,
									collection: Vec::new()
								}
							}
							_ => ()
						}
					}
					token_buf.clear();
				}

				'\n' => {
					println!("New Line Hit...");

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
						None => {

						}

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

					result.push(
						Token::Constant(
							Tokenizer::handle_number(raw, &mut iterator)
						)
					);
				},
				/* Handle all other cases, debug */
				_ => {
					let val = iterator.next().unwrap();

					match recognizer.token {
						Some(Token::Printable(ref c)) => {
							recognizer.collection.push(val.to_string());
						}
						_ => {}
					}
				}
			}
		}
		Ok(result)
	}


	pub fn handle_number<T: Iterator<Item = char>> (raw: char, iterator: &mut Peekable<T>) -> i64
	{
		let mut num = raw.to_string().parse::<i64>().expect("Should have been a digit");

		while let Some(Ok(digit)) = iterator.peek().map(|c| c.to_string().parse::<i64>()) {
			num = num * 10 + digit; /* Apply binary operations here to make more efficient */
			iterator.next();
		}
		num
	}

}