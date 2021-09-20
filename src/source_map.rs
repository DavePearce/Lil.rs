use std::collections::HashMap;

/// Represents additional information which can be attached to the
/// tree.
#[derive(Clone,Debug,PartialEq)]
pub struct SourceMap<'a> {
    input : &'a str,
    map : Vec<&'a str>
}

impl<'a> SourceMap<'a> {
    pub fn new(input: &'a str) -> Self {
	let map = Vec::new();
	Self{input,map}
    }
    
    /// Map a given source string to a unique identifier which can be
    /// subsequently used to identify the corresponding AST syntactic
    /// element.
    pub fn map(&mut self, element: &'a str) -> usize {
	// Allocate index
	let index = self.map.len();
	// Store details
	self.map.push(element);
	// Done
	index
    }
}

/**
 * Calculate the offset of one slice from another.  Specifically,
 * we're expecting that `inner` is a subslice of `outer`.
 */
fn subslice_offset<'a>(outer: &'a str, inner: &'a str) -> usize {
    let o_ptr = outer.as_ptr() as usize;    
    let i_ptr = inner.as_ptr() as usize;
    // Sanity check
    assert!(i_ptr >= o_ptr && i_ptr <= o_ptr.wrapping_add(outer.len()));
    // Calulcate offset of inner string from outer
    let offset = i_ptr.wrapping_sub(o_ptr);
    // Done
    offset
}
