use std::result;
use std::collections::HashMap;
use crate::ast::*;

/// A type map maps AST nodes to their type.
pub struct TypeMap {
    map : Vec<Type>
}

pub type Result<T> = result::Result<T, ()>;

/// A simple structure to abstract the mapping of variables to types.
pub struct Environment<'a> {
    typing : HashMap<&'a str, Type>
}

pub fn type_check<'a>(d : Decl<usize>) -> Result<TypeMap> {
    return Err(());
}

