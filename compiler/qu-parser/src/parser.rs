mod handle_expr;
mod handle_stmt;
mod handle_type;
mod parse_state;

use qu_ast::Ast;
use qu_context::SourceId;
use qu_diagnostics::Diagnostic;

use crate::{
    lexer::Lexer,
    parse_context::ParseContext,
    parser::handle_stmt::parse_statement,
    tok,
    token::{Keyword, Token},
};

pub type PResult<T> = Option<T>;

#[derive(Debug)]
pub struct Parser<'a> {
    pub(super) lexer: Lexer<'a>,
    pub(super) current_token: Token,
    pub(super) previous_token: Token,
    pub(super) source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, source_id: SourceId) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(source, source_id),
            current_token: Token::default(),
            previous_token: Token::default(),
            source,
        };
        parser.next().unwrap_or_else(|_| panic!());
        parser
    }

    pub fn next(&mut self) -> Result<Token, Diagnostic> {
        self.previous_token = self.current_token;
        self.current_token = self.lexer.next_token()?;
        Ok(self.previous_token)
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
                    )
                    .unwrap();
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
