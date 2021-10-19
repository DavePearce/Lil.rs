use std::iter::Peekable;
use std::str::CharIndices;

// =================================================================
// Token
// =================================================================

#[derive(Clone,Copy,PartialEq)]
pub enum TokenType {
    Ampersand,
    AmpersandAmpersand,
    Assert,
    Bar,
    BarBar,
    Bool,
    Break,
    Case,
    Colon,
    Comma,
    Continue,
    Default,
    Do,
    Dot,
    Delete,
    Else,
    EOF,
    Equal,
    EqualEqual,
    False,
    For,
    Identifier,
    If,
    I8,
    I16,
    I32,
    I64,
    Integer,
    LeftAngle,
    LeftAngleEquals,    
    LeftBrace,
    LeftCurly,
    LeftSquare,
    Minus,
    MinusGreater,
    New,
    Null,
    Percent,
    Plus,
    Return,
    RightAngle,
    RightAndleEquals,
    RightBrace,
    RightCurly,
    RightSlash,
    RightSlashSlash,
    RightSquare,
    Shreak,    
    ShreakEquals,
    SemiColon,
    Skip,
    Switch,
    Star,
    True,
    Type,
    While,
    U8,
    U16,
    U32,
    U64,
    Void
}

/// Represents a single token generated from a string slice.  This
/// identifies where the token starts and ends in the original slice.
#[derive(Clone,Copy,PartialEq)]
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

    /// Get the string payload associated with this token.
    pub fn as_string(&self) -> String {
	return self.content.to_string();
    }

    /// Get offset of the last character of this token.
    pub fn end(&self) -> usize {
	self.start + self.content.len()
    }
}

/// Represents the end of the input stream.  This is helpful because
/// it allows us to avoid using `Option<>` everywhere.
pub const EOF : Token<'static> = Token{kind: TokenType::EOF,start:usize::MAX,content: ""};

// =================================================================
// Lexer
// =================================================================

/// Provides machinery for splitting up a string slice into a sequence
/// of tokens.
pub struct Lexer<'a> {
    /// String slice being tokenized
    pub input: &'a str,
    /// Peekable interator into characters
    chars: Peekable<CharIndices<'a>>,
    /// Lookahead
    lookahead: Option<Token<'a>>
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
            input, chars, lookahead: None
        }
    }

    /// Peek at the next token in the sequence, or none if we have
    /// reached the end.
    pub fn peek(&mut self) -> Token<'a> {
	// Check whether lookahead already available
	if self.lookahead.is_none() {
	    // Lookahead not initialised, so physically read token.
	    self.lookahead = Some(self.next())
	}
	//
	self.lookahead.unwrap()
    }
    
    /// Get the next token in the sequence, or none if we have reached
    /// the end.
    pub fn next(&mut self) -> Token<'a> {
	// Check whether lookahead available
	match self.lookahead {
	    Some(t) => {
		// Reset lookahead
		self.lookahead = None;
		// Return it
		t
	    }
	    None => {
		// Try and extract next character
		let n = self.chars.next();
		// Sanity check it
		match n {
		    None => {
			EOF
		    }
		    Some((offset,ch)) => {
			self.scan(offset,ch)
		    }
		}
	    }
	}
    }

    /// Begin process of scanning a token based on its first
    /// character.  The actual work is offloaded to a helper based on
    /// this.
    fn scan(&mut self, start: usize, ch: char) -> Token<'a> {
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
    fn scan_whitespace(&mut self) -> Token<'a> {
        // Drop all following whitespace
        self.scan_whilst(|c| c.is_whitespace());
        // Scan an actual token
        self.next()
    }

    /// Scan all digits from a given starting point.
    fn scan_integer(&mut self, start: usize) -> Token<'a> {
        let kind = TokenType::Integer;
        let end = self.scan_whilst(|c| c.is_digit(10));
        let content = &self.input[start..end];
        Token{kind,start,content}
    }

    /// Scan an identifier or keyword.
    fn scan_identifier_or_keyword(&mut self, start: usize) -> Token<'a> {
        let end = self.scan_whilst(is_identifier_middle);
        let content = &self.input[start..end];
        let kind = match content {
	    "assert" => {
                TokenType::Assert
            }
	    "bool" => {
                TokenType::Bool
            }
	    "break" => {
                TokenType::Break
            }
	    "case" => {
                TokenType::Case
            }
	    "continue" => {
                TokenType::Continue
            }
	    "default" => {
                TokenType::Default
            }
	    "Do" => {
                TokenType::Do
            }
	    "delete" => {
                TokenType::Delete
            }
	    "else" => {
                TokenType::Else
            }
	    "false" => {
                TokenType::False
            }
	    "for" => {
                TokenType::For
            }
            "if" => {
                TokenType::If
            }
	    "i8" => {
                TokenType::I8
            }
	    "i16" => {
                TokenType::I16
            }
	    "i32" => {
                TokenType::I32
            }
	    "i64" => {
                TokenType::I64
            }   
	    "new" => {
                TokenType::New
            }
	    "null" => {
                TokenType::Null
            }
	    "return" => {
                TokenType::Return
            }
	    "skip" => {
                TokenType::Skip
            }
	    "switch" => {
                TokenType::Switch
            }
	    "true" => {
                TokenType::True
            }
	    "type" => {
                TokenType::Type
            }
            "while" => {
                TokenType::While
            }
	    "u8" => {
                TokenType::U8
            }
	    "u16" => {
                TokenType::U16
            }
	    "u32" => {
                TokenType::U32
            }
	    "u64" => {
                TokenType::U64
            }	    	    
	    "void" => {
                TokenType::Void
            }
            _ => {
                TokenType::Identifier
            }
        };
        Token{kind,start,content}
    }

    /// Scan an operator from a given starting point.
    fn scan_operator(&self, start: usize, ch: char) -> Token<'a> {
        let end : usize;
        let kind = match ch {
	    '&' => {
		// FIXME: &&
		end = start + 1;
                TokenType::Ampersand
	    }
	    '|' => {
		// FIXME: ||
		end = start + 1;
                TokenType::Bar
	    }
	    ':' => {
		end = start + 1;
                TokenType::Colon
	    }
	    ',' => {
		end = start + 1;
                TokenType::Comma
	    }
	    '.' => {
		end = start + 1;
                TokenType::Dot
	    }
	    '=' => {
		// FIXME: ==		
		end = start + 1;
                TokenType::Equal
	    }
	    '<' => {
		// FIXME: <=
                end = start + 1;
                TokenType::LeftAngle
            }
            '(' => {
                end = start + 1;
                TokenType::LeftBrace
            }
	    '{' => {
                end = start + 1;
                TokenType::LeftCurly
            }
	    '[' => {
                end = start + 1;
                TokenType::LeftSquare
            }
	    '-' => {
		// FIXME: ->
                end = start + 1;
                TokenType::Minus
            }
	    '%' => {
                end = start + 1;
                TokenType::Percent
            }
	    '+' => {
                end = start + 1;
                TokenType::Plus
            }
            '>' => {
		// FIXME: >=
                end = start + 1;
                TokenType::RightAngle
            }
	    ')' => {
                end = start + 1;
                TokenType::RightBrace
            }
	    '}' => {
                end = start + 1;
                TokenType::RightCurly
            }
	    '/' => {
		// FIXME: //
                end = start + 1;
                TokenType::RightSlash
            }
	    ']' => {
                end = start + 1;
                TokenType::RightSquare
            }
	    ';' => {
		end = start + 1;
                TokenType::SemiColon
	    }
	    '!' => {
		// FIXME: != 
                end = start + 1;
                TokenType::Shreak
            }
	    '*' => {
                end = start + 1;
                TokenType::Star
            }
            _ => {
                return EOF;
            }
        };
        let content = &self.input[start..end];
        Token{kind,start,content}
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
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_02() {
    let mut l = Lexer::new(" ");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_03() {
    let mut l = Lexer::new("  ");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_04() {
    let mut l = Lexer::new("\n");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_05() {
    let mut l = Lexer::new(" \n");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_06() {
    let mut l = Lexer::new("\n ");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_07() {
    let mut l = Lexer::new("\t");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_08() {
    let mut l = Lexer::new("\t ");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

#[test]
fn test_09() {
    let mut l = Lexer::new(" \t");
    assert!(l.peek() == EOF);
    assert!(l.next() == EOF);
}

// Literals

#[test]
fn test_10() {
    let mut l = Lexer::new("1");
    assert!(l.peek().kind == TokenType::Integer);    
    assert!(l.next().kind == TokenType::Integer);
    assert!(l.next() == EOF);
}

#[test]
fn test_11() {
    let mut l = Lexer::new("  1");
    assert!(l.peek().as_int() == 1);    
    assert!(l.next().as_int() == 1);
    assert!(l.next() == EOF);
}

#[test]
fn test_12() {
    let mut l = Lexer::new("1234");
    assert!(l.peek().as_int() == 1234);
    assert!(l.next().as_int() == 1234);
    assert!(l.next() == EOF);
}

#[test]
fn test_13() {
    let mut l = Lexer::new("1234 ");
    assert!(l.peek().as_int() == 1234);
    assert!(l.next().as_int() == 1234);
    assert!(l.next() == EOF);
}

#[test]
fn test_14() {
    let mut l = Lexer::new("1234_");
    assert!(l.peek().kind == TokenType::Integer);    
    assert!(l.next().kind == TokenType::Integer);
    assert!(l.peek().kind == TokenType::Identifier);
    assert!(l.next().kind == TokenType::Identifier);
    assert!(l.peek() == EOF);
}

#[test]
fn test_15() {
    let mut l = Lexer::new("1234X");
    assert!(l.peek().as_int() == 1234);
    assert!(l.next().as_int() == 1234);
    assert!(l.peek().kind == TokenType::Identifier);
    assert!(l.next().kind == TokenType::Identifier);
    assert!(l.peek() == EOF);
}

#[test]
fn test_16() {
    let mut l = Lexer::new("1234 12");
    assert!(l.peek().as_int() == 1234);
    assert!(l.next().as_int() == 1234);
    assert!(l.peek().as_int() == 12);
    assert!(l.next().as_int() == 12);
}

// Identifiers

#[test]
fn test_20() {
    let mut l = Lexer::new("abc");
    let t = l.next();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "abc");
    assert!(l.next() == EOF);
}

#[test]
fn test_21() {
    let mut l = Lexer::new("  abc");
    assert!(l.peek().kind == TokenType::Identifier);
    let t = l.next();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "abc");
    assert!(l.next() == EOF);
}

#[test]
fn test_22() {
    let mut l = Lexer::new("_abc");
    assert!(l.peek().kind == TokenType::Identifier);
    let t = l.next();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "_abc");
    assert!(l.next() == EOF);
}

#[test]
fn test_23() {
    let mut l = Lexer::new("a_bD12233_");
    assert!(l.peek().kind == TokenType::Identifier);
    let t = l.next();
    assert!(t.kind == TokenType::Identifier);
    assert!(t.content == "a_bD12233_");
    assert!(l.next() == EOF);
}

#[test]
fn test_24() {
    let mut l = Lexer::new("_abc cd");
    assert!(l.peek().kind == TokenType::Identifier);
    let t1 = l.next();
    assert!(t1.kind == TokenType::Identifier);
    assert!(t1.content == "_abc");
    assert!(l.peek().kind == TokenType::Identifier);
    let t2 = l.next();    
    assert!(t2.kind == TokenType::Identifier);
    assert!(t2.content == "cd");
    assert!(l.next() == EOF);
}

// Keywords

#[test]
fn test_30() {
    let mut l = Lexer::new("if");
    assert!(l.peek().kind == TokenType::If);
    assert!(l.next().kind == TokenType::If);
    assert!(l.next() == EOF);
}

#[test]
fn test_31() {
    let mut l = Lexer::new("while");
    assert!(l.peek().kind == TokenType::While);
    assert!(l.next().kind == TokenType::While);
    assert!(l.next() == EOF);
}

// Operators

#[test]
fn test_40() {
    let mut l = Lexer::new("(");
    assert!(l.peek().kind == TokenType::LeftBrace);
    assert!(l.next().kind == TokenType::LeftBrace);
    assert!(l.next() == EOF);
}

#[test]
fn test_41() {
    let mut l = Lexer::new("((");
    assert!(l.peek().kind == TokenType::LeftBrace);
    assert!(l.next().kind == TokenType::LeftBrace);
    assert!(l.peek().kind == TokenType::LeftBrace);
    assert!(l.next().kind == TokenType::LeftBrace);
    assert!(l.next() == EOF);
}

#[test]
fn test_42() {
    let mut l = Lexer::new(")");
    assert!(l.peek().kind == TokenType::RightBrace);
    assert!(l.next().kind == TokenType::RightBrace);
}

#[test]
fn test_43() {
    let mut l = Lexer::new("))");
    assert!(l.peek().kind == TokenType::RightBrace);
    assert!(l.next().kind == TokenType::RightBrace);
    assert!(l.peek().kind == TokenType::RightBrace);
    assert!(l.next().kind == TokenType::RightBrace);
    assert!(l.next() == EOF);
}

#[test]
fn test_44() {
    let mut l = Lexer::new("()");
    assert!(l.peek().kind == TokenType::LeftBrace);
    assert!(l.next().kind == TokenType::LeftBrace);
    assert!(l.peek().kind == TokenType::RightBrace);
    assert!(l.next().kind == TokenType::RightBrace);
    assert!(l.next() == EOF);
}

// Combinations

#[test]
fn test_60() {
    let mut l = Lexer::new("while(");
    let t1 = l.next();
    assert!(t1.kind == TokenType::While);
    assert!(t1.content == "while");
    let t2 = l.next();
    assert!(t2.kind == TokenType::LeftBrace);
    assert!(t2.content == "(");
    assert!(l.next() == EOF);
}

#[test]
fn test_61() {
    let mut l = Lexer::new("12345(");
    let t1 = l.next();
    assert!(t1.kind == TokenType::Integer);
    assert!(t1.as_int() == 12345);
    let t2 = l.next();
    assert!(t2.kind == TokenType::LeftBrace);
    assert!(t2.content == "(");
    assert!(l.next() == EOF);
}
