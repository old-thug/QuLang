mod handle_expr;
mod handle_stmt;
mod handle_type;
mod parse_state;

use qu_ast::Ast;
use qu_diagnostics::Diagnostic;
use qu_source::SourceId;

use crate::{
    lexer::Lexer,
    parse_context::ParseContext,
    parser::handle_stmt::parse_statement,
    tok,
    token::{Keyword, Token}, token_stream::TokenStream,
};

pub type PResult<T> = Option<T>;

#[derive(Debug)]
pub struct Parser<'a> {
    pub(super) ts: TokenStream,
    pub(super) source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, ts: TokenStream) -> Self {
        let mut parser = Self {
            ts,
            source,
        };
        parser
    }

    pub fn next(&mut self) -> Token {
        self.ts.next()
    }

    pub fn parse(&mut self) -> Result<Ast, Vec<Diagnostic>> {
        let mut ctx = ParseContext::new(self);
        let mut ast = Ast::new();
        while !ctx.is_done() {
            match parse_statement(&mut ctx) {
                Some(stmt) => {
                    ast.push(stmt);
                }
                None => {
                    ctx.skip_to(
                        false,
                        [
                            tok!(kw Keyword::Fn),
                            tok!(kw Keyword::Pub),
                            tok!(kw Keyword::Module),
                            tok!(kw Keyword::Use),
                            tok!(kw Keyword::Extern),
                        ],
                    );
                }
            }
        }
        if ctx.diagnostics.len() != 0 {
            return Err(ctx.diagnostics);
        } else {
            return Ok(ast);
        }
    }
}
