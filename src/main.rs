use std::io;
use std::io::Write;
use parser::Parser;

mod ast;
mod lexer;
mod parser;

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
	// Parse it!
	let d = Parser::new(input.as_str()).parse();
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
