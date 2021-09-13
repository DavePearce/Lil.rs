use std::str::SplitWhitespace;

pub enum Token {
    Keyword(String),
    Identifier(String),
    Integer(i32)    
}

pub trait Tokenizer<'a> {
    /// A Tokenizer always needs to produce an Iterator of Tokens.
    type TokenIter: Iterator<Item = Token>;

    /// Takes the input string and tokenizes it based on the implementations rules.
    fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}


pub struct TokenIter<'a> {
    ws_iter : SplitWhitespace<'a>
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
	return Some(Token::Keyword("asdasd".to_string()));
    }
}
