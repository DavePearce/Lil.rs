use std::collections::HashMap;
use crate::ast::*;

/// Represents typing information attached to the Abstract Syntax
/// Tree.
#[derive(Clone,Debug,PartialEq)]
pub struct TypeMap {
    map : HashMap<usize,Type>	
}

impl TypeMap {
    pub fn new() -> Self {
	let map = HashMap::new();
	Self{map}
    }
    
    /// Map a given source string to a unique identifier which can be
    /// subsequently used to identify the corresponding AST syntactic
    /// element.
    pub fn map(&mut self, index: usize, element: Type) {
	// // Store details
	self.map.insert(index,element);
    }    
}
