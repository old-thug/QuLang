use qu_diagnostics::Diagnostic;
use qu_source::SourceId;

use crate::{lexer::Lexer, token::Token};

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub cursor: usize,
}

impl TokenStream {
    pub fn new(source: &str, source_id: SourceId) -> Result<Self, Vec<Diagnostic>> {
        let mut lexer = Lexer::new(source, source_id);
        let mut diags = Vec::new();
        let mut tokens = Vec::new();

        loop {
            match lexer.next_token() {
                Ok(token) => {
                    let is_eof = token.is_eof();
                    tokens.push(token);
                    if is_eof { break; }
                }
                Err(diag) => {
                    diags.push(diag);
                },
            }
        }

        if !diags.is_empty() {
            Err(diags)
        } else {
            Ok(TokenStream { tokens, cursor: 0 })
        }
    }

    pub fn peek(&self) -> Token {
        *self.tokens.get(self.cursor)
            .or_else(|| self.tokens.last())
            .expect("TokenStream must contain at least an EOF token")
    }

    pub fn next(&mut self) -> Token {
        let token = self.peek();
        if !token.is_eof() {
            self.cursor += 1;
        }
        token
    }

    pub fn peek_nth(&self, nth: i32) -> Token {
        let target_index = self.cursor as i32 + nth;
        if target_index < 0 {
            return *self.tokens.first().expect("TokenStream is empty");
        }

        *self.tokens.get(target_index as usize)
            .or_else(|| self.tokens.last())
            .expect("TokenStream must contain at least an EOF token")
    }
}
