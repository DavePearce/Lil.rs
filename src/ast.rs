use std::fmt;
use std::vec;
use std::convert::From;
use syntactic_heap::SyntacticHeap;
use syntactic_heap::Ref;

// =============================================================================
// Abstract Syntax Tree
// =============================================================================

pub type AbstractSyntaxTree = SyntacticHeap<Node>;

// =============================================================================
// Terms
// =============================================================================

#[derive(Clone,Debug,PartialEq)]
pub enum Node {
    // Declarations
    TypeDecl(String,Type),
    MethodDecl(String,Type,Vec<Parameter>,Stmt),
    // Statements
    // StmtAssert(Expr),
    // StmtBlock(Vec<Stmt>),
    // StmtSkip,
    // Expressions
    // ExprBool(bool),
    // ExprInt(i32),
    // Types
    ArrayType(Type),
    BoolType,
    IntType(bool,u8),
    NullType,
    RecordType(Vec<(Type,String)>),
    ReferenceType(Type),
    VoidType
}

// =============================================================================
// Declarations
// =============================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct Decl { pub index: usize }

/// Represents a parameter declaration in the source of a given method.
#[derive(Clone,Debug,PartialEq)]
pub struct Parameter {
    pub declared : Type,
    pub name : String
}

impl Decl {
    pub fn new(ast: &mut AbstractSyntaxTree, t : Node) -> Self {
        // Sanity check is declaration
        assert!(Decl::is(&t));
        // Create new node
        let index = ast.push(t).raw_index();
        // Done
        Decl{index}
    }

    /// Determine whether a given term is a declaration or not.
    pub fn is(t: &Node) -> bool {
        match t {
	    Node::MethodDecl(_,_,_,_) => true,	    
            Node::TypeDecl(_,_) => true,
            _ => false
        }
    }
}

// =============================================================================
// Statements
// =============================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct Stmt { pub index: usize }

// =============================================================================
// Expressions
// =============================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct Expr { pub index: usize }

// =============================================================================
// Types
// =============================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct Type { pub index: usize }


impl Type {
    pub fn new(ast: &mut AbstractSyntaxTree, t : Node) -> Self {
        // Sanity check is declaration
        assert!(Type::is(ast,&t));
        // Create new node
        let index = ast.push(t).raw_index();
        // Done
        Type{index}
    }

    /// Determine whether a given term is a type (or not).
    pub fn is(ast: &AbstractSyntaxTree, t: &Node) -> bool {
        match t {
            Node::BoolType => true,
            Node::IntType(_,_) => true,
            Node::NullType => true,
            Node::VoidType => true,
            Node::ArrayType(t) => Type::is(ast,ast.get(t.index)),
            Node::ReferenceType(t) => Type::is(ast,ast.get(t.index)),
            Node::RecordType(fs) => {
                for (t,s) in fs {
                    if !Type::is(ast,ast.get(t.index)) {
                        return false;
                    }
                }
                return true;
            }
            _ => false
        }
    }
}

// =============================================================================
// Conversions
// =============================================================================

impl From<Ref<'_,Node>> for Decl {
    fn from(r: Ref<'_,Node>) -> Decl {
	Decl{index:r.raw_index()}
    }
}

impl From<Ref<'_,Node>> for Stmt {
    fn from(r: Ref<'_,Node>) -> Stmt {
	Stmt{index:r.raw_index()}
    }
}

impl From<Ref<'_,Node>> for Type {
    fn from(r: Ref<'_,Node>) -> Type {
	Type{index:r.raw_index()}
    }
}

// =============================================================================
// Debug
// =============================================================================

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::TypeDecl(n,t) => {
                write!(f,"TypeDecl({},{})",n,t.index)
            }
            Node::ArrayType(t) => {
                write!(f,"ArrayType({})",t.index)
            }
            // Default for those without children
            _ => write!(f,"{:?}",self)
        }
    }
}



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
