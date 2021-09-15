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

/// An acceptor determines whether or not a character is part of a
/// given token.
type Acceptor = fn(char)->bool;

impl<'a> Lexer<'a> {
    /// Construct a new lexer for a given string slice.
    pub fn new(input: &'a str) -> Self {
        // Extract peekable iterator
        let chars = input.char_indices().peekable();
        // Construct lexer
        return Self {
            input, chars, offset: 0
        }
    }

    /// Get the next token in the sequence, or none if we have reached
    /// the end.
    pub fn next(&mut self) -> Option<Token> {
        // Try and extract next character
        let n = self.chars.next();
        // Sanity check it
        match n {
            None => {
                None
            }
            Some((offset,ch)) => {
                self.scan(offset,ch)
            }
        }
    }

    /// Begin process of scanning a token based on its first
    /// character.  The actual work is offloaded to a helper based on
    /// this.
    fn scan(&mut self, offset: usize, ch: char) -> Option<Token> {
        if ch.is_digit(10) {
            // Group all following digits together as an integer
            self.scan_token(offset,TokenType::Integer,is_digit)
        } else {
            None
        }
    }

    /// Gobble all characters matched by an acceptor into a token of a
    /// given kind.  For example, we might want to continue matching
    /// digits until we encounter something which isn't a digit (or is
    /// the end of the file).
    fn scan_token(&mut self, offset: usize, kind: TokenType, pred : Acceptor) -> Option<Token> {
        // Continue reading whilst we're still matching characters
        while let Some((o,c)) = self.chars.peek() {
            if !pred(*c) {
                // If we get here, then bumped into something which is
                // not part of this token.
                let content = &self.input[offset .. *o];
                // Done
                return Some(Token{kind: TokenType::Integer,start: offset,content})
            }
            // Move to next character
            self.chars.next();
        }
        // If we get here, then ran out of characters.  So everything
        // from the starting point onwards is part of the token.
        let content = &self.input[offset .. ];
        //
        Some(Token{kind,start: offset,content})
    }
}

/// Determine whether a given character is a digit or not.
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}


// ======================================================
// Tests
// ======================================================

#[test]
fn test_01() {
    let mut l = Lexer::new("");
    assert!(l.next().is_none());
}

#[test]
fn test_01b() {
    let mut l = Lexer::new(" ");
    assert!(l.next().is_none());
}

#[test]
fn test_01c() {
    let mut l = Lexer::new("  ");
    assert!(l.next().is_none());
}

#[test]
fn test_01d() {
    let mut l = Lexer::new("\n");
    assert!(l.next().is_none());
}

#[test]
fn test_01e() {
    let mut l = Lexer::new(" \n");
    assert!(l.next().is_none());
}

#[test]
fn test_01f() {
    let mut l = Lexer::new("\n ");
    assert!(l.next().is_none());
}

// Literals

#[test]
fn test_02() {
    let mut l = Lexer::new("1");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

#[test]
fn test_02b() {
    let mut l = Lexer::new("  1");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

#[test]
fn test_03() {
    let mut l = Lexer::new("1234");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

#[test]
fn test_03b() {
    let mut l = Lexer::new("1234 ");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

#[test]
fn test_0c() {
    let mut l = Lexer::new("1234_");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

#[test]
fn test_0d() {
    let mut l = Lexer::new("1234X");
    assert!(l.next().unwrap().kind == TokenType::Integer);
}

// Identifiers

#[test]
fn test_04() {
    let mut l = Lexer::new("abc");
    assert!(l.next().unwrap().kind == TokenType::Identifier);
}

#[test]
fn test_04b() {
    let mut l = Lexer::new("  abc");
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
