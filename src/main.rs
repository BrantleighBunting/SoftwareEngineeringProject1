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

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;


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

#[derive(Debug)]
struct Call {
	name: String,
	passed_params: Vec<String>,
	params: HashMap<String, String>,
	local_vars: HashMap<String, String>,
	body: Vec<String>
}

#[derive(Debug)]
struct Function {
	params: HashMap<String, String>,
	body: Vec<String>
}



/* We want to return an array of c++ lines to write to file */
/* TODO: Build a method to write to file a Vec<String> */
fn filter_to_cpp(raw_tokens: Vec<Token>) -> (Vec<String>, HashMap<String, i64>) {

	let mut output_lines: Vec<String> = Vec::new();

	let tokens = raw_tokens.clone();

	

	/* Stores our variable assignments, used with rvalue and print in jaz */
	let mut variables: HashMap<String, i64> = HashMap::new();
	/* To update values in this use: *my_map.get_mut("a").unwrap() += 10; */

	let mut call = Call { 
		name: String::new(),
		passed_params: Vec::new(),
		params: HashMap::new(),
		local_vars: HashMap::new(),
		body: Vec::new()
	};

	/* For the first pass, store a vector of calls for later use. */
	// let mut calls: Vec<Call> = Vec::new();

	let mut fp_iter = tokens.iter().peekable();


	let mut done_params: bool = true;

	while let Some(&raw) = fp_iter.peek() {
		match raw {
			Token::Keyword(ref lvalue) => {
				fp_iter.next();


				if lvalue == "begin" {
					println!("BEGIN");
					done_params = false;
					
				}

				
				if lvalue == "lvalue" {
					
					if let Token::Assignment(ref passed) = fp_iter.peek().unwrap() {
						fp_iter.next();
						
						if let Token::Keyword(ref rvalue) = fp_iter.peek().unwrap() {
							fp_iter.next();
							
							if rvalue == "rvalue" {

								/* parameter we want */
								if !done_params {
									if let Token::Assignment(ref rpassed) = fp_iter.peek().unwrap() {
										println!("LVALUE: {}", lvalue);
										println!("PASSED: {}", passed);
										println!("RVALUE: {}", rvalue);
										println!("PASSED: {}", rpassed);
										fp_iter.next();
										fp_iter.next();
										
											println!("Loaded Param into Call: {}", passed);
											call.passed_params.push(passed.clone());

											println!("Loaded Local Var into Call: {}", passed);
											println!("Loaded Local Var into Call: {}", rpassed);
											call.local_vars.insert(passed.clone(), rpassed.clone());

										
									}
								}
							}
						}
					}
				}
				

				

				
				if lvalue == "call" {
					done_params = true;
					if let Token::FunctionCallWithParams(ref fname) = fp_iter.peek().unwrap() {
						fp_iter.next();
						println!("Loaded Name into Call: {}", fname);
						call.name = fname.clone();
					}
				} 

			}
			_ => {fp_iter.next(); }
		}
	}


	/* Second Pass, loop through non-function tokens created by earlier pass */
	let mut iterable = tokens.iter().peekable();
	while let Some(&raw) = iterable.peek() {
		match raw {
			Token::Printable(ref c) => {

				/* Printable is defined in Token as Printable(String),
				 * so we reference the string as "ref c"
				 */

				println!("Matched Printable, raw val: {:?}", c);

				println!("Equivalent Statement in c++: ");
				println!("\ncout << {:?} << endl;\n", c);
				output_lines.push(format!("\tcout << {:?} << endl;\n", c));
				iterable.next();
			}
			Token::Keyword(ref c) => {
				println!("Matched Keyword, raw val: {:?}", c);

				/* Consume */
				iterable.next();


				if c == "halt" {
					output_lines.push("\treturn;\n}\n".to_string());
				}
				if c == "return" {
					output_lines.push("\treturn;\n}\n".to_string());
				}
				if c == "goto" {
					if let Token::Constant(ref constant) = iterable.peek().unwrap() {
						iterable.next();

						output_lines.push(format!("\tc{}();\n", constant))
					}
				}
				if c == "call" {
					if let Token::FunctionCallWithParams(ref name) = iterable.peek().unwrap() {
						// output_lines.push(format!("{:?}", ))

						let mut fcall = String::new();
						let mut index = 0;
						fcall.push_str("\t");
						fcall.push_str(name);
						call.name = name.to_string();
						println!("Assigned Name to Call: {}", name);
						fcall.push_str("(");
						
						for (key, value) in call.params.iter() {
							let parameter = *variables.get(value).unwrap();
							if index == call.params.keys().len() - 1 {
								fcall.push_str(&format!("&{}", value));
							} else {
								fcall.push_str(&format!("&{}", value));
								fcall.push_str(",");
							}
							index += 1;
							
						}
						fcall.push_str(");\n");

						output_lines.push(fcall);

						
					}
				}

				

				if c == "begin" {
					/* Start collecting parameters */
					let mut to_coll_function_tokens: bool = true;

					while to_coll_function_tokens == true {
						if let Token::Keyword(left_lval) = iterable.peek().unwrap() {
							
							iterable.next();

							if left_lval == "lvalue" { 
								// iterable.next();

								if let Token::Assignment(lvalue) = iterable.peek().unwrap() {
									
									iterable.next();

									if let Token::Keyword(right_rval) = iterable.peek().unwrap() {
										
										iterable.next();

										if let Token::Assignment(rvalue) = iterable.peek().unwrap() {
											
											iterable.next();
											iterable.next(); /* handle the := */

											call.params.insert(lvalue.to_string(), rvalue.to_string());
											
										}
									}
								}
							} else {
								to_coll_function_tokens = false;
							}
						}
					}
				}

				if c == "label" { /* Function Call Code */
					/* Functions can have either constant names or string names */
					if let Token::Constant(ref val) = iterable.peek().unwrap() {
						iterable.next();
						output_lines.push(format!("\nvoid c{}() {{\n", val));
					}

					if let Token::FunctionName(ref fname) = iterable.peek().unwrap() {
						iterable.next();

						println!("Got FunctionName: {}", fname);
						println!("Call Name: {}", call.name);
						if *fname == *call.name {
							output_lines.push(format!("\nvoid {}(", fname));
							for (i, value) in call.passed_params.iter().enumerate() {
								if i == call.passed_params.len() - 1 {
									output_lines.push(format!("uint64_t* {}", value));
								} else {
									output_lines.push(format!("uint64_t* {}, ", value));
								}
							}
							output_lines.push(") ".to_string());
							output_lines.push("{\n".to_string());
						} else {
							output_lines.push(format!("\nvoid {}() {{\n", fname));
						}
					}
				}

				if c == "lvalue" {
					println!("Got lvalue");
					if let Token::Assignment(ref lassign) = iterable.peek().unwrap() {
						println!("got assign: {}", lassign);
						iterable.next();
						if let Token::Keyword(ref rkeywd) = iterable.peek().unwrap() {
							println!("Got rkeywd: {}", rkeywd);
							iterable.next();

							if rkeywd == "push" {
								// iterable.next();
								if let Token::Constant(int) = iterable.peek().unwrap() {
									iterable.next();
									iterable.next(); /* skip := */
									
									println!("\nuint64_t {} = {};\n", lassign, int);
									variables.insert(lassign.clone(), *int);
								}
							}

							if let Token::Assignment(ref lvalue) = iterable.peek().unwrap() {
								println!("Got lvalue: {}", lvalue);

								iterable.next();
								if let Token::Keyword(ref keywd) = iterable.peek().unwrap() {
									println!("Got keywd: {}", keywd);
									iterable.next();

									if keywd == "push" {
										if let Token::Constant(ref constant) = iterable.peek().unwrap() {
											println!("Got constant: {}", constant);
											iterable.next();

											println!("LEFT ASSIGN: {}", lassign);
											println!("LEFT VALUE: {}", lvalue);
											if lvalue == lassign {
												println!("VALUES MATCHED");


												for (key, var) in call.local_vars.clone() {
													if var == lvalue.clone() {

													}
												}

												if call.local_vars.contains_key(lvalue) {
													println!("CALL LOCAL VARS CONTAINED: {}", lvalue);
													/* TODO: recognize operator for this */
													output_lines.push(
														format!(
															"\t*{} = *{} + {};\n",
															lvalue,
															lvalue,
															constant
														)
													);
												}

												
											}


											if let Token::Assignment(ref assign) = iterable.peek().unwrap() {
												println!("Got assign: {}", assign);
												iterable.next();

												
												

											}
										}
									}
									if keywd == "rvalue" {
										if let Token::Assignment(ref assign) = iterable.peek().unwrap() {
											println!("Got RIGHT assign: {}", assign);
											iterable.next();
											if let Token::Assignment(ref rassign) = iterable.peek().unwrap() {
												println!("Got assign: {}", rassign);
												iterable.next();

												if call.local_vars.contains_key(lvalue) {
													println!("CALL LOCAL VARS CONTAINED: {}", lvalue);
													/* TODO: recognize operator for this */
													output_lines.push(
														format!(
															"\t*{} = *{} + *{};\n",
															lvalue,
															lvalue,
															assign
														)
													);
												}
												
											}
										}
									}
								}
							}
						}
					}
				}



				if c == "rvalue" {
					if let Token::Assignment(ref val) = iterable.peek().unwrap() {
						iterable.next();
						if let Token::Keyword(ref a) = iterable.peek().unwrap() {
							if a == "print" {
								iterable.next();

								println!("Printable rvalue...");

								println!("\tcout << {} << endl;\n", variables.get(val).unwrap());
								output_lines.push(format!("\tcout << {} << endl;\n", val ));
							}
						}
					}
				}

				// if let Token::Assignment(variable) = iterable.peek().unwrap() {
				// 	println!("Matched Assignment, raw val: {:?}", variable);
				// 	iterable.next();

				// 	if let Token::Keyword(keywd) = iterable.peek().unwrap() {
				// 		if keywd == "push" {
				// 			iterable.next();
				// 			if let Token::Constant(int) = iterable.peek().unwrap() {
				// 				iterable.next();
				// 				println!("\nuint64_t {} = {};\n", variable, int);
				// 				variables.insert(variable.clone(), *int);
				// 			}
				// 		}
				// 	}
				// }
			}

			Token::FunctionCall(ref c) => {
				println!("Matched FunctionCall, raw val: {:?}", c);

				println!("\n{}();\n", c);

				output_lines.push(format!("\t{}();\n", c));
				output_lines.push("}\n".to_string());

				iterable.next();
			} 

			Token::FunctionName(ref c) => {
				println!("Matched FunctionName, raw val: {:?}", c);

				iterable.next();
			}

			Token::FunctionCallWithParams(ref c) => {
				println!("Matched FunctionCallWithParams, raw val: {:?}", c);

				iterable.next();
			}

			Token::Assigned(ref c) => {
				println!("Matched Assigned, raw val: {:?}", c);

				iterable.next();
			}
			Token::Whitespace(ref c) => {
				println!("Matched Whitespace, raw val: {:?}", c);

				iterable.next();
			}
			// Token::Assignment(ref c) => {
			// 	println!("Matched Assignment, raw val: {:?}", c);

			// 	iterable.next();
			// }
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

	return (output_lines, variables)
}


fn write_to_output(
	output_lines: Vec<String>,
	variables: HashMap<String, i64> /* Main function code */
	// function_declarations: Vec<Call>
) {
	let path = Path::new("src/out.cpp");
	let display = path.display();

  	

	let mut file = match File::create(&path) {
		Err(why) => panic!("Couldnt create {}: {}", display, why.description()),
		Ok(file) => file,
	};


	let boilerplate = "#include <iostream>
#include <stdint.h>
#include <stdio.h>
using namespace std;
";
	let main = "
int main(int argc, char* argv[]) {
";


	let mut top_level_vars = String::new();
	for var in variables.keys() {
		top_level_vars.push_str(&format!("uint64_t {} = {};\n", var, variables.get(var).unwrap()));
	}

	
	
	/* Write to file: #include statements */
	match file.write_all(boilerplate.as_bytes()) {
		Err(why) => {
			panic!("Couldnt write to: {}: {}", display, why.description());
		},
		Ok(_) => println!("Successfully wrote output file: {:?}", "Boilerplate")
	}

	/* Write to file: Top Level Variables */
	match file.write_all(top_level_vars.as_bytes()) {
		Err(why) => {
			panic!("Couldnt write to: {}: {}", display, why.description());
		},
		Ok(_) => println!("Successfully wrote output file: {:?}", "Top Level Variables")
	}

	/* Write to file: Main Declaration */
	match file.write_all(main.as_bytes()) {
		Err(why) => {
			panic!("Couldnt write to: {}: {}", display, why.description());
		},
		Ok(_) => println!("Successfully wrote output file: {:?}", "Main Declaration")
	}

	for output in output_lines {
		match file.write_all(output.as_bytes()) {
			Err(why) => {
				panic!("Couldnt write to: {}: {}", display, why.description());
			},
			Ok(_) => {}
		}
	}


	match file.write_all("}".as_bytes()) {
			Err(why) => {
				panic!("Couldnt write to: {}: {}", display, why.description());
			},
			Ok(_) => println!("Successfully wrote output file: {:?}", "Boilerplate")
	}
	
}

fn main() {

    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("\n\nError, exiting...\nUsage: {:?} src/demo.jaz", argv[0]);
        process::exit(1);
    }

    let data: String = FileIO::read_in_file(&argv[1]);

    let result: Vec<Token> = Tokenizer::lex(&data).unwrap();

    println!("\n\n");
    for item in &result {
    	println!("{:?}", item);
    }

    println!("\n\n");
    
    let filtered = filter_to_cpp(result.clone());
    /* Tuple Access Syntax, very nice */
    let output_lines = filtered.0;
    let variables = filtered.1;

    write_to_output(output_lines, variables);

   
}


