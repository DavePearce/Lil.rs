use std::io;
use std::io::Write;

mod ast;
mod lexer;
mod parser;
mod source_map;

use crate::parser::Parser;
use crate::parser::Error;
use crate::source_map::SourceMap;

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
	let line = input.as_str();
	// Construct temporary source map
	let mut source_map = SourceMap::new(line);	
	// Parse it!
	let d = Parser::new(line, |s| source_map.map(s)).parse_decl();
	//
	if d.is_err() {
	    print_error(line,d.err().unwrap());
	} else {
	    println!("DECL: {}",d.ok().unwrap());
	}	  
	//
	input.clear();
    }
}

fn print_error(line: &str, err: Error) {
    println!("error:{}: {}",err.start,err.message);
    println!();
    print!("{}",line);
    let indent = to_whitespace(line,err.start);
    print!("{}",indent);
    //
    for i in err.start .. err.end {
	print!("^");
    }
    println!("");
}

/// Convert the start of a given line into corresponding whitespace.
/// This is pretty straightforward, where most characters are simply
/// converted into spaces.  However, in some cases, we want to keep
/// the character as is (e.g. for a tab).
fn to_whitespace(line: &str, offset: usize) -> String {
    // Personally, a loop has more clarity than this jiberish :)
    line.char_indices().filter(|s| s.0 < offset).map(|s| " ").collect()
}
