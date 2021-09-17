use std::result;
use crate::lexer::EOF;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::ast::*;

pub type Result<T> = result::Result<T, ()>;

// =================================================================
// Parser
// =================================================================

/// Response for turning a stream of tokens into an Abstract Syntax
/// Tree and/or producing error messages along the way.
struct Parser<'a> {
    /// Provides access to our token stream.
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
   
    pub fn new(input: &'a str) -> Self {
	Self { lexer: Lexer::new(input) }
    }
    
    /// Parse a declaration from token stream.  This returns `None`
    /// when the end of the stream is reached
    fn parse(&mut self) -> Option<Decl> {
	let lookahead = self.lexer.peek();
	// Check whether stream empty
	if lookahead == EOF {
	    None
	} else {
	    // Attempt to parse declaration
	    let r = self.parse_decl();
	    // Decode what happened.
	    match r {
		Ok(d) => {
		    Some(d)
		}
		Err(e) => {
		    Some(Decl::Error)
		}
	    }
	}
    }

    // =========================================================================
    // Declarations
    // =========================================================================    

    /// Parse an arbitrary declaration
    fn parse_decl(&mut self) -> Result<Decl> {
	let lookahead = self.lexer.peek();
	// Attempt to parse declaration
	match lookahead.kind {
	    TokenType::Type => {
		self.parse_decl_type()
	    }
	    _ => {
		Err(())
	    }
	}
    }
    
    /// Parse a type declaration of the from `type name is type;`.
    pub fn parse_decl_type(&mut self) -> Result<Decl> {
	// "type"
	let tok = self.snap(TokenType::Type)?;
	// Identifier
	let name = self.parse_identifier()?;
	// "is"
	self.snap(TokenType::Equal)?;
	// Type
	let typ_e : Type = self.parse_type()?;
	// Semi-colon
	let _ = self.snap(TokenType::SemiColon)?;
	//
	Ok(Decl::TypeAlias(name,typ_e))
    }

    // =========================================================================
    // Types
    // =========================================================================    

    pub fn parse_type(&mut self) -> Result<Type> {
	self.parse_type_compound()
    }

    pub fn parse_type_compound(&mut self) -> Result<Type> {
	let l = self.lexer.peek();
	// Attemp to distinguish
	match l.kind {
	    TokenType::EOF => {
		// Something went wrong
		Err(())
	    }
	    TokenType::Ampersand => {
		// Looks like a reference type
		self.parse_type_ref()
	    }
	    TokenType::LeftCurly => {
		// Looks like a record type
		self.parse_type_record()
	    }
	    _ => {
		// Could be an array type
		self.parse_type_array()	    
	    }
	}
    }

    pub fn parse_type_ref(&mut self) -> Result<Type> {
	let mut n = 1;
	// "&"
	self.snap(TokenType::Ampersand)?;
	// Check for nested references
	while self.snap(TokenType::Ampersand).is_ok() {
	    n = n + 1;
	}	
	// Type	
	let mut t = self.parse_type_bracketed()?;	
	// Unwind references
	for i in 0..n {
	    t = Ref(t);
	}
	// Done
	Ok(t)
    }

    pub fn parse_type_record(&mut self) -> Result<Type> {
	let mut fields : Vec<(Type,String)> = vec![];
	// "&"
	self.snap(TokenType::LeftCurly)?;
	// Keep going until a right brace
	while self.snap(TokenType::RightCurly).is_err() {
	    // Check if first time or not
	    if !fields.is_empty() {
		// Not first time, so match comma
		self.snap(TokenType::Comma)?;
	    }
	    // Type
	    let f_type = self.parse_type()?;
	    // Identifier
	    let f_name = self.parse_identifier()?;
	    // 
	    fields.push((f_type,f_name));
	}
	// Done
	Ok(Type::Record(fields))
    }

    pub fn parse_type_array(&mut self) -> Result<Type> {
	// Type
	let mut t = self.parse_type_bracketed()?;
	// ([])*
	while self.snap(TokenType::LeftSquare).is_ok() {
	    self.snap(TokenType::RightSquare)?;
	    t = Type::Array(Box::new(t));
	}
	//
	Ok(t)
    }

    /// Parse a type which may (or may not) be bracketed.  For
    /// example, in `(&int)[]` the type `&int` is bracketed.
    pub fn parse_type_bracketed(&mut self) -> Result<Type> {
	// Try and match bracket!
	if self.snap(TokenType::LeftBrace).is_ok() {
	    // Bingo!
	    let typ_e = self.parse_type();
	    // Must match closing brace
	    self.snap(TokenType::RightBrace)?;
	    // Done
	    typ_e
	} else {
	    self.parse_type_base()
	}
    }
    
    pub fn parse_type_base(&mut self) -> Result<Type> {
	let lookahead = self.lexer.peek();
	// Look at what we've got!
	let t = match lookahead.kind {
	    TokenType::Null => {
		Type::Null
	    }
	    //
	    TokenType::Bool => {
		Type::Bool
	    }
	    //
	    TokenType::I8 => {
		Type::Int8
	    }
	    TokenType::I16 => {
		Type::Int16
	    }
	    TokenType::I32 => {
		Type::Int32
	    }
	    TokenType::I64 => {
		Type::Int64
	    }
	    //
	    TokenType::U8 => {
		Type::Uint8
	    }
	    TokenType::U16 => {
		Type::Uint16
	    }
	    TokenType::U32 => {
		Type::Uint32
	    }
	    TokenType::U64 => {
		Type::Uint64
	    }
	    //
	    TokenType::Void => {
		Type::Void
	    }
	    _ => {
		return Err(());
	    }
	};
	// Move over it
	self.lexer.next();
	//
	Ok(t)
    }
    
    // =========================================================================
    // Misc
    // =========================================================================
    
    pub fn parse_identifier(&mut self) -> Result<String> {
	let tok = self.snap(TokenType::Identifier)?;
	Ok(tok.as_string())
    }
    
    // =========================================================================
    // Helpers
    // =========================================================================        

    /// Match a given token type in the current stream.  If the kind
    /// matches, then the token stream advances.  Otherwise, it
    /// remains at the same position and an error is returned.
    fn snap(&mut self, kind : TokenType) -> Result<Token<'a>> {
	// Peek at the next token
	let t = self.lexer.peek();
	// Check it!
	if t.kind == kind {
	    // Accept it
	    self.lexer.next();
	    //
	    Ok(t)
	} else {
	    // Reject
	    Err(())
	}
    }
}

// ======================================================
// Tests
// ======================================================

#[test]
fn test_01() { 
    let d = Parser::new("type nat = i32").parse().unwrap();
    assert_eq!(d,Decl::Error);
}

#[test]
fn test_02() { 
    let d = Parser::new("type nat i8;").parse().unwrap();
    assert_eq!(d,Decl::Error);
}

#[test]
fn test_03() { 
    let d = Parser::new("type t = bool;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("t".to_string(), Type::Bool ));
}

#[test]
fn test_04() { 
    let d = Parser::new("type nat = i8;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Int8 ));
}

#[test]
fn test_05() { 
    let d = Parser::new("type nat = i16;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Int16 ));
}

#[test]
fn test_06() { 
    let d = Parser::new("type nat = i32;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Int32 ));
}

#[test]
fn test_07() { 
    let d = Parser::new("type nat = i64;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Int64 ));
}

#[test]
fn test_08() { 
    let d = Parser::new("type nat = u8;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Uint8 ));
}

#[test]
fn test_09() { 
    let d = Parser::new("type nat = u16;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Uint16 ));
}

#[test]
fn test_10() { 
    let d = Parser::new("type nat = u32;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Uint32 ));
}

#[test]
fn test_11() { 
    let d = Parser::new("type nat = u64;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Uint64 ));
}

#[test]
fn test_12() { 
    let d = Parser::new("type intarr = i32[];").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("intarr".to_string(), Array(Type::Int32)));
}

#[test]
fn test_13() { 
    let d = Parser::new("type intarrarr = i32[][];").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("intarrarr".to_string(), Array(Array(Type::Int32))));
}

#[test]
fn test_14() { 
    let d = Parser::new("type r_int = &i16;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("r_int".to_string(), Ref(Type::Int16)));
}

#[test]
fn test_15() { 
    let d = Parser::new("type r_int = &&i16;").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("r_int".to_string(), Ref(Ref(Type::Int16))));
}

#[test]
fn test_16() { 
    let d = Parser::new("type rec = {i64 f};").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("rec".to_string(), Record(&[(Type::Int64,"f".to_string())])));
}

#[test]
fn test_17() { 
    let d = Parser::new("type rec = {i64 f, u32 g};").parse().unwrap();
    let fields = [(Type::Int64,"f".to_string()), (Type::Uint32,"g".to_string())];
    assert_eq!(d,Decl::TypeAlias("rec".to_string(), Record(&fields)));
}

#[test]
fn test_18() { 
    let d = Parser::new("type piarr = (&u32)[];").parse().unwrap();
    assert_eq!(d,Decl::TypeAlias("piarr".to_string(), Array(Ref(Type::Uint32))));
}

#[test]
fn test_19() { 
    let d = Parser::new("type rec = {&i8 f, u16[] g};").parse().unwrap();
    let fields = [(Ref(Type::Int8),"f".to_string()), (Array(Type::Uint16),"g".to_string())];
    assert_eq!(d,Decl::TypeAlias("rec".to_string(), Record(&fields)));
}
