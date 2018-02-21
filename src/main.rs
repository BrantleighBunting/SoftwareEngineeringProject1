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



/* We want to return an array of c++ lines to write to file */
/* TODO: Build a method to write to file a Vec<String> */
fn filter_to_cpp(raw_tokens: Vec<Token>) -> Vec<String> {

	let output_lines: Vec<String> = Vec::new();

	for item in raw_tokens {
		match &item {
			
			Token::Printable(ref c) => {

				/* Printable is defined in Token as Printable(String),
				 * so we reference the string as "ref c"
				 */

				println!("Matched Printable, raw val: {:?}", c);

				println!("Equivalent Statement in c++: ");
				println!("cout << {:?} << endl", c);
			}
			Token::Keyword(ref c) => {
				println!("Matched Keyword, raw val: {:?}", c);
			}

			Token::Assigned(ref c) => {
				println!("Matched Assigned, raw val: {:?}", c);
			}
			Token::Whitespace(ref c) => {
				println!("Matched Whitespace, raw val: {:?}", c);
			}
			Token::Assignment(ref c) => {
				println!("Matched Assignment, raw val: {:?}", c);
			}
			Token::Constant(ref c) => {
				println!("Matched Constant, raw val: {:?}", c);
			} 		
			&Token::Equivalent => {

			}			
			&Token::LessThanEqual => {

			}		
			&Token::GreaterThanEqual => {

			}	
			&Token::LessThan => {

			} 			
			&Token::GreaterThan => {

			}		
			&Token::Equal => {

			}				
			&Token::And => {

			}				
			&Token::Or => {

			}					
			&Token::Not => {
				
			}				
			&Token::Plus => {
				
			}				
			&Token::Minus => {
				
			}				
			&Token::IntDiv => {
				
			}				
			RemainDiv => {
				
			}


			_ => {
				println!("Unreachable Hit.");
				unreachable!()
			}
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


