use logos::Logos;
use std::ops::Range;

pub struct Tokens<'a> {
    items: &'a [Token],
    spans: &'a [Range<usize>],
    next: usize
}

impl<'a> Tokens<'a> {
    pub fn new(items: &'a [Token], spans: &'a [Range<usize>]) -> Tokens<'a> {
        Tokens { items: items, spans: spans, next: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.get_token(self.next)
    }

    #[allow(dead_code)]
    pub fn advance(&mut self) {
        self.next += 1
    }

    #[allow(dead_code)]
    pub fn advance_by(&mut self, count: usize) {
        self.next += count
    }

    #[allow(dead_code)]
    pub fn remember(&self) -> usize {
        self.next
    }

    #[allow(dead_code)]
    pub fn backtrack(&mut self, next_idx: usize) {
        self.next = next_idx
    }

    pub fn span(&self) -> std::ops::Range<usize> {
        match self.span_explicit() {
        Some(span) => span,
        None       => 0..0
        }
    }

    pub fn span_explicit(&self) -> Option<std::ops::Range<usize>> {
        if self.next <= self.spans.len() {
            Some(self.spans[self.next - 1].clone())
        } else {
            None
        }
    }

    pub fn get_token(&self, index: usize) -> Option<&Token> {
        self.items.get(index)
    }

    #[allow(dead_code)]
    pub fn get_span(&self, index: usize) -> Option<&std::ops::Range<usize>> {
        self.spans.get(index)
    }

    pub fn has_next(&self) -> bool {
        self.next < self.items.len()
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.items.len() {
            self.next += 1;
            return Some(&self.items[self.next - 1])
        } else {
            return None
        }
    }
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token {
    #[regex("echo")]
    Echo,

    #[regex("function")]
    Function,

    #[regex("if")]
    If,

    #[regex("else")]
    Else,

    #[regex("while")]
    While,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,

    #[regex(r"(//|#).*\n", logos::skip)]
    Comment,

    #[regex(r"-?[0-9][0-9_]*(\.[0-9][0-9_]*)?")]
    Number,

    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,
    
    #[token("(")]
    Open,
    
    #[token(")")]
    Close,
    
    #[token("{")]
    Begin,
    
    #[token("}")]
    End,

    #[token(",")]
    Next,

    #[token("=")]
    Assign,

    #[token("..")]
    Spread,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    
    #[regex("\\$[a-zA-Z_][a-zA-Z0-9_]*")]
    Variable,

    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f;]+", logos::skip)]
    Whitespace,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
