use std::result;
use std::collections::HashMap;
use syntactic_heap::Ref;
use crate::ast::*;
use crate::ast::Node::*;
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
pub struct TypeChecker<'a,F>
where F : FnMut(usize,Type) {
    ast: &'a mut AbstractSyntaxTree,
    globals : Env,
    mapper : F
}

impl<'a,F> TypeChecker<'a,F>
where F : FnMut(usize,Type) {

    pub fn new(ast: &'a mut AbstractSyntaxTree, mapper: F) -> Self {
	let globals : Env = HashMap::new();
	TypeChecker{ast,globals,mapper}
    }

    // Declarations
    // -----------------------------------------------------------------
    pub fn check(&mut self, d : Decl) -> Result<()> {
	let n = self.ast.get(d.index);
	//
	match n {
	    Node::TypeDecl(name,alias) => {
	    	self.check_type_alias(name,*alias)
	    }
	    Node::MethodDecl(name,ret,params,body) => {
		// FIXME: would be nice to avoid cloning here!  To do
		// this, I think the most sensible approach is to put
		// a collection kind into the AST.		
	    	self.check_method(name.clone(),*ret,params.clone(),*body)
	    }
	    _ => Err(internal_failure(0,"unknown declaration"))
	}
    }

    pub fn check_type_alias(&self, name : &String, alias : Type) -> Result<()> {
	// Sanity check alias type
	self.check_type(&alias)?;
	// Done!
	Ok(())
    }

    pub fn check_method(&mut self, name : String, ret: Type, params : Vec<Parameter>, body : Stmt) -> Result<()> {
    	// Clone environment, since we're going to update it.
    	let mut env = self.globals.clone();
    	// Allocate parameters into environment
    	for p in params {
    	    env.insert(p.name.clone(),p.declared);
    	}
    	// Check the body
    	let nbody = self.check_stmt(&env, body)?;
    	// Done
    	Ok(())
    }

    // Statements
    // -----------------------------------------------------------------
    
    /// Check a given statement makes sense.  More specifically, that
    /// all expressions are used in a type-safe fashion.  For example,
    /// a statement `assert 1;` is not type safe.
    pub fn check_stmt(&mut self, env : &Env, stmt : Stmt) -> Result<()> {
	let n = self.ast.get(stmt.index);
	//
	match n {
	    Node::AssertStmt(cond) => {
		self.check_assert(env,*cond)
	    }
	    Node::BlockStmt(stmts) => {
		// FIXME: would be nice to avoid cloning here!  To do
		// this, I think the most sensible approach is to put
		// a collection kind into the AST.
		self.check_block(env,stmts.clone())
	    }
	    Node::SkipStmt => {
		self.check_skip(env)
	    }
	    _ => Err(internal_failure(0,"unknown statement"))
	}
    }

    pub fn check_assert(&mut self, env : &Env, cond : Expr) -> Result<()> {
	let t = self.check_expr(env,cond)?;
	// Ensure boolean condition
	self.check_bool_type(t)?;
	//
	Ok(())
    }

    pub fn check_block(&mut self, env : &Env, stmts: Vec<Stmt>) -> Result<()> {
	for stmt in stmts {
	    self.check_stmt(env,stmt)?;
	}
	Ok(())
    }

    pub fn check_skip(&self, env : &Env) -> Result<()> {
	Ok(())
    }
    
    // Expressions
    // -----------------------------------------------------------------
    
    pub fn check_expr(&mut self, env : &Env, expr : Expr) -> Result<Type> {
	let n = self.ast.get(expr.index);
	//
	match n {
	    Node::BoolExpr(lit) => {
		self.check_boolean_literal(env,*lit)
	    }
	    Node::IntExpr(lit) => {
		self.check_integer_literal(env,*lit)
	    }	   
	    Node::LessThanExpr(lhs,rhs) => {
		self.check_lessthan_comparator(env,*lhs,*rhs)
	    }
	    Node::VarExpr(name) => {
		self.check_variable_access(env,name)
	    }
	    _ => Err(internal_failure(expr.index, "unknown expression"))
	}
	// FIXME: how do we record type?
    }

    pub fn check_boolean_literal(&mut self, env : &Env, literal: bool) -> Result<Type> {
	Ok(Type::new(self.ast,Node::BoolType))
    }

    pub fn check_integer_literal(&mut self, env : &Env, literal: i32) -> Result<Type> {
	// FIXME: for now this is a conservative assumption.
	Ok(Type::new(self.ast,Node::IntType(true,32)))
    }

    pub fn check_lessthan_comparator(&mut self, env : &Env, lhs: Expr, rhs: Expr) -> Result<Type> {
	let lhs_t = self.check_expr(env,lhs)?;
	let rhs_t = self.check_expr(env,rhs)?;
	// Check lhs is integer (of some kind)
	self.check_int_type(lhs_t)?;
	// Check rhs has matching type
	self.check_matching_types(&lhs_t, &rhs_t)?;
	// Done
	Ok(Type::new(self.ast,Node::BoolType))	
    }

    pub fn check_variable_access(&self, env : &Env, name: &String) -> Result<Type> {
	let r = env.get(name);
	//
	match r {
	    Some(t) => Ok(*t),
	    None => {
		panic!("variable not found");
	    }
	}
    }
    
    // Types
    // -----------------------------------------------------------------
    
    /// Check a declared type makes sense.  For example, if a compound
    /// type contains a nominal type which is unknown.
    pub fn check_type(&self, t : &Type) -> Result<()> {
	let n = self.ast.get(t.index);
	//
	match n {
	    // Primitives all fine
	    BoolType => { Ok(()) }
	    NullType => { Ok(()) }
	    IntType(_,_) => { Ok(()) }
	    VoidType  => { Ok(()) }
	    // Compounds depend on element
	    ArrayType(bt) => {
		self.check_type(&bt)
	    }
	    ReferenceType(bt) => {
	    	self.check_type(&bt)
	    }
	    RecordType(fields) => {
	    	for (t,n) in fields {
	    	    self.check_type(&t)?;
	    	}
	    	Ok(())
	    }
	    _ => {
		panic!("do something");
	    }
	}
    }

    /// Check two types have identical structure.
    pub fn check_matching_types(&self, t1 : &Type, t2 : &Type) -> Result<()> {
	let n1 : &Node = self.ast.get(t1.index);
	let n2 : &Node = self.ast.get(t2.index);	
	//
	match (n1,n2) {
	    // Primitives all fine
	    (BoolType, BoolType) => { Ok(()) }
	    (NullType, NullType) => { Ok(()) }
	    (IntType(b1,w1), IntType(b2,w2)) if (b1 == b2 && w1 == w2) => { Ok(()) }
	    (VoidType, VoidType) => { Ok(()) }
	    // Compounds depend on elements
	    (ArrayType(e1), ArrayType(e2)) => {
		self.check_matching_types(e1,e2)
	    }
	    (ReferenceType(e1), ReferenceType(e2)) => {
		self.check_matching_types(e1,e2)
	    }
	    _ => {
		panic!("do something");
	    }
	}
    }
 
    
    /// Check a given type is a boolean type.
    pub fn check_bool_type(&self, t : Type) -> Result<()> {
	let n = self.ast.get(t.index);
	//	
	match n {
	    // Primitives all fine
	    BoolType => { Ok(()) }
	    _ => {
		panic!("error, expected boolean");
	    }
	}
    }

    /// Check a given type is an integer type.
    pub fn check_int_type(&self, t : Type) -> Result<()> {
	let n = self.ast.get(t.index);
	//	
	match n {
	    // Primitives all fine
	    IntType(_,_) => { Ok(()) }
	    _ => {
		panic!("error, expected integer");
	    }
	}
    }
}
