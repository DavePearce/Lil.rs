use std::collections::HashMap;

/// Represents additional information which can be attached to the
/// tree.
#[derive(Clone,Debug,PartialEq)]
pub struct SourceMap<'a> {
    pub input : &'a str,
    pub map : HashMap<usize,&'a str>
}

impl<'a> SourceMap<'a> {
    pub fn new(input: &'a str) -> Self {
	let map = HashMap::new();
	Self{input,map}
    }
    
    /// Map a given source string to a unique identifier which can be
    /// subsequently used to identify the corresponding AST syntactic
    /// element.
    pub fn map(&mut self, index: usize, element: &'a str) {
	// // Store details
	self.map.insert(index,element);
    }

    pub fn get_highlight(&self, index: usize) -> Highlight {
	// Lookup given node in the map
	let val = self.map.get(&index);
	// See what we got
	match val { 
	    Some(s) => {
		Highlight{line: s, start:0, end: 1}
	    }
	    None => {
		EMPTY_HIGHLIGHT
	    }
	}
    }
}

/// Provides a useful package for reporting error messages.
pub struct Highlight<'a> {
    pub line : &'a str,
    pub start: usize,
    pub end: usize
}

/// A dummy highlight to use when (for whatever reason) the necessary
/// source information for a given node is missing.
pub const EMPTY_HIGHLIGHT : Highlight<'static> = Highlight{ line: "", start: 0, end: 0 };

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
