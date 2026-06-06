module;
#include "../def.hpp"
export module parser.def;

import std;
import lexer;
import context;
import source;
import parser.state;
import diagnostic;

namespace qu::parser
{
    export class Parser {
        using This = Parser;
        qu::Context *context;
        Parser(qu::Context *context, qu::DiagnosticPool *diags)
            : context(context), diags(diags) {}
      public:
        lexer::Lexer lexer;
        lexer::Token current_token, previous_token;
        std::string_view source;
        qu::DiagnosticPool *diags;
        std::vector<State> states;

        QU_METHOD auto init(qu::Context *context, qu::DiagnosticPool *diags, qu::SourceId id) -> std::optional<This> {
            auto parser = This(context, diags);
            auto source = context->get_source(id);
            if (!source.has_value()) return std::nullopt;
            parser.lexer = lexer::Lexer::init(source.value().get_content(), id);
            parser.states = {};
            parser.source = source.value().get_content();
            return parser;
        }

        auto parse(this This &self) -> std::expected<int, int>;
    };
}// namespace qu::parser
