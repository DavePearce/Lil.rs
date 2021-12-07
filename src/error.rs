use std::fmt;

/// Identifiers a particular kind of syntax error.
#[derive(Clone,Debug)]
pub enum ErrorCode {
    InternalFailure(String),
    /// Expected lhs type, got rhs type
    ExpectedSubtype,
    /// Access unknown variable
    VariableNotFound
}

/// Identifies some form of syntax error on a given Abstract Syntax
/// Tree node.  Every error is codified using a specific error code.
#[derive(Clone,Debug)]
pub struct SyntaxError {
    /// Identifies an AST node in the source file.
    pub node : usize,
    /// Identifies the kind of error
    pub errno : ErrorCode
}

/// Construct a syntax error which represents an internal failure of
/// some kind.
#[allow(dead_code)]
pub fn internal_failure(node: usize, msg: &str) -> SyntaxError {
    SyntaxError{node, errno: ErrorCode::InternalFailure(msg.to_string())}
}

/// Construct a syntax error representing a subtype error of some kind
/// (e.g. expected int, found bool).
#[allow(dead_code)]
pub fn expected_subtype(node: usize) -> SyntaxError {
    SyntaxError{node, errno: ErrorCode::ExpectedSubtype}
}

/// Construct a syntax error representing a variable not found error.
#[allow(dead_code)]
pub fn variable_not_found(node: usize) -> SyntaxError {
   SyntaxError{node, errno: ErrorCode::VariableNotFound}
}

/// Simple mechanism for printing an error code
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"expected a type, found another type")
    }
}
