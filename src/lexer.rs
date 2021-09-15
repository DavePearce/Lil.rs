use std::iter::Peekable;
use std::str::CharIndices;

#[derive(PartialEq)]
pub enum TokenType {
    Identifier,
    Integer,
    LeftBrace,
    RightBrace
}

/// Represents a single token generated from a string slice.  This
/// identifies where the token starts and ends in the original slice.
pub struct Token<'a> {
    /// Type of the token
    pub kind : TokenType,
    /// Identifies the starting point within the original string of
    /// this token.
    pub start : usize,
    /// Identifies the token within the original string slice.  From
    /// this we can extract useful information.  For example, if its
    /// an identifier we can extract the actual identifier string.
    pub content : &'a str
}


/// Provides machinery for splitting up a string slice into a sequence
/// of tokens.
pub struct Lexer<'a> {
    /// String slice being tokenized
    input: &'a str,
    /// Peekable interator into characters
    chars: Peekable<CharIndices<'a>>,
    /// Offset from sequence start
    offset: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        // Extract peekable iterator
        let chars = input.char_indices().peekable();
        // Construct lexer
        return Self {
            input, chars, offset: 0
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        None
    }
}

// ======================================================
// Tests
// ======================================================

#[test]
fn test_01() {
    let mut l = Lexer::new("");
    assert!(l.next().is_none());
}

// Literals

#[test]
fn test_02() {
    let mut l = Lexer::new("1");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

#[test]
fn test_03() {
    let mut l = Lexer::new("1234");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

// Identifiers

#[test]
fn test_04() {
    let mut l = Lexer::new("abc");
    assert!(l.next().unwrap().kind == TokenType::Identifier);
}

#[test]
fn test_05() {
    let mut l = Lexer::new("_abc");
    assert!(l.next().unwrap().kind == TokenType::Identifier);
}

#[test]
fn test_06() {
    let mut l = Lexer::new("a_bD12233_");
    assert!(l.next().unwrap().kind == TokenType::Identifier);
}

// Operators

#[test]
fn test_07() {
    let mut l = Lexer::new("(");
    assert!(l.next().unwrap().kind == TokenType::LeftBrace);
}

#[test]
fn test_08() {
    let mut l = Lexer::new(")");
    assert!(l.next().unwrap().kind == TokenType::RightBrace);
}
