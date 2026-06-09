use qu_ast::{
    expr::{BinaryOperator, Expr, ExprData, ExprRef},
    stmt,
};
use qu_diagnostics::{Diagnostic, Severity};

use crate::{
    PResult,
    parse_context::ParseContext,
    parser::parse_statement,
    tok,
    token::{Literal, Operator, Separator, TokenKind},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Index,
    MemberAccess,
    Primary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Associativity {
    Left,
    Right,
}

type ParsePrefix = fn(&mut ParseContext) -> PResult<ExprRef>;
type ParseInfix = fn(&mut ParseContext, ExprRef) -> PResult<ExprRef>;

type Rule = (
    Precedence,
    Associativity,
    Option<ParsePrefix>,
    Option<ParseInfix>,
);

const RULES: &[(TokenKind, Rule)] = &[
    (
        TokenKind::Operator(Operator::Add),
        (
            Precedence::Term,
            Associativity::Left,
            None,
            Some(parse_term_expr),
        ),
    ),
    (
        TokenKind::Operator(Operator::Minus),
        (
            Precedence::Term,
            Associativity::Left,
            None,
            Some(parse_term_expr),
        ),
    ),
    (
        TokenKind::Operator(Operator::Pipe), // |>
        (
            Precedence::Factor,
            Associativity::Left,
            None,
            Some(parse_pipe_expr),
        ),
    ),
    (
        TokenKind::Literal(Literal::Integer),
        (
            Precedence::Primary,
            Associativity::Left,
            Some(parse_primary_expr),
            None,
        ),
    ),
    // Identifier
    (
        TokenKind::Identifier,
        (
            Precedence::Primary,
            Associativity::Left,
            Some(parse_primary_expr),
            None,
        ),
    ),
    (
        TokenKind::Literal(Literal::String),
        (
            Precedence::Primary,
            Associativity::Left,
            Some(parse_primary_expr),
            None,
        ),
    ),
];

fn get_rule(tok: TokenKind) -> Option<Rule> {
    RULES
        .iter()
        .find(|(kind, _)| *kind == tok)
        .map(|(_, rule)| *rule)
}

pub(super) fn parse_expression(ctx: &mut ParseContext, precedence: Precedence) -> PResult<ExprRef> {
    // 1. Parse the prefix expression
    let mut left = match get_rule(ctx.current_kind()) {
        Some((_, _, Some(prefix), _)) => prefix(ctx)?,
        _ => {
            ctx.emit(Diagnostic::new(
                Severity::Error,
                "unexpected token".to_string(),
                ctx.current().span,
                format!("expected expression, got {:?}", ctx.current_kind()),
            ));
            return None;
        }
    };

    // 2. Loop and consume infix operators with higher precedence
    loop {
        match get_rule(ctx.current_kind()) {
            Some((next_precedence, assoc, _, Some(infix))) => {
                let should_break = if assoc == Associativity::Right {
                    precedence > next_precedence
                } else {
                    precedence >= next_precedence
                };

                if should_break {
                    break;
                }

                // Consume the infix operator token itself before parsing the rest
                ctx.next()?;
                left = infix(ctx, left)?;
            }
            _ => break,
        }
    }

    Some(left)
}

// functions

fn parse_call_expr(ctx: &mut ParseContext, _callee: ExprRef) -> PResult<ExprRef> {
    todo!()
}

fn parse_index_expr(ctx: &mut ParseContext, _reciever: ExprRef) -> PResult<ExprRef> {
    todo!()
}

fn parse_member_access_expr(ctx: &mut ParseContext, _reciever: ExprRef) -> PResult<ExprRef> {
    todo!()
}

fn parse_primary_expr(ctx: &mut ParseContext) -> PResult<ExprRef> {
    let token = ctx.next()?;
    match token.kind {
        TokenKind::Identifier => {
            return Some(qu_ast::expr::Expr::new(
                token.span,
                ExprData::Identifier(ctx.slice(token.span)),
            ));
        }
        tok!(lt Literal::Integer) => {
            let value = ctx
                .slice(token.span)
                .parse::<i64>()
                .expect("integer literal");
            return Some(qu_ast::expr::Expr::new(
                token.span,
                ExprData::Integer(value),
            ));
        }
        _ => todo!(),
    }
}

fn parse_unary_expr(ctx: &mut ParseContext) -> PResult<ExprRef> {
    todo!()
}

fn parse_factor_expr(ctx: &mut ParseContext, _lhs: ExprRef) -> PResult<ExprRef> {
    todo!()
}

fn parse_term_expr(ctx: &mut ParseContext, _lhs: ExprRef) -> PResult<ExprRef> {
    let op_tok = ctx.previous();
    let op = match op_tok.kind {
        TokenKind::Operator(op) => match op {
            Operator::Add => BinaryOperator::Add,
            Operator::Minus => BinaryOperator::Sub,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    let rhs = parse_expression(ctx, Precedence::Factor)?;
    Some(qu_ast::expr::Expr::new_binary(
        op_tok.span.cover(rhs.span),
        _lhs,
        rhs,
        op,
    ))
}

fn parse_pipe_expr(ctx: &mut ParseContext, _lhs: ExprRef) -> PResult<ExprRef> {
    todo!()
}

pub(super) fn parse_block_expr(ctx: &mut ParseContext) -> PResult<ExprRef> {
    let begin = ctx.current().span;
    let block = parse_block_(ctx)?;
    let end = ctx.current().span;
    Some(Expr::new(begin.cover(end), ExprData::Block(block)))
}

pub(super) fn parse_block_(ctx: &mut ParseContext) -> PResult<qu_ast::expr::Block> {
    let mut stmts = vec![];
    while !ctx.is_sp(Separator::CloseBrace) {
        let stmt = parse_statement(ctx)?;
        if stmt.is_expr() && !ctx.is_sp(Separator::SemiColon) {
            ctx.skip_until(false, |t| t.is_sp(Separator::CloseBrace))?;
            ctx.emit(
                Diagnostic::new(
                    Severity::Error,
                    "missing semicolon".to_string(),
                    ctx.current().span,
                    format!("expected semicolon here"),
                )
                .with_label(
                    format!(
                        "consider using `return {};` to return the value",
                        ctx.slice(stmt.span().clone())
                    ),
                    stmt.span().clone(),
                ),
            );
            ctx.skip_until(true, |t| t.is_sp(Separator::SemiColon))?;
        } else if stmt.is_expr() {
            ctx.eat([tok!(sp Separator::SemiColon)])?;
        }
        stmts.push(stmt);
    }
    ctx.eat([tok!(sp Separator::CloseBrace)])?;
    Some(qu_ast::expr::Block { stmts })
}
