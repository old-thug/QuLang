export module parser.state;

import std;
import lexer.token;

namespace qu::parser
{
    export struct State {
        enum Kind {
            GlobalDeclarations,
            FunctionDefinition,
            FunctionParameters,
        };

        Kind kind;
        bool has_error;
        std::optional<lexer::Token> begin;

        operator Kind() { return kind; }
        State(Kind kind) : kind(kind), has_error(false), begin(std::nullopt) {}

        auto is_errored(this State& state) -> bool { return state.has_error; }
    };
} // namespace qu::parser
