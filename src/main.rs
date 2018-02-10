#![allow(unused_variables)]
#![allow(dead_code)]

/* Command Line Args Imports */
use std::env;
use std::process;

/* Module Imports */
mod tokenizer;
mod fileio;

/* Module Utilization */
use tokenizer::Tokenizer;
use fileio::FileIO;


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



fn main() {

    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("\n\nError, exiting...\nUsage: {:?} src/demo.jaz", argv[0]);
        process::exit(1);
    }

    let data: String = FileIO::read_in_file(&argv[1]);
    let a = Some(Tokenizer::lex(&data));
    println!("{:?}", a);
}


