/* File I/O Imports */
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct FileIO;

impl FileIO {
	pub fn read_in_file(file_name: &str) -> String
	{
		let path = Path::new(file_name);
		let display = path.display();

		/* Open the path in read-only mode, returns `io::Result<File>` */
		let mut file = match File::open(&path)
		{
			Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
			Ok(file) => file,
		};

		/* Read the file contents into a string, returns `io:Result<usize>` */
		let mut s = String::new();
		match file.read_to_string(&mut s)
		{
			Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
			Ok(_) => {
				// print!("{} contains:\n{}", display, s)
			}
		}

		s
		/* file goes out of scope, and file_name gets closed */
	}
}