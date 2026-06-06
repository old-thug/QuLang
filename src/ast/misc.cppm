export module ast.misc;

import std;
import lexer.locus;

export namespace qu::ast::misc
{
    struct Name {
        lexer::Locus locus;
        std::string value;
    };
}
