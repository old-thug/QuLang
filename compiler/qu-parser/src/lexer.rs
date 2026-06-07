use std::str::Chars;

use qu_ast::stmt::UsePath::Pair;
use qu_context::SourceId;
use qu_diagnostics::{Diagnostic, span::Span};

use crate::{tok, token::{Literal, Operator, Separator, Token, TokenKind}};

#[derive(Debug)]
pub struct Lexer<'source> {
    source: &'source str,
    index: usize,
    prev_index: usize,
    source_id: SourceId,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, source_id: SourceId) -> Self {
        Lexer {
            source,
            source_id,
            index: 0,
            prev_index: 0,
        }
    }

    /// Peek at the current character without consuming it.
    /// Returns '\0' if we have reached the end of the source.
    fn cur(&self) -> char {
        self.source[self.index..].chars().next().unwrap_or('\0')
    }

    /// Advance the lexer by one character and return it.
    fn next(&mut self) -> char {
        let ch = self.cur();
        if ch != '\0' {
            self.index += ch.len_utf8();
        }
        ch
    }

    /// Conditionally consume a prefix string if it matches the current position.
    fn eat(&mut self, prefix: &str) -> bool {
        if self.source[self.index..].starts_with(prefix) {
            self.index += prefix.len();
            return true;
        }
        false
    }

    /// Check if the lexer has exhausted the source string.
    fn is_done(&self) -> bool {
        self.index >= self.source.len() || self.cur() == '\0'
    }

    pub fn next_token(&mut self) -> Result<Token, Diagnostic> {
        // 1. Skip whitespace and comments safely
        loop {
            let c = self.cur();
            if c == '\0' {
                break;
            }

            if "\n\t\r ".contains(c) {
                self.next();
                continue;
            }

            if self.eat("//") {
                while !"\n\r\0".contains(self.cur()) {
                    self.next();
                }
                continue;
            }

            break;
        }

        // 2. Check for End of File
        if self.is_done() {
            return Ok(self.tok(TokenKind::EndOfFile));
        }

        self.prev_index = self.index;
        let first_char = self.cur();

        // 3. Match Identifiers and Keywords
        if first_char.is_alphabetic() || first_char == '_' {
            while self.cur().is_alphanumeric() || self.cur() == '_' {
                self.next();
            }
            
            let slice = &self.source[self.prev_index..self.index];
            let kind = match slice {
                "fn" => tok!(kw crate::token::Keyword::Fn),
                "pub" => tok!(kw crate::token::Keyword::Pub),
                "module" => tok!(kw crate::token::Keyword::Module),
                "use" => tok!(kw crate::token::Keyword::Use),
                "type" => tok!(kw crate::token::Keyword::Type),
                "return" => tok!(kw crate::token::Keyword::Return),
                "if" => tok!(kw crate::token::Keyword::If),
                "else" => tok!(kw crate::token::Keyword::Else),
                "for" => tok!(kw crate::token::Keyword::For),
                "break" => tok!(kw crate::token::Keyword::Break),
                "continue" => tok!(kw crate::token::Keyword::Continue),
                "const" => tok!(kw crate::token::Keyword::Const),
                "let" => tok!(kw crate::token::Keyword::Let),
                "mut" => tok!(kw crate::token::Keyword::Mut),
                "move" => tok!(kw crate::token::Keyword::Move),
                "cast" => tok!(kw crate::token::Keyword::Cast),
                "true" => tok!(lt Literal::True),
                "false" => tok!(lt Literal::False),
                _ => TokenKind::Identifier,
            };
            
            return Ok(self.tok(kind));
        }        

        // 4. Match Integer Literals
        if first_char.is_ascii_digit() {
            while self.cur().is_ascii_digit() {
                self.next();
            }
            return Ok(self.tok(tok!(lt Literal::Integer)));
        }

        // 5. Match Operators and Separators (ordered by length descending)
        let kind = if self.eat("->") { tok!(sp Separator::Arrow) }
        else if self.eat("|>") { tok!(op Operator::Pipe) }
        else if self.eat("+=") { tok!(op Operator::AddAssign) }
        else if self.eat("-=") { tok!(op Operator::SubAssign) }
        else if self.eat("*=") { tok!(op Operator::MulAssign) }
        else if self.eat("/=") { tok!(op Operator::DivAssign) }
        else if self.eat(".")  { tok!(sp Separator::Dot) }
        else if self.eat(";")  { tok!(sp Separator::SemiColon) }
        else if self.eat(",")  { tok!(sp Separator::Comma) }
        else if self.eat(":")  { tok!(sp Separator::Colon) }
        else if self.eat("(")  { tok!(sp Separator::OpenParen) }
        else if self.eat(")")  { tok!(sp Separator::CloseParen) }
        else if self.eat("{")  { tok!(sp Separator::OpenBrace) }
        else if self.eat("}")  { tok!(sp Separator::CloseBrace) }
        else if self.eat("[")  { tok!(sp Separator::OpenBracket) }
        else if self.eat("]")  { tok!(sp Separator::CloseBracket) }
        else if self.eat("+")  { tok!(op Operator::Add) }
        else if self.eat("-")  { tok!(op Operator::Minus) }
        else if self.eat("*")  { tok!(op Operator::Star) }
        else if self.eat("/")  { tok!(op Operator::Slash) }
        else if self.eat("=")  { tok!(op Operator::Assign) }
        else {
            // Fallback for unexpected characters. 
            // Replace this with your custom miette Diagnostic error later!
            let invalid_char = self.next();
            todo!("Handle unexpected character syntax error: '{}'", invalid_char);
        };

        Ok(self.tok(kind))
    }

    fn tok(&self, kind: TokenKind) -> Token {
        Token { 
            kind, 
            span: Span::new(self.prev_index, self.index, self.source_id) 
        }
    }
}
