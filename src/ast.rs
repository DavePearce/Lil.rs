use std::fmt;
use std::vec;
use std::convert::From;

#[derive(Clone,Debug,PartialEq)]
pub struct Stmt { pub index: usize }

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
    DeclType(String,Type),    
    DeclMethod(String,Type,Vec<Parameter>,Stmt),
    // Statements
    // Expressions
    // Types
    TypeArray(Type),
    TypeBool,
    TypeInt(bool,u8),
    TypeNull,
    TypeRecord(Vec<(Type,String)>),
    TypeReference(Type),    
    TypeVoid
}

// =============================================================================
// Node Reference
// =============================================================================

/// A temporary reference into an AbstractSyntaxTree.  This is really
/// a wrapper for a node index into that tree.
#[derive(Copy,Clone)]
pub struct Ref<'a> {
    parent: &'a AbstractSyntaxTree,
    index: usize
}

/// Allow conversion from things to references, provided a suitable
/// parent pointer is available.
pub trait ToRef {
    fn to_ref<'a>(&self, ast: &'a AbstractSyntaxTree) -> Ref<'a>;
}

impl<'a> Ref<'a> {
    pub fn new(parent: &'a AbstractSyntaxTree, index: usize) -> Self {
	Ref{parent,index}
    }
}

impl fmt::Display for Ref<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let n = &self.parent.nodes[self.index];
	match n {
	    Node::DeclType(s,t) => {
		write!(f,"type {}={}",s,t.to_ref(self.parent))
	    }
	    Node::DeclMethod(n,r,ps,b) => {
		let pstr = to_string(ps);
		write!(f,"DeclMethod({},{},{},{})",n,r,pstr,b)		       
	    }
	    Node::TypeArray(t) => {
		write!(f,"{}[]",t.to_ref(self.parent))
	    }
	    Node::TypeBool => {
		write!(f,"bool")
	    }	    
	    Node::TypeInt(s,w) => {
		if *s {
		    write!(f,"i{}",w)
		} else {
		    write!(f,"u{}",w)
		}
	    }
	    Node::TypeNull => {
		write!(f,"null")
	    }
	    Node::TypeRecord(fields) => {
		let mut s = String::new();
		let mut b = true;
		s.push('{');    
		for (t,n) in fields {
		    if !b { s.push(','); }
		    b = false;
		    s.push_str(&t.to_ref(self.parent).to_string());
		    s.push_str(" ");
		    s.push_str(&n);
		}
		s.push('}');
		write!(f,"{}",s)
	    }
	    Node::TypeReference(t) => {
		write!(f,"&{}",t.to_ref(self.parent))
	    }
	    _ => {
		write!(f,"(????)")
	    }
	}
    }
}

impl From<Ref<'_>> for Stmt {
    fn from(r: Ref<'_>) -> Stmt {
	Stmt{index:r.index}
    }
}

impl From<Ref<'_>> for Type {
    fn from(r: Ref<'_>) -> Type {
	Type{index:r.index}
    }
}

impl ToRef for Type {
    fn to_ref<'a>(&self, parent: &'a AbstractSyntaxTree) -> Ref<'a> {
	Ref::new(parent,self.index)
    }
}

// =============================================================================
// Abstract Syntax Tree
// =============================================================================

pub struct AbstractSyntaxTree {
    nodes: Vec<Node>
}

impl AbstractSyntaxTree {
    pub fn new() -> AbstractSyntaxTree {
	AbstractSyntaxTree{nodes: Vec::new()}
    }
    /// Determine how many nodes are in this AST.
    pub fn len(&self) -> usize {
	self.nodes.len()
    }
    /// Access a given node
    pub fn get(&self, index: usize) -> &Node {
	&self.nodes[index]
    }
    /// Push a new node onto the tree
    pub fn push(&mut self, n: Node) -> usize {
	// Save current size of tree
	let idx = self.nodes.len();
	// Push new node in place	
	self.nodes.push(n);
	// Return its index
	idx
    }
}

impl fmt::Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"{}",to_string(&self.nodes))
    }
}

// =============================================================================
// Debug
// =============================================================================


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Node::DeclType(s,t) => {
		write!(f,"DeclType({},{})",s,t)
	    }
	    Node::DeclMethod(n,r,ps,b) => {
		let pstr = to_string(ps);
		write!(f,"DeclMethod({},{},{},{})",n,r,pstr,b)		       
	    }
	    Node::TypeArray(t) => {
		write!(f,"TypeArray({})",t)
	    }
	    Node::TypeBool => {
		write!(f,"TypeBool")
	    }	    
	    Node::TypeInt(s,w) => {
		write!(f,"TypeInt({},{})",s,w)
	    }
	    Node::TypeNull => {
		write!(f,"TypeNull")
	    }
	    Node::TypeReference(t) => {
		write!(f,"TypeReference({})",t)
	    }
	    _ => {
		write!(f,"(????)")
	    }
	}
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {	
	write!(f,"Param({},{})",self.declared,self.name)
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {	
	write!(f,"Stmt({})",self.index)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {	
	write!(f,"Type({})",self.index)
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
