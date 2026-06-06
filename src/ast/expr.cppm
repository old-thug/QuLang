module;
#include <cmath>
#include <cstdint>
export module ast.expr;

import std;
import lexer.locus;

export namespace qu::ast::expr
{
    struct Expr;
    using ExprRef = std::unique_ptr<Expr>;

    struct Identifier {
        std::string value;
    };

    using ConstantInteger = std::int64_t;
    using ConstantFloat   = std::double_t;
    using ConstantBool    = bool;
    struct ConstantString {
        enum Kind {
            Raw,
            NullTerminated,
            Regular,
        };
        std::string value;
    };

    struct FunctionCall {
        ExprRef callee;
        std::vector<ExprRef> arguments;
    };

    using ExprData =
        std::variant<Identifier, ConstantInteger, ConstantFloat,
                         ConstantBool, ConstantString, FunctionCall>;

    struct Expr {
        lexer::Locus locus;
        ExprData data;
    };

    auto make_expr(lexer::Locus locus, ExprData data) -> ExprRef {
        return std::make_unique<Expr>(locus, std::move(data));
    }
} // namespace qu::ast::expr
