use std::result;
use std::matches;
use std::convert::From;
use crate::lexer::EOF;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::ast::*;

// =================================================================
// Error
// =================================================================

/// Identifies possible errors stemming from the parser.
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
    /// Provides mechanism for source maps
    mapper : F
	
}

impl<'a,'b,F> Parser<'a,F>
where 'a :'b, F : FnMut(usize,&'a str) {
   
    pub fn new(input: &'a str, ast: &'a mut AbstractSyntaxTree, mapper : F) -> Self {
	Self { lexer: Lexer::new(input), ast, mapper }
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
	//
	let idx = self.ast.push(Node::DeclType(name),&[typ_e.index]);
	// Done
	Ok(Ref::new(&self.ast,idx))
    }

    /// Parse a method declaration of the form `Type name([Type
    /// Identifier]*) Stmt.Block`.
    pub fn parse_decl_method(&'b mut self) -> Result<Decl> {
	todo!("reinstate me");
	// // Type
	// let ret_type = self.parse_type()?;
	// // Identifier
	// let name = self.parse_identifier()?;
	// // "(" [Type Identifier]+ ")"
	// let params = self.parse_decl_parameters()?;
	// // "{" [Stmt]* "}"
	// let body = self.parse_stmt_block()?;
	// // // Apply source map
	// // //let attr = (self.mapper)("test");
	// //
	// let idx = self.ast.push(Node::DeclMethod(name,ret_type,params,body));
	// // //
	// Ok(Ref::new(&self.ast,idx))
    }

    // /// Parse a list of parameter declarations
    // pub fn parse_decl_parameters(&mut self) -> Result<Vec<Parameter>> {
    // 	let mut params : Vec<Parameter> = vec![];
    // 	// "("
    // 	self.snap(TokenType::LeftBrace)?;
    // 	// Keep going until a right brace
    // 	while self.snap(TokenType::RightBrace).is_err() {
    // 	    // Check if first time or not
    // 	    if !params.is_empty() {
    // 		// Not first time, so match comma
    // 		self.snap(TokenType::Comma)?;
    // 	    }
    // 	    // Type
    // 	    let f_type = self.parse_type()?;
    // 	    // Identifier
    // 	    let f_name = self.parse_identifier()?;
    // 	    // 
    // 	    params.push(Parameter{declared:f_type,name:f_name});
    // 	}
    // 	// Done
    // 	Ok(params)
    // }

    // =========================================================================
    // Statements
    // =========================================================================    

    // /// Parse a block of zero or more statements surrounded by curly
    // /// braces.  For example, `{ int x = 1; x = x + 1; }`.
    // pub fn parse_stmt_block(&mut self) -> Result<Stmt> {
    // 	let mut stmts : Vec<Stmt> = Vec::new();
    // 	// "{"
    // 	self.snap(TokenType::LeftCurly)?;
    // 	// Keep going until a right curly
    // 	while self.snap(TokenType::RightCurly).is_err() {
    // 	    stmts.push(self.parse_stmt()?);
    // 	}
    // 	// Allocate statement block
    // 	let index = self.ast.push(Node::StmtBlock(stmts));
    // 	// Temporary for now
    // 	Ok(Stmt{index})
    // }    
    
    // /// Parse an arbitrary statement.
    // pub fn parse_stmt(&mut self) -> Result<Stmt> {
    // 	let lookahead = self.lexer.peek();
    // 	//
    // 	match lookahead.kind {
    // 	    _ => self.parse_unit_stmt()
    // 	}
    // }

    // /// Parse a unit statement.  This one which does not contain other
    // /// statements, and is terminated with a ";".
    // pub fn parse_unit_stmt(&mut self) -> Result<Stmt> {
    // 	let lookahead = self.lexer.peek();
    // 	//
    // 	let stmt = match lookahead.kind {
    // 	    TokenType::Assert => {
    // 	    	self.parse_stmt_assert()
    // 	    }
    // 	    TokenType::Skip => {
    // 		self.parse_stmt_skip()
    // 	    }
    // 	    _ => {
    // 		return Err(Error::new(lookahead,"unknown token encountered"));
    // 	    }
    // 	};
    // 	// ";"
    // 	self.snap(TokenType::SemiColon)?;
    // 	// Done
    // 	stmt
    // }

    // pub fn parse_stmt_assert(&mut self) -> Result<Stmt> {
    // 	// "assert"
    // 	self.snap(TokenType::Assert)?;
    // 	// Expr
    // 	let expr = self.parse_expr()?;
    // 	// Allocate node
    // 	let index = self.ast.push(Node::StmtAssert(expr));
    // 	// Done
    // 	Ok(Stmt{index})	
    // }

    // pub fn parse_stmt_skip(&mut self) -> Result<Stmt> {
    // 	// "skip"
    // 	self.snap(TokenType::Skip)?;
    // 	// Allocate node
    // 	let index = self.ast.push(Node::StmtSkip);
    // 	// Done
    // 	Ok(Stmt{index})
    // }

    // =========================================================================
    // Expressions
    // =========================================================================    

    // pub fn parse_expr(&mut self) -> Result<Expr> {
    // 	self.parse_expr_term()
    // }

    // pub fn parse_expr_term(&mut self) -> Result<Expr> {
    // 	let lookahead = self.lexer.peek();
    // 	//
    // 	let index : usize = match lookahead.kind {
    // 	    TokenType::False => {
    // 		self.lexer.next();
    // 		self.ast.push(Node::ExprBool(false))
    // 	    }
    // 	    TokenType::Integer => {
    // 	    	self.lexer.next();
    // 		self.ast.push(Node::ExprInt(lookahead.as_int()))	
    // 	    }
    // 	    TokenType::LeftBrace => {
    // 	    	return self.parse_expr_bracketed()
    // 	    }
    // 	    TokenType::True => {
    // 		self.lexer.next();
    // 		self.ast.push(Node::ExprBool(true))
    // 	    }
    // 	    _ => {
    // 		return Err(Error::new(lookahead,"unknown token encountered"))
    // 	    }
    // 	};
    // 	//
    // 	Ok(Expr{index})
    // }

    // pub fn parse_expr_bracketed(&mut self) -> Result<Expr> {
    // 	// "("
    // 	self.snap(TokenType::LeftBrace)?;
    // 	// Expr
    // 	let expr = self.parse_expr();
    // 	// ")"
    // 	self.snap(TokenType::RightBrace)?;
    // 	//
    // 	expr
    // }
    
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
	    // TokenType::Ampersand => {
	    // 	// Looks like a reference type
	    // 	self.parse_type_ref()
	    // }
	    // TokenType::LeftCurly => {
	    // 	// Looks like a record type
	    // 	self.parse_type_record()
	    // }
	    // _ => {
	    // 	// Could be an array type
	    // 	self.parse_type_array()	    
	    // }
	    _ => {
		self.parse_type_base()	    
	    }	    
	}
    }

    // /// Parse a reference type, such as `&i32`, `&(i32[])`, `&&u16`,
    // /// etc.
    // pub fn parse_type_ref(&mut self) -> Result<Type> {
    // 	let mut n = 1;
    // 	// "&"
    // 	self.snap(TokenType::Ampersand)?;
    // 	// Check for nested references
    // 	while self.snap(TokenType::Ampersand).is_ok() {
    // 	    n = n + 1;
    // 	}	
    // 	// Type	
    // 	let mut t = self.parse_type_bracketed()?;	
    // 	// Unwind references
    // 	for i in 0..n {
    // 	    let index = self.ast.push(Node::TypeReference(t));
    // 	    t = Type{index};
    // 	}
    // 	// Done
    // 	Ok(t)
    // }

    // /// Parse a record type, such as `{ i32 f }`, `{ bool f, u64 f }`,
    // /// `{ &bool f, u64[] f }`, etc.
    // pub fn parse_type_record(&mut self) -> Result<Type> {
    // 	let mut fields : Vec<(Type,String)> = vec![];
    // 	// "{"
    // 	self.snap(TokenType::LeftCurly)?;
    // 	// Keep going until a right brace
    // 	while self.snap(TokenType::RightCurly).is_err() {
    // 	    // Check if first time or not
    // 	    if !fields.is_empty() {
    // 		// Not first time, so match comma
    // 		self.snap(TokenType::Comma)?;
    // 	    }
    // 	    // Type
    // 	    let f_type = self.parse_type()?;
    // 	    // Identifier
    // 	    let f_name = self.parse_identifier()?;
    // 	    // 
    // 	    fields.push((f_type,f_name));
    // 	}
    // 	// Done
    // 	let index = self.ast.push(Node::TypeRecord(fields));
    // 	Ok(Type{index})
    // }

    // /// Parse an array type, such as `i32[]`, `bool[][]`, etc.
    // pub fn parse_type_array(&'b mut self) -> Result<Type> {
    // 	// Type
    // 	let mut t = self.parse_type_bracketed()?;
    // 	// ([])*
    // 	while self.snap(TokenType::LeftSquare).is_ok() {
    // 	    self.snap(TokenType::RightSquare)?;
    // 	    let index = self.ast.push(Node::TypeArray(t));
    // 	    t = Type{index};
    // 	}
    // 	//
    // 	Ok(t)
    // }

    // /// Parse a type which may (or may not) be bracketed.  For
    // /// example, in `(&int)[]` the type `&int` is bracketed.
    // pub fn parse_type_bracketed(&'b mut self) -> Result<Type> {
    // 	// Try and match bracket!
    // 	if self.snap(TokenType::LeftBrace).is_ok() {
    // 	    // Bingo!
    // 	    let typ_e = self.parse_type()?;
    // 	    // Must match closing brace
    // 	    self.snap(TokenType::RightBrace)?;
    // 	    // Done
    // 	    Ok(typ_e)
    // 	} else {
    // 	    self.parse_type_base()
    // 	}
    // }
    
    pub fn parse_type_base(&'b mut self) -> Result<Type> {
	let lookahead = self.lexer.peek();
	// Look at what we've got!
	let index : usize = match lookahead.kind {
	    TokenType::Null => {
		self.ast.push(Node::TypeNull, NO_CHILDREN)
	    }
	    //
	    TokenType::Bool => {
		self.ast.push(Node::TypeBool, NO_CHILDREN)
	    }
	    //
	    TokenType::I8 => {
		self.ast.push(Node::TypeInt(true,8), NO_CHILDREN)
	    }
	    TokenType::I16 => {
		self.ast.push(Node::TypeInt(true,16), NO_CHILDREN)
	    }
	    TokenType::I32 => {
		self.ast.push(Node::TypeInt(true,32), NO_CHILDREN)
	    }
	    TokenType::I64 => {
		self.ast.push(Node::TypeInt(true,64), NO_CHILDREN)
	    }
	    //
	    TokenType::U8 => {
		self.ast.push(Node::TypeInt(false,8), NO_CHILDREN)
	    }
	    TokenType::U16 => {
		self.ast.push(Node::TypeInt(false,16), NO_CHILDREN)
	    }
	    TokenType::U32 => {
		self.ast.push(Node::TypeInt(false,32), NO_CHILDREN)
	    }
	    TokenType::U64 => {
		self.ast.push(Node::TypeInt(false,64), NO_CHILDREN)
	    }
	    //
	    TokenType::Void => {
		self.ast.push(Node::TypeVoid, NO_CHILDREN)
	    }
	    _ => {
		return Err(Error::new(lookahead,"unknown token encountered"));
	    }
	};
	// Move over it
	self.lexer.next();
	//
	Ok(Type{index})
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

// ======================================================
// Tests (Type DeclTypees)
// ======================================================

/// A dummy mapper which does nothing.
fn dummy<'a>(_: usize, _: &'a str) { }

#[test]
fn test_type_01() {
    check_error("type nat = i32");
}

#[test]
fn test_type_02() { 
    check_error("type nat i8;");
}

#[test]
fn test_type_03() { 
    let ast = check_parse("type t = bool;");
    assert!(matches!(ast.get(0),Node::TypeBool));
}

// #[test]
// fn test_type_04() { 
//     let d = Parser::new("type nat = i8;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Int8 ));
// }

// #[test]
// fn test_type_05() { 
//     let d = Parser::new("type nat = i16;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Int16 ));
// }

// #[test]
// fn test_type_06() { 
//     let d = Parser::new("type nat = i32;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Int32 ));
// }

// #[test]
// fn test_type_07() { 
//     let d = Parser::new("type nat = i64;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Int64 ));
// }

// #[test]
// fn test_type_08() { 
//     let d = Parser::new("type nat = u8;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Uint8 ));
// }

// #[test]
// fn test_type_09() { 
//     let d = Parser::new("type nat = u16;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Uint16 ));
// }

// #[test]
// fn test_type_10() { 
//     let d = Parser::new("type nat = u32;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Uint32 ));
// }

// #[test]
// fn test_type_11() { 
//     let d = Parser::new("type nat = u64;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("nat".to_string(), Type::Uint64 ));
// }

// #[test]
// fn test_type_12() { 
//     let d = Parser::new("type intarr = i32[];").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("intarr".to_string(), Array(Type::Int32)));
// }

// #[test]
// fn test_type_13() { 
//     let d = Parser::new("type intarrarr = i32[][];").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("intarrarr".to_string(), Array(Array(Type::Int32))));
// }

// #[test]
// fn test_type_14() { 
//     let d = Parser::new("type r_int = &i16;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("r_int".to_string(), Ref(Type::Int16)));
// }

// #[test]
// fn test_type_15() { 
//     let d = Parser::new("type r_int = &&i16;").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("r_int".to_string(), Ref(Ref(Type::Int16))));
// }

// #[test]
// fn test_type_16() { 
//     let d = Parser::new("type rec = {i64 f};").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("rec".to_string(), Record(&[(Type::Int64,"f".to_string())])));
// }

// #[test]
// fn test_type_17() { 
//     let d = Parser::new("type rec = {i64 f, u32 g};").parse().unwrap();
//     let fields = [(Type::Int64,"f".to_string()), (Type::Uint32,"g".to_string())];
//     assert_eq!(d,Decl::DeclType("rec".to_string(), Record(&fields)));
// }

// #[test]
// fn test_type_18() { 
//     let d = Parser::new("type piarr = (&u32)[];").parse().unwrap();
//     assert_eq!(d,Decl::DeclType("piarr".to_string(), Array(Ref(Type::Uint32))));
// }

// #[test]
// fn test_type_19() { 
//     let d = Parser::new("type rec = {&i8 f, u16[] g};").parse().unwrap();
//     let fields = [(Ref(Type::Int8),"f".to_string()), (Array(Type::Uint16),"g".to_string())];
//     assert_eq!(d,Decl::DeclType("rec".to_string(), Record(&fields)));
// }

// ======================================================
// Tests (DeclMethods)
// ======================================================

// #[test]
// fn test_method_01() { 
//     let d = Parser::new("voi").parse().unwrap();
//     assert_eq!(d,Decl::Error);
// }

// #[test]
// fn test_method_02() { 
//     let d = Parser::new("void").parse().unwrap();
//     assert_eq!(d,Decl::Error);
// }

// #[test]
// fn test_method_03() { 
//     let d = Parser::new("void f").parse().unwrap();
//     assert_eq!(d,Decl::Error);
// }

// #[test]
// fn test_method_04() { 
//     let d = Parser::new("void f(").parse().unwrap();
//     assert_eq!(d,Decl::Error);
// }

// #[test]
// fn test_method_05() { 
//     let d = Parser::new("void f() {").parse().unwrap();
//     assert_eq!(d,Decl::Error);
// }

// #[test]
// fn test_method_06() { 
//     let d = Parser::new("void f() {}").parse().unwrap();
//     //
//     match d {
// 	Decl::DeclMethod(n,r,ps,b) => {
// 	    assert_eq!(n,"f".to_string());
// 	    assert_eq!(r,Type::Void);
// 	    assert!(ps.is_empty());
// 	    assert_eq!(b,Stmt::Block(Vec::new()));
// 	}
// 	_ => {
// 	    panic!("Invalid match");
// 	}
//     }
// }

// #[test]
// fn test_method_07() { 
//     let d = Parser::new("bool f(i32 x) {}").parse().unwrap();
//     //
//     match d {
// 	Decl::DeclMethod(n,r,ps,b) => {
// 	    assert_eq!(n,"f".to_string());
// 	    assert_eq!(r,Type::Bool);
// 	    assert!(ps.len() == 1);
// 	    assert!(ps[0] == (Type::Int32,"x".to_string()));
// 	    assert_eq!(b,Stmt::Block(Vec::new()));
// 	}
// 	_ => {
// 	    panic!("Invalid match");
// 	}
//     }
// }

// #[test]
// fn test_method_08() { 
//     let d = Parser::new("bool f(i32 i, bool b) {}").parse().unwrap();
//     //
//     match d {
// 	Decl::DeclMethod(n,r,ps,b) => {
// 	    assert_eq!(n,"f".to_string());
// 	    assert_eq!(r,Type::Bool);
// 	    assert!(ps.len() == 2);
// 	    assert!(ps[0] == (Type::Int32,"i".to_string()));
// 	    assert!(ps[1] == (Type::Bool,"b".to_string()));
// 	    assert_eq!(b,Stmt::Block(Vec::new()));
// 	}
// 	_ => {
// 	    panic!("Invalid match");
// 	}
//     }
// }

// #[test]
// fn test_method_09() { 
//     let d = Parser::new("void f() { assert true; }").parse().unwrap();
//     assert!(matches!(d, Decl::DeclMethod { .. }));
// }

// ======================================================
// Tests (Statements)
// ======================================================

// #[test]
// fn test_stmt_01() { 
//     let s = Parser::new("asse").parse_stmt();
//     assert!(s.is_err());
// }

// #[test]
// fn test_stmt_02() { 
//     let s = Parser::new("assert").parse_stmt();
//     assert!(s.is_err());
// }

// #[test]
// fn test_stmt_03() { 
//     let s = Parser::new("assert true").parse_stmt();
//     assert!(s.is_err());
// }

// #[test]
// fn test_stmt_04() {
//     let s = Parser::new("assert true;").parse_stmt();
//     assert!(s.is_ok());
// }

// #[test]
// fn test_stmt_05() {
//     let s = Parser::new("assert false;").parse_stmt();
//     assert!(s.is_ok());
// }

#[cfg(test)]
fn check_parse(input: &str) -> Box<AbstractSyntaxTree> {
    let mut ast = AbstractSyntaxTree::new();
    let mut p = Parser::new(input,&mut ast, dummy);
    let d = p.parse_decl();
    assert!(!d.is_err());
    Box::new(ast)
}

#[cfg(test)]
fn check_error(input: &str) {
    let mut ast = AbstractSyntaxTree::new();
    let mut p = Parser::new(input,&mut ast, dummy);
    let d = p.parse_decl();
    assert!(d.is_err());
}
