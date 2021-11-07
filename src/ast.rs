use std::fmt;
use std::vec;
use std::convert::From;
use syntactic_heap::SyntacticHeap;
use syntactic_heap::Ref;
use syntactic_heap::ToRef;

pub const NO_CHILDREN : &[usize] = &[];

#[derive(Clone,Debug,PartialEq)]
pub struct Decl { pub index: usize }

#[derive(Clone,Debug,PartialEq)]
pub struct Stmt { pub index: usize }

#[derive(Clone,Debug,PartialEq)]
pub struct Expr { pub index: usize }

#[derive(Clone,Debug,PartialEq)]
pub struct Type { pub index: usize }

/// Represents a parameter declaration in the source of a given method.
#[derive(Clone,Debug,PartialEq)]
pub struct Parameter {
    pub declared : Type,
    pub name : String
}

#[derive(Clone,Debug,PartialEq)]
pub enum Node {
    // Declarations
    DeclType(String),
    // DeclMethod(String,Type,Vec<Parameter>,Stmt),
    // Statements
    // StmtAssert(Expr),
    // StmtBlock(Vec<Stmt>),
    // StmtSkip,    
    // Expressions
    // ExprBool(bool),
    // ExprInt(i32),    
    // Types
    // TypeArray(Type),
    TypeBool,
    TypeInt(bool,u8),
    TypeNull,
    // TypeRecord(Vec<(Type,String)>),
    // TypeReference(Type),    
    TypeVoid
}

// =============================================================================
// Node References
// =============================================================================

impl From<Ref<'_,Node>> for Decl {
    fn from(r: Ref<'_,Node>) -> Decl {
	Decl{index:r.index}
    }
}

impl From<Ref<'_,Node>> for Stmt {
    fn from(r: Ref<'_,Node>) -> Stmt {
	Stmt{index:r.index}
    }
}

impl From<Ref<'_,Node>> for Type {
    fn from(r: Ref<'_,Node>) -> Type {
	Type{index:r.index}
    }
}

impl ToRef for Decl {
    fn to_ref<'a>(&self, parent: &'a AbstractSyntaxTree) -> Ref<'a,Node> {
	Ref::new(parent,self.index)
    }
}

impl ToRef for Stmt {
    fn to_ref<'a>(&self, parent: &'a AbstractSyntaxTree) -> Ref<'a,Node> {
	Ref::new(parent,self.index)
    }
}

impl ToRef for Expr {
    fn to_ref<'a>(&self, parent: &'a AbstractSyntaxTree) -> Ref<'a,Node> {
	Ref::new(parent,self.index)
    }
}

impl ToRef for Type {
    fn to_ref<'a>(&self, parent: &'a AbstractSyntaxTree) -> Ref<'a,Node> {
	Ref::new(parent,self.index)
    }
}

// =============================================================================
// Abstract Syntax Tree
// =============================================================================

pub type AbstractSyntaxTree = SyntacticHeap<Node>;
 
// =============================================================================
// Debug
// =============================================================================


fn to_string<T:fmt::Display>(items : &[T]) -> String {
    let mut s = String::new();
    let mut f = true;
    s.push('[');    
    for item in items {
	if !f { s.push(','); }
	f = false;
	s.push_str(&item.to_string());
    }
    s.push(']');
    return s;
}
