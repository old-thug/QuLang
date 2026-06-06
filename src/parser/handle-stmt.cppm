module;
#include "../def.hpp"
export module parser.stmt;

import std;
import ast;
import diagnostic;
import lexer.token;
import parser.utils;
import parser.state;

namespace qu::parser
{
    using lexer::Token;

    auto parse_function(ParseContext &ctx)
        -> std::optional<ast::stmt::StmtRef>;

    export auto parse_statement(ParseContext &ctx)
        -> std::optional<ast::stmt::StmtRef> {
        auto head =
            ctx.eat({Token::Fn, Token::Module, Token::Use, Token::Type,
                     Token::Const, Token::Let});
        if (head.has_value()) {
            switch (head->kind) {
            case Token::Fn:
                return parse_function(ctx);
            default:
                UNREACHABLE();
            }
        }
        UNREACHABLE();
    }

    auto parse_name(ParseContext &ctx, ast::misc::Name &name) -> bool {
        if (ctx.equals(Token::Identifier)) {
            name.locus = ctx.current().locus;
            name.value = ctx.slice(name.locus);
            ctx.next();
            return true;
        }
        return false;
    }

    auto parse_function_prototype(
        ParseContext &ctx, ast::stmt::FunctionDefinition::Prototype &prototype)
        -> bool {
        // Parse function parameters
        ctx.with_state(State::FunctionParameters, [](auto ctx) {
            if (!ctx.try_eat({ Token::OpenParen })) {
                if (!ctx.get_state()->is_errored()) {
                    ctx.emit_diag(Diagnostic(Severity::Error, "unexpected token", ctx.current().locus, "expected function parameter type list"));
                }
                ctx.get_state()->has_error = true;
                return false;
            }

            while (!ctx.equals(Token::CloseParen)) {
                auto parameter = ast::stmt::FunctionDefinition::Parameter{};
                if (!parse_name(ctx, parameter.name)) {
                    if (!ctx.get_state()->is_errored()) {
                        ctx.emit_diag(Diagnostic(Severity::Error, "unexpected token", ctx.current().locus, "parameter name"));
                    }
                    ctx.get_state()->has_error = true;
                    if (!ctx.skip_to({ Token::Colon, Token::Comma })) {
                        return false;
                    }
                }

                if (ctx.try_eat({ Token::Colon })) {

                }
            }
            return true;
        });
        UNREACHABLE();
    }

    auto parse_function(ParseContext &ctx)
        -> std::optional<ast::stmt::StmtRef> {
        auto begin = ctx.previous();
        auto function = ast::stmt::FunctionDefinition{};
        // Parse function name
        if (!parse_name(ctx, function.name)) {
            ctx.emit_diag(Diagnostic(Severity::Error, "unexpected token", ctx.current().locus, "expected function name after `fn`"));
            if (!ctx.skip_to({Token::OpenBrack, Token::OpenParen}).has_value()) {
                return std::nullopt;
            }
        }
        if (ctx.equals(Token::OpenBrack)) {
            UNREACHABLE("error: expected function generic parameters");
        }

        if (!parse_function_prototype(ctx, function.prototype)) {
            if (!ctx.skip_to({Token::OpenBrace, Token::Arrow}).has_value()) {
                return std::nullopt;
            }
        }

        UNREACHABLE();
    }
} // namespace qu::parser
