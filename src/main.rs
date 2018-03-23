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
	operation_on_variable: HashMap<String, (String, i64)>, /* Defines an operation on the passed variable */
	body: Vec<String>
}

#[derive(Debug)]
struct Function {
	params: HashMap<String, String>,
	body: Vec<String>
}



/* We want to return an array of c++ lines to write to file */
/* TODO: Build a method to write to file a Vec<String> */
fn filter_to_cpp(raw_tokens: Vec<Token>) -> (Vec<String>, HashMap<String, i64>, Call) {

	let mut output_lines: Vec<String> = Vec::new();

	let tokens = raw_tokens.clone();

	

	/* Stores our variable assignments, used with rvalue and print in jaz */
	let mut variables: HashMap<String, i64> = HashMap::new();
	/* To update values in this use: *my_map.get_mut("a").unwrap() += 10; */


	/* Store all calls in here */
	let mut calls: Vec<Call> = Vec::new();

	let mut call = Call { 
		name: String::new(),
		passed_params: Vec::new(),
		params: HashMap::new(),
		local_vars: HashMap::new(),
		operation_on_variable: HashMap::new(),
		body: Vec::new()
	};

	let mut fp_iter = tokens.iter().peekable();
	let mut done_params: bool = true;
	let mut to_replace_inner_with_goto: bool = false;

	while let Some(&raw) = fp_iter.peek() {
		match raw {
			Token::Keyword(ref lvalue) => {
				fp_iter.next();
				/* These 3 if statements collect a function call with parameters */
				if lvalue == "begin" {
					done_params = false;
				}
				if lvalue == "lvalue" {
					if let Some(Token::Assignment(ref passed)) = fp_iter.peek() {
						fp_iter.next();
						if let Some(Token::Keyword(ref rvalue)) = fp_iter.peek() {
							fp_iter.next();
							if rvalue == "rvalue" {
								if !done_params {
									if let Some(Token::Assignment(ref rpassed)) = fp_iter.peek() {
										fp_iter.next();
										fp_iter.next();
										call.passed_params.push(passed.clone());
										call.local_vars.insert(passed.clone(), rpassed.clone());
									}
								}
							}
						}
					}
				}
				if lvalue == "call" {
					done_params = true;
					if let Some(Token::FunctionCallWithParams(ref fname)) = fp_iter.peek() {
						fp_iter.next();
						call.name = fname.clone();
					}
				}
			}
			Token::FunctionName(ref c) => {
				fp_iter.next();
			}
			Token::FunctionCallWithParams(ref function_name) => {

				/* If we didnt consume this earlier, the call is a function without params */
				calls.push(Call { 
					name: function_name.clone(),
					passed_params: Vec::new(),
					params: HashMap::new(),
					local_vars: HashMap::new(),
					operation_on_variable: HashMap::new(),
					body: Vec::new()
				});

				fp_iter.next();
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
				output_lines.push(format!("\tcout << {:?} << endl;\n", c));
				iterable.next();
			}
			Token::Keyword(ref first_key_in_pattern) => {
				iterable.next();
				if first_key_in_pattern == "push" {
					if let Some(Token::Constant(constant)) = iterable.peek() {
						iterable.next();
						if let Some(Token::Keyword(show)) = iterable.peek() {
							iterable.next();
							if show == "show" {
								/* push -> constant -> show -> ! */
								if let Some(Token::Printable(printable)) = iterable.peek() {
									iterable.next();
									/* Handle the printable token */
									output_lines.push(format!("\tcout << {:?} << endl;\n", printable));
									if let Some(Token::Not) = iterable.peek() {
										iterable.next();
										output_lines.push(format!("\tcout << \" \" << !{} << endl;\n", constant))
									}
								}
							}
							if show == "push" {
								/* push -> constant -> push -> constant */
								if let Some(Token::Constant(sub_const)) = iterable.peek() {
									iterable.next();
									if let Some(Token::Keyword(shown)) = iterable.peek() {
										iterable.next();
										if shown == "show" {
											if let Some(Token::Printable(printable)) = iterable.peek() {
												iterable.next();
												/* Handle the printable token */
												output_lines.push(format!("\tcout << {:?} << endl;\n", printable));

												let mut operator;
												match iterable.peek() {
													Some(Token::Plus) => {
														operator = '+';
														output_lines.push(
															format!(
																"\tcout << {} {} {} << endl;\n", 
																constant,
																operator, 
																sub_const
															)
														);
														iterable.next();
													}
													Some(Token::Minus) => {
														operator = '-';
														output_lines.push(
															format!(
																"\tcout << {} {} {} << endl;\n", 
																constant,
																operator, 
																sub_const
															)
														);
														iterable.next();
													}
													Some(Token::IntDiv) => {
														operator = '/';
														output_lines.push(
															format!(
																"\tcout << {} {} {} << endl;\n", 
																constant,
																operator, 
																sub_const
															)
														);
														iterable.next();
													}
													Some(Token::Multiply) => {
														operator = '*';
														output_lines.push(
															format!(
																"\tcout << {} {} {} << endl;\n", 
																constant,
																operator, 
																sub_const
															)
														);
														iterable.next();
													}
													Some(Token::Or) => {
														operator = '|';
														output_lines.push(
															format!(
																"\tcout << ({} {} {}) << endl;\n", 
																constant,
																operator, 
																sub_const
															)
														);
														iterable.next();
													}
													Some(Token::And) => {
														operator = '&';
														output_lines.push(
															format!(
																"\tcout << ({} {} {}) << endl;\n", 
																constant,
																operator, 
																sub_const
															)
														);
														iterable.next();
													}
													Some(Token::RemainDiv(ref a)) => {
														operator = '%';
														output_lines.push(
															format!(
																"\tcout << {} {} {} << endl;\n", 
																constant,
																operator, 
																sub_const
															)
														);
														iterable.next();
													}

													Some(Token::Keyword(ref div)) => {
														if div == "div" {
															operator = '%';
															output_lines.push(
																format!(
																	"\tcout << {} {} {} << endl;\n", 
																	constant,
																	operator, 
																	sub_const
																)
															);
															iterable.next();
														} else {
															output_lines.push(
																format!("\tcout << {} - {} << endl;\n", 
																	constant,
																	sub_const
																)
															);
														}
													}

													Some(Token::LessThan) => {
														iterable.next();
														match iterable.peek() {
															Some(Token::GreaterThan) => {
																iterable.next();
																output_lines.push(
																	format!("\tcout << (int)({} != {}) << endl;\n", 
																		constant,
																		sub_const
																	)
																);
															}
															Some(Token::Equal) => {
																iterable.next();
																output_lines.push(
																	format!("\tcout << (int)({} <= {}) << endl;\n", 
																		constant,
																		sub_const
																	)
																);
															}

															_ => {
																iterable.next();
																output_lines.push(
																	format!("\tcout << (int)({} < {}) << endl;\n", 
																		constant,
																		sub_const
																	)
																);
															}
														}
													}

													Some(Token::GreaterThan) => {
														iterable.next();
														match iterable.peek() {
															Some(Token::Equal) => {
																iterable.next();
																output_lines.push(
																	format!("\tcout << (int)({} >= {}) << endl;\n", 
																		constant,
																		sub_const
																	)
																);
															}

															_ => {
																iterable.next();
																output_lines.push(
																	format!("\tcout << (int)({} > {}) << endl;\n", 
																		constant,
																		sub_const
																	)
																);
															}
														}
													}

													None => {
														output_lines.push(
															format!("\tcout << {} - {} << endl;\n", 
																constant,
																sub_const
															)
														);
													}
													_ => {
														output_lines.push(
															format!("\tcout << {} - {} << endl;\n", 
																constant,
																sub_const
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
				}
				if first_key_in_pattern == "halt" {
					to_replace_inner_with_goto = false;
					output_lines.push("\n}\n".to_string());
				}
				if first_key_in_pattern == "return" {
					to_replace_inner_with_goto = false;
					output_lines.push("\treturn;\n}\n".to_string());
				}
				if first_key_in_pattern == "goto" {
					if let Some(Token::Constant(ref constant)) = iterable.peek(){
						iterable.next();
						output_lines.push(format!("\tc{}();\n}}\n", constant))
					}
				}
				if first_key_in_pattern == "call" {
					to_replace_inner_with_goto = false;
					if let Some(Token::FunctionCallWithParams(ref name)) = iterable.peek() {
						// output_lines.push(format!("{:?}", ))

						let mut fcall = String::new();
						let mut index = 0;

						fcall.push_str("\t");
						fcall.push_str(name);

						call.name = name.to_string();

						fcall.push_str("(");

						let col_params: Vec<_> = call.params.iter().collect();
						for (key, value) in col_params.iter() {
							match variables.get(*value) {
								Some(parameter) => {
									if index == call.params.keys().len() - 1 {
										fcall.push_str(&format!("&{}", value));
									} else {
										fcall.push_str(&format!("&{}", value));
										fcall.push_str(",");
									}
									index += 1;
								}
								None => {
								}
							}						
						}
						fcall.push_str(");\n");
						output_lines.push(fcall);
					}
				}
				if first_key_in_pattern == "begin" {
					/* Start collecting parameters */
					let mut to_coll_function_tokens: bool = true;
					while to_coll_function_tokens == true {
						if let Some(Token::Keyword(left_lval)) = iterable.peek() {
							iterable.next();
							if left_lval == "lvalue" { 
								if let Some(Token::Assignment(lvalue)) = iterable.peek() {
									iterable.next();
									if let Some(Token::Keyword(right_rval)) = iterable.peek() {
										iterable.next();
										if let Some(Token::Assignment(rvalue)) = iterable.peek() {
											if let Some(Token::Keyword(pushed)) = iterable.peek() {
												iterable.next();
												if let Some(Token::Constant(constant)) = iterable.peek() {
													iterable.next();
													let mut operator;
													match iterable.peek() {
														Some(Token::Plus) => {
															operator = '+';
															call.params.insert(lvalue.to_string(), rvalue.to_string() );
															call.operation_on_variable.insert(lvalue.to_string(), (operator.to_string(), *constant));
															iterable.next();
														}
														Some(Token::Minus) => {
															operator = '-';
															call.params.insert(lvalue.to_string(), rvalue.to_string() );
															call.operation_on_variable.insert(lvalue.to_string(), (operator.to_string(), *constant));
															iterable.next();
														}
														Some(Token::IntDiv) => {
															operator = '/';
															call.params.insert(lvalue.to_string(), rvalue.to_string() );
															call.operation_on_variable.insert(lvalue.to_string(), (operator.to_string(), *constant));
															iterable.next();
														}
														Some(Token::Multiply) => {
															operator = '*';
															call.params.insert(lvalue.to_string(), rvalue.to_string() );
															call.operation_on_variable.insert(lvalue.to_string(), (operator.to_string(), *constant));
															iterable.next();
														}
														Some(Token::RemainDiv(ref a)) => {
															operator = '%';
															call.params.insert(lvalue.to_string(), rvalue.to_string() );
															call.operation_on_variable.insert(lvalue.to_string(), (operator.to_string(), *constant));
															iterable.next();
														}
														None => {
															call.params.insert(lvalue.to_string(), rvalue.to_string());
														}
														_ => {}
													}
													iterable.next();
													iterable.next(); /* handle the := */
												}
											} else {
												iterable.next();
												iterable.next(); /* handle the := */

												call.params.insert(lvalue.to_string(), rvalue.to_string());
											}
										}
									}
								}
							} else {
								to_coll_function_tokens = false;
							}
						} else {
							to_coll_function_tokens = false;
						}
					}
				}
				if first_key_in_pattern == "label" { /* Function Call Code */
					/* Functions can have either constant names or string names */
					if let Some(Token::Constant(ref val)) = iterable.peek() {
						if !to_replace_inner_with_goto {
							to_replace_inner_with_goto = true;
							iterable.next();
							output_lines.push(format!("\nvoid c{}() {{\n", val));
						} else {
							iterable.next();
							output_lines.push(format!("\ngoto c{}\n", val));	
						}	
					}
					if let Some(Token::FunctionName(ref fname)) = iterable.peek() {
						if !to_replace_inner_with_goto {
							to_replace_inner_with_goto = true;
							iterable.next();
							if *fname == *call.name {
								output_lines.push(format!("\nvoid {}(", fname));
								let col_params: Vec<_> = call.passed_params.iter().collect();
								for (i, value) in col_params.iter().enumerate() {
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
						} else {
							iterable.next();
							output_lines.push(format!("\n{}:\n", fname));
						}
					}
				}
				if first_key_in_pattern == "lvalue" {
					if let Some(Token::Assignment(ref lassign)) = iterable.peek() {
						iterable.next();
						if let Some(Token::Keyword(ref rkeywd)) = iterable.peek() {
							iterable.next();
							if rkeywd == "push" {
								if let Some(Token::Constant(int)) = iterable.peek() {
									iterable.next();
									iterable.next(); /* skip := */
									variables.insert(lassign.clone(), *int);
								}
							}
							if let Some(Token::Assignment(ref lvalue)) = iterable.peek() {
								iterable.next();
								if let Some(Token::Keyword(ref keywd)) = iterable.peek() {
									iterable.next();
									if keywd == "push" {
										if let Some(Token::Constant(ref constant)) = iterable.peek() {
											iterable.next();
											if lvalue == lassign {
												let mut operator = '-';
												match iterable.peek() {
													Some(Token::Plus) => {
														operator = '+';
														iterable.next();
													}
													Some(Token::Minus) => {
														operator = '-';
														iterable.next();
													}
													Some(Token::IntDiv) => {
														operator = '/';
														iterable.next();
													}
													Some(Token::Multiply) => {
														operator = '*';
														iterable.next();
													}
													Some(Token::RemainDiv(ref a)) => {
														operator = '%';
														iterable.next();
													}
													None => {}
													_ => {}
												}

												if call.local_vars.contains_key(lvalue) {
													output_lines.push(
														format!(
															"\t*{} = *{} {} {};\n",
															lvalue,
															lvalue,
															operator,
															constant
														)
													);
												}	
											}
										}
									}
									if keywd == "rvalue" {
										if let Some(Token::Assignment(ref assign)) = iterable.peek() {
											iterable.next();
											let mut operator = '_';
											match iterable.peek() {
												Some(Token::Plus) => {
													operator = '+';
													iterable.next();
												}
												Some(Token::Minus) => {
													operator = '-';
													iterable.next();
												}
												Some(Token::IntDiv) => {
													operator = '/';
													iterable.next();
												}
												Some(Token::Multiply) => {
													operator = '*';
													iterable.next();
												}
												Some(Token::RemainDiv(ref a)) => {
													operator = '%';
													iterable.next();
												}
												None => {}
												_ => {}
											}
											if let Some(Token::Assignment(ref rassign)) = iterable.peek() {
												iterable.next();
												if call.local_vars.contains_key(lvalue) {
													match iterable.peek() {
														Some(Token::Plus) => {
															operator = '+';
															iterable.next();
														}
														Some(Token::Minus) => {
															operator = '-';
															iterable.next();
														}
														Some(Token::IntDiv) => {
															operator = '/';
															iterable.next();
														}
														Some(Token::Multiply) => {
															operator = '*';
															iterable.next();
														}
														Some(Token::RemainDiv(ref a)) => {
															operator = '%';
															iterable.next();
														}
														None => {}
														_ => {}
													}
													output_lines.push(
														format!(
															"\t*{} = *{} {} *{};\n",
															lvalue,
															lvalue,
															operator,
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
				if first_key_in_pattern == "rvalue" {
					if let Some(Token::Assignment(ref val)) = iterable.peek() {
						iterable.next();
						if let Some(Token::Keyword(ref pushed)) = iterable.peek() {
							if pushed == "push" {
								iterable.next();
								if let Some(Token::Constant(ref constant)) = iterable.peek() {
									iterable.next();
									if let Some(Token::GreaterThan) = iterable.peek() {
										iterable.next();
										if let Some(Token::Keyword(ref gofalse)) = iterable.peek() {
											iterable.next();
											if let Some(Token::GotoLabel(ref label)) = iterable.peek() {
												output_lines.push(format!("\tif (*{} < {}) {{\n", val, constant));
												output_lines.push(format!("\t\tgoto {};\n", label));
												output_lines.push("\t}\n".to_string());
											}
										}
									}
								}
							}
						}
						if let Some(Token::Keyword(ref a)) = iterable.peek() {
							if a == "print" {
								iterable.next();
								match variables.get(val) {
									Some(variable) => {
										output_lines.push(format!("\tcout << {} << endl;\n", val));
									}
									None => {
										match call.params.get(val) {
											Some(variable) => {
												output_lines.push(format!("\tcout << {} << endl;\n", variable));
											}
											None => {}
										}
									}
								}
							}
						}
					}
				}
			}
			Token::FunctionCall(ref c) => {
				if to_replace_inner_with_goto {
					output_lines.push(format!("\tgoto {};\n", c));
					iterable.next();
				} else {
					output_lines.push(format!("\t{}();\n", c));
					output_lines.push("}\n".to_string());
					iterable.next();
				}
			} 
			Token::FunctionName(ref c) => {
				iterable.next();
			}
			Token::FunctionCallWithParams(ref c) => {
				iterable.next();
			}
			Token::Assigned(ref c) => {
				iterable.next();
			}
			Token::Whitespace(ref c) => {
				iterable.next();
			}
			Token::Constant(ref c) => {
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

	return (output_lines, variables, call)
}

fn write_to_output(
	output_lines: Vec<String>,
	variables: HashMap<String, i64>,
	call: Call
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

	let mut function_declarations: String = "\n".to_string();

	for line in output_lines.clone() {
		let mut b_line = line.clone();
		let a:Option<&mut str> = b_line.get_mut(1..5); /* Grabs "void" */
		let a = a.map(|a| {
			&*a
		});
		match a {
			Some("void") => {
				let chars_to_trim: &[char] = &[' ', '(', ')', '{', '\n' ];
				let mut trimmed: &str = line.trim_matches(chars_to_trim);
				let str_trimmed: String = trimmed.replace("void ", "");

				if str_trimmed == call.name {
					/* Handle parameter passing call here */
					let mut out_str: String = format!("void {} (", call.name);
					for (index, key) in call.params.keys().enumerate() {
						if index == call.params.len() - 1 {
							out_str.push_str(&format!("uint64_t *{}", key).to_string());
						} else {
							out_str.push_str(&format!("uint64_t *{}, ", key).to_string());
						}
					}
					out_str.push_str(");\n");
					function_declarations.push_str(&out_str);
				} else {
					/* Put all other non-parameter passing calls here */
					function_declarations.push_str(&format!("void {}();\n", str_trimmed));
				}
			}
			_ => {}
		}
	}

	let mut top_level_vars = "\n".to_string();

	let col_params: Vec<_> = variables.keys().collect();

	for var in col_params {
		top_level_vars.push_str(&format!("uint64_t {} = {};\n", var, variables.get(var).unwrap()));
	}

	/* Write to file: #include statements */
	match file.write_all(boilerplate.as_bytes()) {
		Err(why) => {
			panic!("Couldnt write to: {}: {}", display, why.description());
		},
		Ok(_) => println!("Successfully wrote to output file: {:?}", "Boilerplate")
	}

	/* Write to file: Function Declarations */
	match file.write_all(function_declarations.as_bytes()) {
		Err(why) => {
			panic!("Couldnt write to: {}: {}", display, why.description());
		},
		Ok(_) => println!("Successfully wrote to output file: {:?}", "Function Declarations")
	}

	/* Write to file: Top Level Variables */
	match file.write_all(top_level_vars.as_bytes()) {
		Err(why) => {
			panic!("Couldnt write to: {}: {}", display, why.description());
		},
		Ok(_) => println!("Successfully wrote to output file: {:?}", "Top Level Variables")
	}

	/* Write to file: Main Declaration */
	match file.write_all(main.as_bytes()) {
		Err(why) => {
			panic!("Couldnt write to: {}: {}", display, why.description());
		},
		Ok(_) => println!("Successfully wrote to output file: {:?}", "Main Declaration")
	}

	for output in output_lines {
		match file.write_all(output.as_bytes()) {
			Err(why) => {
				panic!("Couldnt write to: {}: {}", display, why.description());
			},
			Ok(_) => {}
		}
	}
}

fn main() {

    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("\n\nError, exiting...\nUsage: {:?} src/factProc.jaz", argv[0]);
        process::exit(1);
    }

    let data: String = FileIO::read_in_file(&argv[1]);
    let result: Vec<Token> = Tokenizer::lex(&data).unwrap();
    
    let filtered = filter_to_cpp(result.clone());
    /* Tuple Access Syntax, very nice */
    let output_lines = filtered.0;
    let variables = filtered.1;
    let call = filtered.2;

    write_to_output(output_lines, variables, call);

   
}


