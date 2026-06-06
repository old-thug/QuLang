module;
#include "../def.hpp"
export module parser;

import std;

export import parser.def;
export import parser.utils;
import parser.stmt;
import lexer.token;

namespace qu::parser
{
    export auto parse_module(Parser &parser) -> std::expected<int, int> {
        auto ctx = ParseContext(parser);
        ctx.next();

        while (!ctx.is_done()) {
            auto stmt = parse_statement(ctx);
            if (!stmt.has_value()) {
                UNREACHABLE();
            }
        }
        return std::unexpected(40);
    }
} // namespace qu::parser
