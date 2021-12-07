use std::result;
use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::ast::*;

/// The parsing environment maps raw strings to on-tree names.
type Env = HashMap<String, Name>;

// =================================================================
// Error
// =================================================================

/// Identifies possible errors stemming from the parser.
#[derive(Debug)]
pub struct Error {
    pub start: usize,
    pub end: usize,
    pub message: &'static str
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn new<'a>(tok: Token<'a>, message: &'static str) -> Error {
	let start = tok.start;
	let end = tok.end();
	Error{start,end,message}
    }
}

// =================================================================
// Parser
// =================================================================

/// Response for turning a stream of tokens into an Abstract Syntax
/// Tree and/or producing error messages along the way.
pub struct Parser<'a, F>
where F : FnMut(usize,&'a str) {
    /// Provides access to our token stream.
    lexer: Lexer<'a>,
    /// Provides access to the ast
    ast: &'a mut AbstractSyntaxTree,
    /// Provides name cache
    env: Env,
    /// Provides mechanism for source maps
    mapper : F

}

impl<'a,'b,F> Parser<'a,F>
where 'a :'b, F : FnMut(usize,&'a str) {

    pub fn new(input: &'a str, ast: &'a mut AbstractSyntaxTree, mapper : F) -> Self {
	let env : Env = HashMap::new();
	Self { lexer: Lexer::new(input), ast, env, mapper }
    }

    // =========================================================================
    // Accessors / Mutators
    // =========================================================================

    // =========================================================================
    // Declarations
    // =========================================================================

    /// Parse an arbitrary declaration
    pub fn parse_decl(&'b mut self) -> Result<Decl> {
	let lookahead = self.lexer.peek();
	// Attempt to parse declaration
	match lookahead.kind {
	    TokenType::Type => {
		self.parse_decl_type()
	    }
	    _ => {
		self.parse_decl_method()
	    }
	}
    }

    /// Parse a type declaration of the from `type name is type;`.
    pub fn parse_decl_type(&'b mut self) -> Result<Decl> {
	// "type"
	let start = self.snap(TokenType::Type)?;
	// Identifier
	let name = self.parse_identifier()?;
	// "="
	self.snap(TokenType::Equal)?;
	// Type
	let typ_e = self.parse_type()?;
	// Semi-colon
	let end = self.snap(TokenType::SemiColon)?;
	// Extract corresponding (sub)slice
	let slice = &self.lexer.input[start.start .. end.end()];
	// Apply source map
	//let attr = (self.mapper)(slice);
	// Done
	Ok(Decl::new(self.ast,Node::TypeDecl(name,typ_e)))
    }

    /// Parse a method declaration of the form `Type name([Type
    /// Identifier]*) Stmt.Block`.
    pub fn parse_decl_method(&'b mut self) -> Result<Decl> {
	// Type
	let ret_type = self.parse_type()?;
	// Identifier
	let name = self.parse_identifier()?;
	// "(" [Type Identifier]+ ")"
	let params = self.parse_decl_parameters()?;
	// "{" [Stmt]* "}"
	let body = self.parse_stmt_block()?;
	// // Apply source map
	// //let attr = (self.mapper)("test");
	//
	Ok(Decl::new(self.ast,Node::MethodDecl(name,ret_type,params,body)))
    }

    /// Parse a list of parameter declarations
    pub fn parse_decl_parameters(&mut self) -> Result<Vec<Parameter>> {
    	let mut params : Vec<Parameter> = vec![];
    	// "("
    	self.snap(TokenType::LeftBrace)?;
    	// Keep going until a right brace
    	while self.snap(TokenType::RightBrace).is_err() {
    	    // Check if first time or not
    	    if !params.is_empty() {
    		// Not first time, so match comma
    		self.snap(TokenType::Comma)?;
    	    }
    	    // Type
    	    let f_type = self.parse_type()?;
    	    // Identifier
    	    let f_name = self.parse_identifier()?;
    	    //
    	    params.push(Parameter{declared:f_type,name:f_name});
    	}
    	// Done
    	Ok(params)
    }

    // =========================================================================
    // Statements
    // =========================================================================

    /// Parse a block of zero or more statements surrounded by curly
    /// braces.  For example, `{ int x = 1; x = x + 1; }`.
    pub fn parse_stmt_block(&mut self) -> Result<Stmt> {
    	let mut stmts : Vec<Stmt> = Vec::new();
    	// "{"
    	self.snap(TokenType::LeftCurly)?;
    	// Keep going until a right curly
    	while self.snap(TokenType::RightCurly).is_err() {
    	    stmts.push(self.parse_stmt()?);
    	}
    	// Done
    	Ok(Stmt::new(self.ast,Node::BlockStmt(stmts)))
    }

    /// Parse an arbitrary statement.
    pub fn parse_stmt(&mut self) -> Result<Stmt> {
    	let lookahead = self.lexer.peek();
    	//
    	match lookahead.kind {
    	    _ => self.parse_unit_stmt()
    	}
    }

    /// Parse a unit statement.  This one which does not contain other
    /// statements, and is terminated with a ";".
    pub fn parse_unit_stmt(&mut self) -> Result<Stmt> {
    	let lookahead = self.lexer.peek();
    	//
    	let stmt = match lookahead.kind {
    	    TokenType::Assert => {
    	    	self.parse_stmt_assert()
    	    }
    	    TokenType::Skip => {
    		self.parse_stmt_skip()
    	    }
    	    _ => {
    		return Err(Error::new(lookahead,"unknown token encountered"));
    	    }
    	};
    	// ";"
    	self.snap(TokenType::SemiColon)?;
    	// Done
    	stmt
    }

    pub fn parse_stmt_assert(&mut self) -> Result<Stmt> {
    	// "assert"
    	self.snap(TokenType::Assert)?;
    	// Expr
    	let expr = self.parse_expr()?;
    	// Done
    	Ok(Stmt::new(self.ast,Node::AssertStmt(expr)))
    }

    pub fn parse_stmt_skip(&mut self) -> Result<Stmt> {
    	// "skip"
    	self.snap(TokenType::Skip)?;
    	// Done
    	Ok(Stmt::new(self.ast,Node::SkipStmt))
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    pub fn parse_expr(&mut self) -> Result<Expr> {
    	let lhs = self.parse_expr_term()?;
	// Check for binary expression
    	let lookahead = self.lexer.peek();
	//
	match lookahead.kind {
	    TokenType::LeftAngle => {
		self.lexer.next();
		let rhs = self.parse_expr_term()?;
		Ok(Expr::new(self.ast,Node::LessThanExpr(lhs,rhs)))
	    }
	    _ => {
		Ok(lhs)
	    }
	}
    }

    pub fn parse_expr_term(&mut self) -> Result<Expr> {
    	let lookahead = self.lexer.peek();
    	//
    	let expr = match lookahead.kind {
    	    TokenType::False => {
    		self.lexer.next();
    		Expr::new(self.ast,Node::BoolExpr(false))
    	    }
	    TokenType::Identifier => {
		let n = self.parse_identifier();
		Expr::new(self.ast,Node::VarExpr(n.unwrap()))
	    }
    	    TokenType::Integer => {
    	    	self.lexer.next();
		Expr::new(self.ast,Node::IntExpr(lookahead.as_int()))
    	    }
    	    TokenType::LeftBrace => {
    	    	return self.parse_expr_bracketed()
    	    }
    	    TokenType::True => {
    		self.lexer.next();
    		Expr::new(self.ast,Node::BoolExpr(true))
    	    }
    	    _ => {
    		return Err(Error::new(lookahead,"unknown token encountered"))
    	    }
    	};
    	//
    	Ok(expr)
    }

    pub fn parse_expr_bracketed(&mut self) -> Result<Expr> {
    	// "("
    	self.snap(TokenType::LeftBrace)?;
    	// Expr
    	let expr = self.parse_expr();
    	// ")"
    	self.snap(TokenType::RightBrace)?;
    	//
    	expr
    }

    // =========================================================================
    // Types
    // =========================================================================

    pub fn parse_type(&mut self) -> Result<Type> {
	self.parse_type_compound()
    }

    pub fn parse_type_compound(&mut self) -> Result<Type> {
	let lookahead = self.lexer.peek();
	// Attemp to distinguish
	match lookahead.kind {
	    TokenType::EOF => {
		// Something went wrong
		Err(Error::new(lookahead,"unexpected end-of-file"))
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

    /// Parse a reference type, such as `&i32`, `&(i32[])`, `&&u16`,
    /// etc.
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
            t = Type::new(self.ast,Node::ReferenceType(t));
    	}
    	// Done
    	Ok(t)
    }

    /// Parse a record type, such as `{ i32 f }`, `{ bool f, u64 f }`,
    /// `{ &bool f, u64[] f }`, etc.
    pub fn parse_type_record(&mut self) -> Result<Type> {
    	let mut fields : Vec<(Type,Name)> = vec![];
    	// "{"
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
    	Ok(Type::new(self.ast,Node::RecordType(fields)))
    }

    /// Parse an array type, such as `i32[]`, `bool[][]`, etc.
    pub fn parse_type_array(&'b mut self) -> Result<Type> {
    	// Type
    	let mut t = self.parse_type_bracketed()?;
    	// ([])*
    	while self.snap(TokenType::LeftSquare).is_ok() {
    	    self.snap(TokenType::RightSquare)?;
            t = Type::new(self.ast,Node::ArrayType(t));
    	}
    	//
    	Ok(t)
    }

    /// Parse a type which may (or may not) be bracketed.  For
    /// example, in `(&int)[]` the type `&int` is bracketed.
    pub fn parse_type_bracketed(&'b mut self) -> Result<Type> {
    	// Try and match bracket!
    	if self.snap(TokenType::LeftBrace).is_ok() {
    	    // Bingo!
    	    let typ_e = self.parse_type()?;
    	    // Must match closing brace
    	    self.snap(TokenType::RightBrace)?;
    	    // Done
    	    Ok(typ_e)
    	} else {
    	    self.parse_type_base()
    	}
    }

    pub fn parse_type_base(&'b mut self) -> Result<Type> {
	let lookahead = self.lexer.peek();
	// Look at what we've got!
	let typ_e : Type = match lookahead.kind {
	    TokenType::Null => {
		Type::new(self.ast,Node::NullType)
	    }
	    //
	    TokenType::Bool => {
                Type::new(self.ast,Node::BoolType)
	    }
	    //
	    TokenType::I8 => {
                Type::new(self.ast,Node::IntType(true,8))
	    }
	    TokenType::I16 => {
                Type::new(self.ast,Node::IntType(true,16))
	    }
	    TokenType::I32 => {
                Type::new(self.ast,Node::IntType(true,32))
	    }
	    TokenType::I64 => {
                Type::new(self.ast,Node::IntType(true,64))
	    }
	    //
	    TokenType::U8 => {
                Type::new(self.ast,Node::IntType(false,8))
	    }
	    TokenType::U16 => {
                Type::new(self.ast,Node::IntType(false,16))
	    }
	    TokenType::U32 => {
                Type::new(self.ast,Node::IntType(false,32))
	    }
	    TokenType::U64 => {
                Type::new(self.ast,Node::IntType(false,64))
	    }
	    //
	    TokenType::Void => {
                Type::new(self.ast,Node::VoidType)
	    }
	    _ => {
		return Err(Error::new(lookahead,"unknown token encountered"));
	    }
	};
	// Move over it
	self.lexer.next();
	//
	Ok(typ_e)
    }

    // =========================================================================
    // Misc
    // =========================================================================

    pub fn parse_identifier(&mut self) -> Result<Name> {
	let tok = self.snap(TokenType::Identifier)?;
	// FIXME: should employ cache!
	Ok(Name::new(self.ast,&tok.content))
    }

    // =========================================================================
    // Helpers
    // =========================================================================

    // fn source_attr(&self, first : Token<'a>, last: Token<'a>) -> Attributes {
    // 	let start = first.start;
    // 	let end = last.end();
    // 	Attributes{start,end}
    // }

    /// Match a given token type in the current stream.  If the kind
    /// matches, then the token stream advances.  Otherwise, it
    /// remains at the same position and an error is returned.
    fn snap(&mut self, kind : TokenType) -> Result<Token<'a>> {
	// Peek at the next token
	let lookahead = self.lexer.peek();
	// Check it!
	if lookahead.kind == kind {
	    // Accept it
	    self.lexer.next();
	    //
	    Ok(lookahead)
	} else {
	    // Reject
	    Err(Error::new(lookahead,"expected one thing, found another"))
	}
    }
}
