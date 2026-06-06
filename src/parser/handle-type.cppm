module;
#include "../def.hpp"
export module parser.type;


import std;
import ast;
import parser.utils;

namespace qu::parser
{
    auto parse_type_hint(ParseContext &ctx) -> std::optional<ast::type_hint::TypeRef> {
        UNREACHABLE();
    }
} // namespace qu::parser
