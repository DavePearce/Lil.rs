use crate::lexer::Lexer;
use crate::ast::*;

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
    pub fn new(lexer: Lexer<'a>) -> Self {
	Self { lexer }
    }
    
    /// Parse a declaration from token stream.  This returns `None`
    /// when the end of the stream is reached.
    pub fn parse(&mut self) -> Option<Decl> {
	let lookahead = self.lexer.peek();
	None
    }
}
