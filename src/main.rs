// #![feature(type_ascription)]
#![allow(unused_variables)]
/* Command Line Args Imports */
use std::env;
use std::process;

/* File I/O Imports */
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::iter::Peekable;

/* Set of acceptible language tokens */
use std::collections::HashSet;
use std::borrow::Borrow;
use std::ops::Deref;

/* Language Specification (modify to be BNF or EBNF format) 
 * ---Stack Manipulation---
 * push c | pushes c onto the stack
 * rvalue l | pushes contents of data location l onto stack
 * lvalue l | pushes address of data location l onto stack
 * pop | throws away value on top of stack
 * := | stack top is placed by the lvalue below it and both are popped
 * copy | pushes a copy of the top value on stack
 *
 *---Control Flow---
 * label l | targets of jumps to l
 * goto l | next instruction is taken from statement with label l
 * gofalse l | pops the top value of the stack and jumps if zero
 * gotrue l | pops the top value of stack and jumps if non-zero
 * halt | stops execution
 *
 *---Arithmetic Operators---
 * + | adds top two values on stack and places results on stack
 * -, * do what they imply
 * / | integer division
 * div | remainder division (modulo) is performed
 *
 *---Logical Operators---
 * & | Logical AND, top two values on stack and places result on stack
 * ! | Negates the top of the stack
 * | | Logical OR is performed
 *
 *---Relational Operators---
 * <> | Returns 0 if top two values on stack equal, else 1
 * <= | tests if top minus 1 is less than or equal top
 * >= | tests if top minus 1 is greater or equal top
 * < | tests if top minus 1 less than top
 * > | tests if top minus 1 greater than top
 * = | tests if top minus 1 equal to top
 *
 *---Output---
 * print | writes top of stack contents to output device
 * show | writes a literal string to output device
 *
 *---Subprogram Control---
 * begin | Marks the beginning of parameter passing and subroutine call
 * end | Marks the end of parameter passing and subroutine call
 * return | return from subroutine
 * call | subroutine call
 *
 */

/* Parse Tree */
struct Node <T> {
	child: Vec<T>,
	entry: T
}

impl <T> Node <T> {
	fn new(entry: T) -> Node <T> {
		return Node {
			child: Vec::new(),
			entry: entry
		}
	}
}





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


fn lex(input: &String) -> Result<Vec<Token>, String>
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


	let FunctionDescriptorTree = Node::new("main");

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
						handle_number(raw, &mut iterator)
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



fn handle_number<T: Iterator<Item = char>> (raw: char, iterator: &mut Peekable<T>) -> i64 
{
	let mut num = raw.to_string().parse::<i64>().expect("Should have been a digit");

	while let Some(Ok(digit)) = iterator.peek().map(|c| c.to_string().parse::<i64>()) {
		num = num * 10 + digit; /* Apply binary operations here to make more efficient */
		iterator.next();
	}
	num
}



fn main() {


    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("\n\nError, exiting...\nUsage: {:?} src/demo.jaz", argv[0]);
        process::exit(1);
    }

    let data: String = read_in_file(&argv[1]);
    let a = Some(lex(&data));
    println!("{:?}", a);
}


fn read_in_file(file_name: &str) -> String {
	let path = Path::new(file_name);
	let display = path.display();

	/* Open the path in read-only mode, returns `io::Result<File>` */
	let mut file = match File::open(&path) {
		Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
		Ok(file) => file,
	};

	/* Read the file contents into a string, returns `io:Result<usize>` */
	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
		Ok(_) => print!("{} contains:\n{}", display, s),
	}
	
	s
	/* file goes out of scope, and file_name gets closed */
}