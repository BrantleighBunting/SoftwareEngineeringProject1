#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![feature(match_default_bindings)]
#![allow(dead_code)]

/* Command Line Args Imports */
use std::env;
use std::process;

/* Module Imports */
mod tokenizer;
mod fileio;
mod parsetree;

/* Module Utilization */
use tokenizer::{Tokenizer, Token};
use fileio::FileIO;
use parsetree::Node;
use std::collections::HashMap;
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



fn build_parse_tree(raw_tokens: &Vec<Token>) -> Option<Node<Token>> {

	let tokens = raw_tokens.clone();

	let root: Node<&Token> = Node::new(tokens.first().unwrap());

	let mut iterable = tokens.iter().peekable();

	while let Some(&raw) = iterable.peek() {

	}


	for item in tokens.clone() {
		/* Build the parse tree by recursively looping through the tree */

	}


	return None;
}


/* We want to return an array of c++ lines to write to file */
/* TODO: Build a method to write to file a Vec<String> */
fn filter_to_cpp(raw_tokens: Vec<Token>) -> Vec<String> {

	let output_lines: Vec<String> = Vec::new();

	let tokens = raw_tokens.clone();

	let mut iterable = tokens.iter().peekable();

	/* Stores our variable assignments, used with rvalue and print in jaz */
	let mut variables: HashMap<String, i64> = HashMap::new();
	


	while let Some(&raw) = iterable.peek() {
		match raw {
			Token::Printable(ref c) => {

				/* Printable is defined in Token as Printable(String),
				 * so we reference the string as "ref c"
				 */

				println!("Matched Printable, raw val: {:?}", c);

				println!("Equivalent Statement in c++: ");
				println!("cout << {:?} << endl", c);

				iterable.next();
			}
			Token::Keyword(ref c) => {
				println!("Matched Keyword, raw val: {:?}", c);

				/* Consume */
				iterable.next();

				if c == "rvalue" {
					iterable.next();
					if let Token::Keyword(ref a) = iterable.peek().unwrap() {
						if a == "print" {
							iterable.next();

							println!("Printable rvalue...");
						}
					}
				}

				if let Token::Assignment(variable) = iterable.peek().unwrap() {
					iterable.next();

					if let Token::Keyword(keywd) = iterable.peek().unwrap() {
						if keywd == "push" {
							iterable.next();
							if let Token::Constant(int) = iterable.peek().unwrap() {
								iterable.next();
								println!("uint64_t {} = {};", variable, int);
								variables.insert(variable.clone(), *int);
							}
						}
					}
				}
			}

			Token::Assigned(ref c) => {
				println!("Matched Assigned, raw val: {:?}", c);

				iterable.next();
			}
			Token::Whitespace(ref c) => {
				println!("Matched Whitespace, raw val: {:?}", c);

				iterable.next();
			}
			Token::Assignment(ref c) => {
				println!("Matched Assignment, raw val: {:?}", c);

				iterable.next();
			}
			Token::Constant(ref c) => {
				println!("Matched Constant, raw val: {:?}", c);

				iterable.next();
			} 		
			&Token::Equivalent => {

				iterable.next();
			}			
			&Token::LessThanEqual => {
				iterable.next();
			}		
			&Token::GreaterThanEqual => {
				iterable.next();
			}	
			&Token::LessThan => {
				iterable.next();
			} 			
			&Token::GreaterThan => {
				iterable.next();
			}		
			&Token::Equal => {
				iterable.next();
			}				
			&Token::And => {
				iterable.next();
			}				
			&Token::Or => {
				iterable.next();
			}					
			&Token::Not => {
				iterable.next();
			}				
			&Token::Plus => {
				iterable.next();
			}				
			&Token::Minus => {
				iterable.next();
			}				
			&Token::IntDiv => {
				iterable.next();
			}				
			RemainDiv => {
				iterable.next();
			}

			_ => { unreachable!() }
		}
	}

	return output_lines
}



fn main() {

    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("\n\nError, exiting...\nUsage: {:?} src/demo.jaz", argv[0]);
        process::exit(1);
    }

    let data: String = FileIO::read_in_file(&argv[1]);

    let result: Vec<Token> = Tokenizer::lex(&data).unwrap();

    println!("{:?}\n\n", result);

    filter_to_cpp(result.clone());

   
}


