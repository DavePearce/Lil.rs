use std::result;
use std::collections::HashMap;
use crate::ast::*;
use crate::error::*;

// =================================================================
// Error
// =================================================================

pub type Result<T> = result::Result<T, SyntaxError>;

// =================================================================
// Type Checker
// =================================================================

pub type Env = HashMap<String, Type>;

/// Responsible for determine appropriate types for all statements and
/// expressions used within a given AST.
pub struct TypeChecker<F>
where F : FnMut(usize,Type) {
    globals : Env,
    mapper : F
}

impl<F> TypeChecker<F>
where F : FnMut(usize,Type) {

    pub fn new(mapper: F) -> Self {
	let globals : Env = HashMap::new();
	TypeChecker{globals,mapper}
    }

    // Declarations
    // -----------------------------------------------------------------
    
    pub fn check(&self, d : &Decl) -> Result<()> {
	match d {
	    Decl::TypeAlias(name,alias) => {
		self.check_type_alias(name,alias)
	    }
	    Decl::Method(name,ret,params,body) => {
	    	self.check_method(name,ret,params,body)
	    }
	    _ => Err(internal_failure(0,"unknown declaration"))
	}
    }
    
    pub fn check_type_alias(&self, name : &String, alias : &Type) -> Result<()> {
	// Sanity check alias type
	self.check_type(alias)?;
	// Done!
	Ok(())
    }

    pub fn check_method(&self, name : &String, ret: &Type, params : &Vec<Parameter>, body : &Stmt) -> Result<()> {
    	// Clone environment, since we're going to update it.
    	let mut env = self.globals.clone();
    	// Allocate parameters into environment
    	for p in params {
    	    env.insert(p.name.clone(),p.declared.clone());
    	}
    	// Check the body
    	let nbody = self.check_stmt(&env, &body)?;
    	// Done
    	Ok(())
    }

    // Statements
    // -----------------------------------------------------------------
    
    /// Check a given statement makes sense.  More specifically, that
    /// all expressions are used in a type-safe fashion.  For example,
    /// a statement `assert 1;` is not type safe.
    pub fn check_stmt(&self, env : &Env, stmt : &Stmt) -> Result<()> {
	match stmt {
	    Stmt::Assert(cond) => {
		self.check_assert(env,&cond)
	    }
	    Stmt::Block(stmts) => {
		self.check_block(env,&stmts)
	    }
	    _ => Err(internal_failure(0,"unknown statement"))
	}
    }

    pub fn check_assert(&self, env : &Env, cond : &Expr) -> Result<()> {
	let t = self.check_expr(env,cond)?;
	// Ensure boolean condition
	self.check_subtype(&Type::Bool,&t)?;
	//
	Ok(())
    }

    pub fn check_block(&self, env : &Env, stmts: &Vec<Stmt>) -> Result<()> {
	for stmt in stmts {
	    self.check_stmt(env,stmt)?;
	}
	Ok(())
    }
    
    // Expressions
    // -----------------------------------------------------------------
    
    pub fn check_expr(&self, env : &Env, expr : &Expr) -> Result<Type> {
	match expr {
	    Expr::Variable(name) => {
		self.check_variable(env,&name)
	    }
	    Expr::BoolLiteral(lit) => {
		self.check_boolean_literal(env,lit)
	    }
	    _ => Err(internal_failure(0,"unknown expression"))
	}
    }

    pub fn check_variable(&self, env : &Env, name: &String) -> Result<Type> {
	println!("CHECK VAR {}",name);
	Ok(Type::Int32)
    }

    pub fn check_boolean_literal(&self, env : &Env, literal: &bool) -> Result<Type> {
	println!("GOT HERE");
	Ok(Type::Bool)
    }

    // Types
    // -----------------------------------------------------------------
    
    /// Check a declared type makes sense.  For example, if a compound
    /// type contains a nominal type which is unknown.
    pub fn check_type(&self, t : &Type) -> Result<()> {
	match t {
	    // Primitives all fine
	    Type::Bool => { Ok(()) }
	    Type::Null => { Ok(()) }
	    Type::Int8 => { Ok(()) }
	    Type::Int16  => { Ok(()) }
	    Type::Int32 => { Ok(()) }
	    Type::Int64 => { Ok(()) }
	    Type::Uint8 => { Ok(()) }
	    Type::Uint16 => { Ok(()) }
	    Type::Uint32 => { Ok(()) }
	    Type::Uint64 => { Ok(()) }
	    Type::Void  => { Ok(()) }
	    // Compounds depend on element
	    Type::Array(bt) => {
		self.check_type(bt)
	    }
	    Type::Ref(bt) => {
	    	self.check_type(bt)
	    }
	    Type::Record(fields) => {
	    	for (t,n) in fields {
	    	    self.check_type(t)?;
	    	}
	    	Ok(())
	    }
	}
    }

    /// Check that one type (`sub`) is a subtype of another (`sup`).
    pub fn check_subtype(&self, sup : &Type, sub: &Type) -> Result<()> {
	if sup == sub {
	    Ok(())
	} else {
	    Err(expected_subtype(0))
	}
    }
}
