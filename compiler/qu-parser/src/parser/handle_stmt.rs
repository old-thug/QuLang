use qu_ast::{
    Visibility,
    stmt::{FunctionDeclKind, FunctionParameter},
    type_hint::{Mutability, TypeRef},
};
use qu_diagnostics::{Diagnostic, Severity, span::Spanned};

use crate::{
    PResult,
    parse_context::ParseContext,
    parser::{
        handle_expr::{Precedence, parse_block_, parse_expression},
        handle_type::{TypeContext, parse_type_hint},
    },
    tok,
    token::{Keyword, Operator, Separator, Token, TokenKind},
};

pub(super) fn parse_statement(ctx: &mut ParseContext) -> PResult<qu_ast::stmt::StmtRef> {
    let (visability, mutability) = ctx.collect_specifiers()?;
    let result = match ctx.current().kind {
        tok!(kw Keyword::Fn) => parse_function(ctx, FunctionDeclKind::Free, visability, mutability),
        tok!(kw Keyword::Return) => parse_return(ctx),
        tok!(kw Keyword::Let) => parse_vardecl(ctx, visability, mutability),
        _ => {
            let expr = parse_expression(ctx, Precedence::None)?;
            Some(qu_ast::stmt::Stmt::new_expr(expr.span, expr))
        }
    };
    result
}

fn parse_name_(ctx: &mut ParseContext, err: String) -> PResult<qu_ast::Name> {
    match ctx.try_eat_many([TokenKind::Identifier])? {
        Some(tok) => {
            return Some(qu_ast::Name {
                span: tok.span,
                value: ctx.slice(tok.span),
            });
        }
        None => {
            ctx.emit(Diagnostic::new(
                Severity::Error,
                "missing name".to_string(),
                ctx.current().span,
                err,
            ));
            return None;
        }
    }
}

pub(super) fn parse_name(ctx: &mut ParseContext, after: TokenKind) -> PResult<qu_ast::Name> {
    match ctx.try_eat_many([TokenKind::Identifier])? {
        Some(tok) => {
            return Some(qu_ast::Name {
                span: tok.span,
                value: ctx.slice(tok.span),
            });
        }
        None => {
            ctx.emit(Diagnostic::new(
                Severity::Error,
                "missing name".to_string(),
                ctx.current().span,
                format!("expected name after {after:?}"),
            ));
            return None;
        }
    }
}

pub(super) fn parse_function_sig(
    ctx: &mut ParseContext,
) -> PResult<qu_ast::stmt::FunctionPrototype> {
    let mut parameters = Vec::new();
    // Expect '(' to start parameters
    ctx.eat([tok!(sp Separator::OpenParen)])?;

    while !ctx.is_sp(Separator::CloseParen) {
        let takes_ownership = ctx.try_eat(tok!(kw Keyword::Move))?;
        let name = parse_name_(ctx, "expected parameter name".to_string())?;

        // 1. Optional Type Hint: If we can eat a `:`, parse the type
        let type_hint = if ctx.try_eat(tok!(sp Separator::Colon))? {
            Some(parse_type_hint(ctx, TypeContext::Parameter)?)
        } else {
            None
        };

        // 2. Optional Default Value: If we can eat a `=`, parse the expression
        let default_value = if ctx.try_eat(tok!(op Operator::Assign))? {
            Some(parse_expression(ctx, Precedence::None)?)
        } else {
            None
        };

        parameters.push(FunctionParameter {
            name,
            type_hint,
            default_value,
            takes_ownership,
        });

        // Parameters must be comma-separated. If there's no trailing comma,
        // we break and expect the closing parenthesis next.
        if !ctx.try_eat(tok!(sp Separator::Comma))? {
            break;
        }
    }

    // Expect ')' to close parameters
    ctx.eat([tok!(sp Separator::CloseParen)])?;

    // 3. Optional Return Type: If we see `->`, parse the return type
    let return_type = if ctx.try_eat(tok!(sp Separator::Arrow))? {
        Some(parse_type_hint(ctx, TypeContext::Return)?)
    } else {
        None
    };

    Some(qu_ast::stmt::FunctionPrototype {
        parameters,
        return_type,
    })
}

pub(super) fn parse_generics(
    ctx: &mut ParseContext,
    tok: TokenKind,
) -> PResult<qu_ast::generics::Generics> {
    let generics = qu_ast::generics::Generics::new();
    if ctx.try_eat(tok!(sp Separator::OpenBracket))? {
        todo!();
        // while !ctx.equals(tok!(sp Separator::CloseBracket)) {
        //     let param_name = parse_name(ctx, tok)?;
        //     if ctx.try_eat(tok!(sp Separator::Colon))? {
        //     }
        // }
    }
    Some(generics)
}

pub(super) fn parse_function(
    ctx: &mut ParseContext,
    kind: FunctionDeclKind,
    visibility: Spanned<Visibility>,
    mutability: Spanned<Mutability>,
) -> PResult<qu_ast::stmt::StmtRef> {
    let fn_kw = ctx.eat([tok!(kw Keyword::Fn)])?;
    let name_ident = parse_name(ctx, tok!(kw Keyword::Fn))?;
    let generics = parse_generics(ctx, tok!(kw Keyword::Fn))?;
    let prototype = parse_function_sig(ctx)?;
    // If we don't encounter beginning of a function body and also we are not in an extern function
    // then we have an error
    if !ctx.is_sp(Separator::OpenBrace) && !matches!(kind, FunctionDeclKind::Extern) {
        ctx.emit(Diagnostic::new(
            Severity::Error,
            "missing function body".to_string(),
            ctx.current().span,
            format!("expected function body"),
        ));
        ctx.skip_until(true, |t| t.is_sp(Separator::OpenBrace))?;
    }

    if matches!(mutability.get(), Mutability::Mutable) {
        ctx.emit(
            Diagnostic::new(
                Severity::Error,
                "invalid token".to_string(),
                mutability.span().clone(),
                "functions cannot be annotated with `mut`".into(),
            )
            .with_help("consider removing `mut`".to_string()),
        );
    }

    ctx.eat([tok!(sp Separator::OpenBrace)])?;
    let body = parse_block_(ctx)?;
    Some(qu_ast::stmt::Stmt::new_function_definition(
            fn_kw.span.cover(ctx.previous().span),
            visibility.get().clone(),
            mutability.get().clone(),
            kind,
            name_ident,
            generics,
            prototype,
            Some(body),
    ))
}

pub(super) fn parse_return(ctx: &mut ParseContext) -> PResult<qu_ast::stmt::StmtRef> {
    let return_kw = ctx.eat([tok!(kw Keyword::Return)])?;
    let expr = parse_expression(ctx, Precedence::None)?;
    ctx.eat([tok!(sp Separator::SemiColon)])?;
    Some(qu_ast::stmt::Stmt::new_return(return_kw.span, expr))
}

pub(super) fn parse_vardecl(
    ctx: &mut ParseContext,
    visibility: Spanned<Visibility>,
    mutability: Spanned<Mutability>,
) -> PResult<qu_ast::stmt::StmtRef> {
    let let_kw = ctx.eat([tok!(kw Keyword::Let)])?;
    let name = parse_name(ctx, let_kw.kind)?;
    let type_hint = if ctx.try_eat(tok!(sp Separator::Colon))? {
        Some(parse_type_hint(ctx, TypeContext::Variable)?)
    } else {
        None
    };
    ctx.eat_or_else(tok!(op Operator::Assign),|t| t, |this| { 
        this.skip_until(true, |t| t.is_sp(Separator::SemiColon)); 
        None
    })?;
    let initializer = match parse_expression(ctx, Precedence::None) {
        Some(expr) => expr,
        None => {
            ctx.skip_until(true, |t| t.is_sp(Separator::SemiColon))?;
            return None;
        }
    };
    ctx.eat([tok!(sp Separator::SemiColon)])?;
    Some(qu_ast::stmt::Stmt::new_variable_decl(
        mutability.span().cover(ctx.previous().span),
        mutability.get().clone(),
        name,
        type_hint,
        initializer,
    ))
}
