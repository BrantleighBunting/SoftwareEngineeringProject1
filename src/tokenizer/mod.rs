/* Set of acceptible language tokens */
use std::collections::HashSet;

use std::iter::Peekable;
use std::ops::Deref;

pub struct Tokenizer;

#[derive(Debug, PartialEq)]
pub enum Token {
	Keyword(String), 		/* handles keywords, variables, and identifiers */
	Whitespace(String),
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

impl Tokenizer {
	pub fn lex(input: &String) -> Result<Vec<Token>, String>
	{
		/* This will let us collect tokens as part of 'show' statements */
		let mut collect_printable_tokens: bool = false;
		let mut printable: Vec<String> = Vec::new();


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
						if collect_printable_tokens {
							printable.push(val.to_string());
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
								println!("Printable Statement Found...");
								collect_printable_tokens = true;
							},
							_ => ()
						}
					}
					token_buf.clear();
				}
				'\n' => {
					println!("New Line Hit...");
					collect_printable_tokens = false;
					println!("Printable Tokens Collected: {}", printable.join(""));
					printable.clear();

					iterator.next();
				}

				' ' => {
					let val = iterator.next().unwrap();
					if collect_printable_tokens {
						printable.push(val.to_string());
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
					if collect_printable_tokens {
						printable.push(val.to_string());
					}

					//return Err(format!("unexpected character {}", raw))
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