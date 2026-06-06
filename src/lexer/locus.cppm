export module lexer.locus;

import std;

namespace qu::lexer
{
    export struct Locus {
        std::size_t first_line, last_line;
        std::size_t first_col, last_col;
        std::size_t first_index, last_index;
        std::size_t source_id;

        auto span_to(this Locus self, Locus other) -> Locus {
            return {
                .first_line = self.first_line, .last_line = other.last_line,
                .first_col = self.first_col, .last_col = other.last_col,
                .first_index = self.first_index, .last_index = other.last_index,
                .source_id = self.source_id,
            };
        }
    };
} // namespace qu::lexer
