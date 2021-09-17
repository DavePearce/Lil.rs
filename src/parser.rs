use std::result;
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
	// Attempt to parse declaration
	let r = match lookahead.kind {
	    TokenType::Type => {
		self.parse_type_decl()
	    }
	    _ => {
		// Token stream is empty, so nothing to return.
		return None;
	    }
	};
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

    /// Parse a type declaration of the from `type name is type;`.
    pub fn parse_type_decl(&mut self) -> Result<Decl> {
	// "type"
	let tok = self.snap(TokenType::Type)?;
	// Identifier
	let ident : Token<'a> = self.snap(TokenType::Identifier)?;
	// "is"
	self.snap(TokenType::Is)?;
	// Type
	let typ_e : Type = self.parse_type()?;
	// Semi-colon
	let _ = self.snap(TokenType::SemiColon)?;
	//
	Ok(Decl::TypeAlias(ident.as_string(),typ_e))
    }

    pub fn parse_type(&mut self) -> Result<Type> {
	let lookahead = self.lexer.peek();
	// Look at what we've got!
	let t = match lookahead.kind {
	    TokenType::Null => {
		Type::Null
	    }
	    TokenType::Bool => {
		Type::Bool
	    }	    
	    TokenType::Int => {
		Type::Int
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
    let mut d = Parser::new("type nat is int;").parse().unwrap();
    // Check 
    assert_eq!(d,Decl::TypeAlias("nat".to_string(), Type::Int ));
}
