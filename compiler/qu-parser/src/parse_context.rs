use qu_ast::{Visibility, type_hint::Mutability};
use qu_context::SourceId;
use qu_diagnostics::{
    Diagnostic, Severity,
    span::{Span, Spanned},
};

use crate::{
    PResult, Parser, tok,
    token::{Keyword, Literal, Operator, Separator, Token, TokenKind},
};

pub struct ParseContext<'a, 'b> {
    parser: &'a mut Parser<'b>,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a, 'b> ParseContext<'a, 'b> {
    pub fn new(parser: &'a mut Parser<'b>) -> Self {
        Self {
            parser,
            diagnostics: Vec::new(),
        }
    }

    pub(super) fn emit(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub(super) fn slice(&self, span: Span) -> String {
        self.parser.source[span.start..span.end].to_string()
    }

    pub(super) fn current(&self) -> Token {
        self.parser.current_token
    }

    pub(super) fn current_kind(&self) -> TokenKind {
        self.current().kind
    }

    pub(super) fn previous_kind(&self) -> TokenKind {
        self.previous().kind
    }

    pub(super) fn previous(&self) -> Token {
        self.parser.previous_token
    }

    pub(super) fn equals(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    pub(super) fn is_kw(&self, kw: Keyword) -> bool {
        self.equals(TokenKind::Keyword(kw))
    }

    pub(super) fn is_op(&self, op: Operator) -> bool {
        self.equals(TokenKind::Operator(op))
    }

    pub(super) fn is_sp(&self, sp: Separator) -> bool {
        self.equals(TokenKind::Separator(sp))
    }

    pub(super) fn is_lt(&self, lt: Literal) -> bool {
        self.equals(TokenKind::Literal(lt))
    }

    pub(super) fn is_done(&self) -> bool {
        self.equals(TokenKind::EndOfFile)
    }

    pub(super) fn eat(
        &mut self,
        kinds: impl IntoIterator<Item = TokenKind> + Clone,
    ) -> PResult<Token> {
        let mut len = 0;
        for kind in kinds.clone() {
            if self.current().kind == kind {
                self.next();
                return Some(self.previous());
            }
            len += 1;
        }
        let mut buffer = String::new();
        for (index, kind) in kinds.into_iter().enumerate() {
            if index != 0 {
                buffer += ", ";
            } else if index != 0 && index < len {
                buffer += " or ";
            }

            buffer += &format!("{:?}", kind)
        }

        self.emit(Diagnostic::new(
            Severity::Error,
            "unexpected token".to_string(),
            self.current().span,
            buffer,
        ));
        None
    }

    pub(super) fn try_eat_many(
        &mut self,
        kinds: impl IntoIterator<Item = TokenKind> + Clone,
    ) -> PResult<Option<Token>> {
        for kind in kinds {
            if self.current().kind == kind {
                self.next()?;
                return Some(Some(self.previous()));
            }
        }
        Some(None)
    }

    pub(super) fn eat_or_else<F, M, U>(
        &mut self,
        kind: TokenKind,
        on_match: M,
        fallback: F,
    ) -> PResult<U>
    where
        M: FnOnce(Token) -> U, // Converts the eaten token to U
        F: FnOnce(&mut ParseContext) -> PResult<U>,
    {
        // Assuming self.eat() returns PResult<Token>
        match self.eat([kind]) {
            Some(token) => Some(on_match(token)),
            None => fallback(self),
        }
    }

    pub(super) fn try_eat(&mut self, kind: TokenKind) -> PResult<bool> {
        if self.current().kind == kind {
            self.next()?;
            return Some(true);
        }
        Some(false)
    }

    pub(super) fn skip_to(
        &mut self,
        eat: bool,
        kinds: impl IntoIterator<Item = TokenKind> + Clone,
    ) -> PResult<bool> {
        while !self.is_done() {
            for kind in kinds.clone() {
                if self.equals(kind) {
                    if eat {
                        self.next()?;
                    }
                    return Some(true);
                }
            }
            self.next()?;
        }
        Some(false)
    }

    pub(super) fn skip_until<F>(&mut self, eat: bool, f: F) -> PResult<()>
    where
        F: Fn(&Token) -> bool,
    {
        while !self.is_done() && !f(&self.current()) {
            self.next()?;
        }

        if !self.is_done() && eat {
            self.next()?;
        }

        Some(())
    }

    pub(super) fn next(&mut self) -> PResult<Token> {
        match self.parser.next() {
            Ok(tok) => Some(tok),
            Err(diag) => {
                self.emit(diag);
                self.next()
            }
        }
    }

    pub(super) fn eat_tok_and<F, U>(&mut self, kind: TokenKind, mut f: F) -> PResult<U>
    where
        F: FnMut(&mut ParseContext, Token) -> U,
    {
        let tok = self.eat([kind])?;
        Some(f(self, tok))
    }

    // TODO: This function feels wrong. Fix it.
    pub(crate) fn collect_specifiers(
        &mut self,
    ) -> PResult<(Spanned<Visibility>, Spanned<Mutability>)> {
        // 1. Parse Visibility
        let visibility = if let Some(token) = self.try_eat_many([tok!(kw Keyword::Pub)])? {
            Spanned::new(Visibility::Public, token.span)
        } else if let Some(token) = self.try_eat_many([tok!(kw Keyword::Shared)])? {
            // Note: Used `try_eat` here since it's a single token, matching your keyword array pattern
            Spanned::new(Visibility::Shared, token.span)
        } else {
            // Fallback to Private. We anchor the span to the current parser index
            // so miette has a valid spot to point to if needed.
            let current_span = self.current().span;
            Spanned::new(Visibility::Private, current_span)
        };

        // 2. Parse Mutability
        let mutability =
            match self.try_eat_many([tok!(kw Keyword::Const), tok!(kw Keyword::Mut)])? {
                Some(token) => match token.kind {
                    tok!(kw Keyword::Const) => Spanned::new(Mutability::Immutable, token.span),
                    tok!(kw Keyword::Mut) => Spanned::new(Mutability::Mutable, token.span),
                    _ => unreachable!("try_eat_many returned an unexpected token keyword"),
                },
                None => {
                    let current_span = self.current().span;
                    Spanned::new(Mutability::ImplicitlyImmutable, current_span)
                }
            };

        Some((visibility, mutability))
    }
}
