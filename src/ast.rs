use std::fmt;
use std::vec;

pub enum Node {
    // Declarations
    TypeAlias(String,Type),
    Method(String,Type,Vec<Parameter>,Stmt),
    // Statements
    // Expressions
    // Types
    TypeNull
}

/// Represents a parameter declaration in the source of a given method.
#[derive(Clone,Debug,PartialEq)]
pub struct Parameter {
    pub declared : Type,
    pub name : String
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Node::TypeAlias(s,t) => {
		write!(f,"Type({},{})",s,t)
	    }
	    Node::Method(n,r,ps,b) => {
		let pstr = to_string(ps);
		write!(f,"Method({},{},{},{})",n,r,pstr,b)		       
	    }
	    _ => {
		write!(f,"(????)")
	    }
	}
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"Parameter({},{})",self.declared,self.name)
    }
}

// =============================================================================
// Statements
// =============================================================================
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Stmt {
    index: usize
}

impl Stmt {
    pub fn new(index: usize) -> Stmt {
	Stmt{index}
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"&Stmt({})",self.index)
    }
}

// =============================================================================
// Types
// =============================================================================

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Type {
    index: usize
}

impl Type {
    pub fn new(index: usize) -> Type {
	Type{index}
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"&Type({})",self.index)
    }
}

// =============================================================================
// Constructors
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

// =============================================================================
// Debug
// =============================================================================

// impl fmt::Display for Stmt {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 	match self {
// 	    Stmt::Block(ss) => {
// 		let body = to_string(ss);
// 		write!(f,"Block{}",body)
// 	    }
// 	    Stmt::Assert(e) => {
// 		write!(f,"Assert({})",e)
// 	    }
// 	    _ => {
// 		write!(f,"Stmt")
// 	    }
// 	}
//     }
// }

// impl fmt::Display for Expr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 	match self {
// 	    Expr::Variable(s) => {
// 		write!(f,"Var({})",s)
// 	    }
// 	    Expr::IntLiteral(i) => {
// 		write!(f,"Int({})",i)
// 	    }
// 	    Expr::BoolLiteral(b) => {
// 		write!(f,"Bool({})",b)
// 	    }
// 	    _ => {
// 		write!(f,"Expr")
// 	    }
// 	}
//     }
// }

// impl fmt::Display for Type {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 	match self {
// 	    Type::Array(elem) => {
// 		write!(f,"{}[]",elem)
// 	    }
// 	    Type::Bool => {
// 		write!(f,"bool")
// 	    }
// 	    Type::Null=> {
// 		write!(f,"null")
// 	    }
// 	    Type::Int8 => {
// 		write!(f,"int8")
// 	    }
// 	    Type::Int16 => {
// 		write!(f,"int16")
// 	    }
// 	    Type::Int32 => {
// 		write!(f,"int32")
// 	    }
// 	    Type::Int64 => {
// 		write!(f,"int64")
// 	    }
// 	    Type::Record(fs) => {
// 		write!(f,"{{ ??? }}")
// 	    }
// 	    Type::Ref(elem) => {
// 		write!(f,"&{}",elem)
// 	    }
// 	    Type::Uint8 => {
// 		write!(f,"uint8")
// 	    }
// 	    Type::Uint16 => {
// 		write!(f,"uint16")
// 	    }
// 	    Type::Uint32 => {
// 		write!(f,"uint32")
// 	    }
// 	    Type::Uint64 => {
// 		write!(f,"uint64")
// 	    }
// 	    Type::Void => {
// 		write!(f,"void")
// 	    }
// 	}
//     }
// }

// impl fmt::Display for Parameter {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 	write!(f,"({},{})",self.declared,self.name)
//     }
// }


fn to_string<T:fmt::Display>(items : &[T]) -> String {
    let mut s = String::new();
    s.push('[');
    for item in items {
	s.push_str(&item.to_string());
    }
    s.push(']');
    return s;
}
