use std::io;
use std::io::Write;

mod ast;
mod lexer;
mod parser;
mod source_map;
//mod typer;
//mod type_map;
mod error;

use crate::parser::Parser;
use crate::parser::Error;
use crate::source_map::SourceMap;
use crate::error::SyntaxError;
use crate::ast::AbstractSyntaxTree;

fn main() -> Result<(),io::Error> {
    repl()?;
    Ok(())
}

fn repl() -> Result<(),io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    // Buffer to hold input before parsing.
    let mut input = String::new();
    //
    let mut ast = AbstractSyntaxTree::new();
    //
    loop {
	write!(stdout,"> ");
	stdout.flush()?;
	// Read input line
    stdin.read_line(&mut input)?;
	let line = input.as_str();
	// Construct temporary source map
	let mut source_map = SourceMap::new(line);
	// Construct temporary type map
	// let mut type_map = TypeMap::new();
	// Parse it!
	let mut p = Parser::new(line, &mut ast, |i,s| source_map.map(i,s));
	let d = p.parse_decl();
	//
	if d.is_err() {
	    print_error(line,d.err().unwrap());
	} else {
	    let _decl = d.ok().unwrap();
	    // println!("Parsed: {}",&decl);
	    // println!("Parsed: {}",&ast);
	    // // Now type check it!
	    // let typing = TypeChecker::new(|i,t| type_map.map(i,t)).check(&ast);
	    // //
	    // if typing.is_err() {
	    // 	print_syntax_error(&typing.err().unwrap(), &source_map);
	    // } else {
	    // 	println!("Type checking suceeded");
	    // }
	}
	//
	input.clear();
    }
}

fn print_error(line: &str, err: Error) {
    println!("error:{}: {}",err.start,err.message);
    println!();
    print!("{}",line);
    print_highlight(line,err.start,err.end);
}

fn print_syntax_error<'a>(err: &SyntaxError, map: &SourceMap<'a>) {
    println!("error: {}",err.errno);
    // Determine the highlight
    let hl = map.get_highlight(err.node);
    // Print the enclosing line
    print!("{}",hl.line);
    // Highlight relevant section
    print_highlight(hl.line,hl.start,hl.end);
}

fn print_highlight<'a>(line: &'a str, start: usize, end: usize) {
    // Convert the given line into equivalent whitespace
    let indent = to_whitespace(line,start);
    // Print out preamble
    print!("{}",indent);
    //
    for _ in start .. end {
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
    line.char_indices().filter(|s| s.0 < offset).map(|_| " ").collect()
}
