use std::io;
use std::io::Write;
use std::marker::PhantomData;

mod ast;
mod lexer;
mod parser;
mod source_map;

use crate::parser::Parser;
use crate::source_map::SourceMap;
// use crate::ast;

fn main() {
    repl();
}

fn repl() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    // Buffer to hold input before parsing.
    let mut input = String::new();
    //
    loop {
	write!(stdout,"> ");	
	stdout.flush();
	// Read input line
        stdin.read_line(&mut input);
	let input_str = input.as_str();
	// Construct temporary source map
	let mut source_map = SourceMap::new(input_str);	
	// Parse it!
	let d  = Parser::new(input_str, |s| source_map.map(s)).parse();
	//
	if d.is_none() {
	    println!("*** ERROR!");
	} else {
	    println!("DECL: {}",d.unwrap());
	}	  
	//
	input.clear();
    }
}
