use std::iter::Peekable;
use std::str::CharIndices;

// =================================================================
// Token
// =================================================================

#[derive(PartialEq)]
pub enum TokenType {
    Identifier,
    If,
    Integer,
    LeftBrace,
    RightBrace,
    While
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

impl<'a> Token<'a> {
    /// Get the integer payload associated with this token, assuming
    /// it has Integer kind.
    pub fn as_int(&self) -> i32 {
	// Can only call this method on integer tokens.
	assert!(self.kind == TokenType::Integer);
	// Parse conents (expecting integer)
	return self.content.parse().unwrap();
    }
}

// =================================================================
// Lexer
// =================================================================

/// Provides machinery for splitting up a string slice into a sequence
/// of tokens.
pub struct Lexer<'a> {
    /// String slice being tokenized
    input: &'a str,
    /// Peekable interator into characters
    chars: Peekable<CharIndices<'a>>
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
            input, chars
        }
    }

    /// Get the next token in the sequence, or none if we have reached
    /// the end.
    pub fn next(&mut self) -> Option<Token> {
        // Skip any preceeding whitespace
        //self.skipt_whitespace();
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
    fn scan(&mut self, start: usize, ch: char) -> Option<Token> {
        // Switch on first character of token
        if ch.is_whitespace() {
            self.scan_whitespace()
        } else if ch.is_digit(10) {
            self.scan_integer(start)
        } else if is_identifier_start(ch)  {
            self.scan_identifier_or_keyword(start)
        } else {
            self.scan_operator(start,ch)
        }
    }

    /// Scan all whitespace from a given starting point, then
    /// recursively scan an actual token.
    fn scan_whitespace(&mut self) -> Option<Token> {
        // Drop all following whitespace
        self.scan_whilst(|c| c.is_whitespace());
        // Scan an actual token
        self.next()
    }

    /// Scan all digits from a given starting point.
    fn scan_integer(&mut self, start: usize) -> Option<Token> {
        let kind = TokenType::Integer;
        let end = self.scan_whilst(|c| c.is_digit(10));
        let content = &self.input[start..end];
        Some(Token{kind,start,content})
    }

    /// Scan an identifier or keyword.
    fn scan_identifier_or_keyword(&mut self, start: usize) -> Option<Token> {
        let end = self.scan_whilst(is_identifier_middle);
        let content = &self.input[start..end];
        let kind = match content {
            "if" => {
                TokenType::If
            }
            "while" => {
                TokenType::While
            }
            _ => {
                TokenType::Identifier
            }
        };
        Some(Token{kind,start,content})
    }

    /// Scan an operator from a given starting point.
    fn scan_operator(&self, start: usize, ch: char) -> Option<Token> {
        let end : usize;
        let kind = match ch {
            '(' => {
                end = start + 1;
                TokenType::LeftBrace
            }
            ')' => {
                end = start + 1;
                TokenType::RightBrace
            }
            _ => {
                return None;
            }
        };
        let content = &self.input[start..end];
        Some(Token{kind,start,content})
    }

    /// Gobble all characters matched by an acceptor.  For example, we
    /// might want to continue matching digits until we encounter
    /// something which isn't a digit (or is the end of the file).
    fn scan_whilst(&mut self, pred : Acceptor) -> usize {
        // Continue reading whilst we're still matching characters
        while let Some((o,c)) = self.chars.peek() {
            if !pred(*c) {
                // If we get here, then bumped into something which is
                // not part of this token.
                return *o;
            }
            // Move to next character
            self.chars.next();
        }
        // If we get here, then ran out of characters.  So everything
        // from the starting point onwards is part of the token.
        self.input.len()
    }
}

/// Determine whether a given character is the start of an identifier.
fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Determine whether a given character can occur in the middle of an identifier
fn is_identifier_middle(c: char) -> bool {
    c.is_digit(10) || is_identifier_start(c)
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
fn test_02() {
    let mut l = Lexer::new(" ");
    assert!(l.next().is_none());
}

#[test]
fn test_03() {
    let mut l = Lexer::new("  ");
    assert!(l.next().is_none());
}

#[test]
fn test_04() {
    let mut l = Lexer::new("\n");
    assert!(l.next().is_none());
}

#[test]
fn test_05() {
    let mut l = Lexer::new(" \n");
    assert!(l.next().is_none());
}

#[test]
fn test_06() {
    let mut l = Lexer::new("\n ");
    assert!(l.next().is_none());
}

#[test]
fn test_07() {
    let mut l = Lexer::new("\t");
    assert!(l.next().is_none());
}

#[test]
fn test_08() {
    let mut l = Lexer::new("\t ");
    assert!(l.next().is_none());
}

#[test]
fn test_09() {
    let mut l = Lexer::new(" \t");
    assert!(l.next().is_none());
}

// Literals

#[test]
fn test_10() {
    let mut l = Lexer::new("1");
    assert!(l.next().unwrap().kind == TokenType::Integer);
    assert!(l.next().is_none());
}

#[test]
fn test_11() {
    let mut l = Lexer::new("  1");
    assert!(l.next().unwrap().as_int() == 1);
    assert!(l.next().is_none());
}

#[test]
fn test_12() {
    let mut l = Lexer::new("1234");
    assert!(l.next().unwrap().as_int() == 1234);
    assert!(l.next().is_none());
}

#[test]
fn test_13() {
    let mut l = Lexer::new("1234 ");
    assert!(l.next().unwrap().as_int() == 1234);
    assert!(l.next().is_none());
}

#[test]
fn test_14() {
    let mut l = Lexer::new("1234_");
    assert!(l.next().unwrap().kind == TokenType::Integer);
    assert!(l.next().unwrap().kind == TokenType::Identifier);
}

#[test]
fn test_15() {
    let mut l = Lexer::new("1234X");
    assert!(l.next().unwrap().as_int() == 1234);
    assert!(l.next().unwrap().kind == TokenType::Identifier);
}

#[test]
fn test_16() {
    let mut l = Lexer::new("1234 12");
    assert!(l.next().unwrap().as_int() == 1234);
    assert!(l.next().unwrap().as_int() == 12);
}

// Identifiers

#[test]
fn test_20() {
    let mut l = Lexer::new("abc");
    let t = l.next().unwrap();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "abc");
    assert!(l.next().is_none());
}

#[test]
fn test_21() {
    let mut l = Lexer::new("  abc");
    let t = l.next().unwrap();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "abc");
    assert!(l.next().is_none());
}

#[test]
fn test_22() {
    let mut l = Lexer::new("_abc");
    let t = l.next().unwrap();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "_abc");
    assert!(l.next().is_none());
}

#[test]
fn test_23() {
    let mut l = Lexer::new("a_bD12233_");
    let t = l.next().unwrap();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "a_bD12233_");
    assert!(l.next().is_none());
}

#[test]
fn test_24() {
    let mut l = Lexer::new("_abc cd");
    let t1 = l.next().unwrap();
    assert!(t1.kind == TokenType::Identifier);
    assert!(t1.content == "_abc");
    let t2 = l.next().unwrap();
    assert!(t2.kind == TokenType::Identifier);
    assert!(t2.content == "cd");
    assert!(l.next().is_none());
}

// Keywords

#[test]
fn test_30() {
    let mut l = Lexer::new("if");
    assert!(l.next().unwrap().kind == TokenType::If);
    assert!(l.next().is_none());
}

#[test]
fn test_31() {
    let mut l = Lexer::new("while");
    assert!(l.next().unwrap().kind == TokenType::While);
    assert!(l.next().is_none());
}

// Operators

#[test]
fn test_40() {
    let mut l = Lexer::new("(");
    assert!(l.next().unwrap().kind == TokenType::LeftBrace);
    assert!(l.next().is_none());
}

#[test]
fn test_41() {
    let mut l = Lexer::new("((");
    let t1 = l.next().unwrap();
    assert!(t1.kind == TokenType::LeftBrace);
    let t2 = l.next().unwrap();
    assert!(t2.kind == TokenType::LeftBrace);
    assert!(l.next().is_none());
}

#[test]
fn test_42() {
    let mut l = Lexer::new(")");
    assert!(l.next().unwrap().kind == TokenType::RightBrace);
}

#[test]
fn test_43() {
    let mut l = Lexer::new("))");
    let t1 = l.next().unwrap();
    assert!(t1.kind == TokenType::RightBrace);
    let t2 = l.next().unwrap();
    assert!(t2.kind == TokenType::RightBrace);
    assert!(l.next().is_none());
}

#[test]
fn test_44() {
    let mut l = Lexer::new("()");
    let t1 = l.next().unwrap();
    assert!(t1.kind == TokenType::LeftBrace);
    let t2 = l.next().unwrap();
    assert!(t2.kind == TokenType::RightBrace);
    assert!(l.next().is_none());
}

// Combinations

#[test]
fn test_60() {
    let mut l = Lexer::new("while(");
    let t1 = l.next().unwrap();
    assert!(t1.kind == TokenType::While);
    assert!(t1.content == "while");
    let t2 = l.next().unwrap();
    assert!(t2.kind == TokenType::LeftBrace);
    assert!(t2.content == "(");
    assert!(l.next().is_none());
}

#[test]
fn test_61() {
    let mut l = Lexer::new("12345(");
    let t1 = l.next().unwrap();
    assert!(t1.kind == TokenType::Integer);
    assert!(t1.as_int() == 12345);
    let t2 = l.next().unwrap();
    assert!(t2.kind == TokenType::LeftBrace);
    assert!(t2.content == "(");
    assert!(l.next().is_none());
}
