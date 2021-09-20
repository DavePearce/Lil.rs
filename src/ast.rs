use std::fmt;
use std::vec;

/// Represents a top-level declaration, such as a type alias or a
/// method declaration.
#[derive(Debug,PartialEq)]
pub enum Decl<T> {
    Error,
    TypeAlias(String,Type,T),
    Method(String,Type,Vec<Parameter>,Stmt,T)
}

/// Represents a statement in the source code of a Lil program. Many
/// standard statement kinds are provided, including `if`, `while`,
/// `for`, etc.
#[derive(Debug,PartialEq)]
pub enum Stmt {
    Assert(Expr),
    Block(Vec<Stmt>),
    If(Expr,Box<Stmt>,Option<Box<Stmt>>)
}

/// Represents an expression in the source code of a Lil program. Many
/// standard expression kinds are provided, including unary operations
/// (e.g.  `!e`, `-e`, `|e|`), binary operations (e.g.  `x==y`,
/// `x!=y`, `x+y`, etc), list expressions (e.g. `ls[i]`, `[1,2,3]`,
/// etc), record expressions (e.g. `r.f`, `{x: 1, y: 2}`, etc).
#[derive(Debug,PartialEq)]
pub enum Expr {
    Variable(String),
    BoolLiteral(bool),
    IntLiteral(i32),
    IntAddition(Box<Expr>,Box<Expr>),
}

/// Represents a type descriptor in the source code of a Lil program.
/// For example, `int` or `int|null` or `{int f}`, etc.
#[derive(Clone,Debug,PartialEq)]
pub enum Type {
    Array(Box<Type>),
    Bool,
    Null,    
    Int8,
    Int16,
    Int32,
    Int64,
    Record(Vec<(Type,String)>),
    Ref(Box<Type>),
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Void
}

// =============================================================================
// Constructors
// =============================================================================

/// Constructor for array types
pub fn Array(element : Type) -> Type {
    Type::Array(Box::new(element))
}

/// Constructor for record types
pub fn Record(fields : &[(Type,String)]) -> Type {
    Type::Record(fields.to_vec())
}

/// Constructor for reference types
pub fn Ref(target : Type) -> Type {
    Type::Ref(Box::new(target))
}

/// Represents a parameter declaration in the source of a given method.
#[derive(Clone,Debug,PartialEq)]
pub struct Parameter {
    pub declared : Type,
    pub name : String
}

// =============================================================================
// Debug
// =============================================================================

impl<T> fmt::Display for Decl<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Decl::Error => {
		write!(f,"Error()")
	    }
	    Decl::TypeAlias(s,t,_) => {
		write!(f,"Type({},{})",s,t)
	    }
	    Decl::Method(n,r,ps,b,_) => {
		let pstr = to_string(ps);
		write!(f,"Method({},{},{},{})",n,r,pstr,b)		       
	    }
	    _ => {
		write!(f,"(decl)")
	    }
	}
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Stmt::Block(ss) => {
		let body = to_string(ss);
		write!(f,"Block{}",body)
	    }
	    Stmt::Assert(e) => {
		write!(f,"Assert({})",e)
	    }
	    _ => {
		write!(f,"Stmt")
	    }
	}
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Expr::Variable(s) => {
		write!(f,"Var({})",s)
	    }
	    Expr::IntLiteral(i) => {
		write!(f,"Int({})",i)
	    }
	    Expr::BoolLiteral(b) => {
		write!(f,"Bool({})",b)
	    }
	    _ => {
		write!(f,"Expr")
	    }
	}
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Type::Array(elem) => {
		write!(f,"{}[]",elem)
	    }
	    Type::Bool => {
		write!(f,"bool")
	    }
	    Type::Null=> {
		write!(f,"null")
	    }
	    Type::Int8 => {
		write!(f,"int8")
	    }
	    Type::Int16 => {
		write!(f,"int16")
	    }
	    Type::Int32 => {
		write!(f,"int32")
	    }
	    Type::Int64 => {
		write!(f,"int64")
	    }
	    Type::Record(fs) => {
		write!(f,"{{ ??? }}")
	    }
	    Type::Ref(elem) => {
		write!(f,"&{}",elem)
	    }
	    Type::Uint8 => {
		write!(f,"uint8")
	    }
	    Type::Uint16 => {
		write!(f,"uint16")
	    }
	    Type::Uint32 => {
		write!(f,"uint32")
	    }
	    Type::Uint64 => {
		write!(f,"uint64")
	    }
	    Type::Void => {
		write!(f,"void")
	    }
	}
    }
}

fn to_string<T:fmt::Display>(items : &[T]) -> String {
    let mut s = String::new();
    s.push('[');
    for item in items {
	s.push_str(&item.to_string());
    }
    s.push(']');
    return s;
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"({},{})",self.declared,self.name)
    }
}
