module;
#include "../def.hpp"
#include <cctype>
#include <cstdio>
export module lexer;

import std;
import source;
import diagnostic;

export import lexer.locus;
export import lexer.token;

namespace qu::lexer
{
    export struct Lexer {
        using This = Lexer;
        struct Cursor {
            std::size_t line   = 1;
            std::size_t column = 1;
            std::size_t index  = 0;
        };

        qu::SourceId id;
        std::string_view source;
        std::size_t end;
        This::Cursor front;
        This::Cursor back;

        auto cur(this This &self) -> char {
            if (self.front.index >= self.end) {
                return EOF;
            }
            return self.source[self.front.index];
        }

        auto next(this This &self) -> char {
            auto c = self.cur();
            if (c == '\n') {
                self.front.line += 1;
                self.front.column = 1;
            } else {
                self.front.column += 1;
            }
            self.front.index += 1;
            return c;
        }

        auto eat(this This &self, char c) -> bool {
            if (self.cur() == c) {
                self.next();
                return true;
            }
            return false;
        }

        auto eat(this This &self, std::string_view prefix) {
            if (self.source.substr(self.front.index).starts_with(prefix)) {
                for (std::size_t n = 0; n < prefix.length(); ++n) {
                    self.next();
                }
                return true;
            }
            return false;
        }

        auto here(this This &self) -> Locus {
            return {
                .first_line  = self.back.line,
                .last_line   = self.front.line,
                .first_col   = self.back.column,
                .last_col    = self.front.column,
                .first_index = self.back.index,
                .last_index  = self.front.index,
                .source_id   = self.id,
            };
        }

        struct TokenSpec {
            std::string_view str;
            Token token;
        };

      public:
        static auto init(const std::string_view &source, qu::SourceId id)
            -> Lexer {
            return {
                .id     = id,
                .source = source,
                .end    = source.size(),
                .front  = Cursor{},
                .back   = Cursor{},
            };
        }

        auto next_token(this This &self)
            -> std::expected<Token, qu::Diagnostic> {
            for (;;) {
                if (self.eat('\n') || self.eat('\t') || self.eat('\r') ||
                    self.eat(' '))
                    continue;

                if (self.eat("//")) {
                    while (self.cur() != EOF && self.cur() != '\n') {
                        self.next();
                    }
                    continue;
                }

                break;
            }

            if (self.cur() == EOF) {
                return Token(Token::EndOfFile, self.here());
            }

            self.back = self.front;

            if (std::isalpha(self.cur()) || self.cur() == '_') {
                while (self.cur() != EOF &&
                       (std::isalnum(self.cur()) || self.cur() == '_')) {
                    self.next();
                }
                auto length = self.front.index - self.back.index;
                auto slice  = self.source.substr(self.back.index, length);
                for (auto [str, tok] : (TokenSpec[]){
                         {"fn", Token::Fn},         {"pub", Token::Pub},
                         {"const", Token::Const},   {"let", Token::Let},
                         {"mut", Token::Mut},       {"module", Token::Module},
                         {"return", Token::Return}, {"use", Token::Use},
                         {"if", Token::If},         {"else", Token::Else},
                         {"for", Token::For},       {"i8", Token::I8},
                         {"i16", Token::I16},       {"i32", Token::I32},
                         {"i64", Token::I64},       {"u8", Token::U8},
                         {"u16", Token::U16},       {"u32", Token::U32},
                         {"u64", Token::U64},       {"as", Token::As},
                         {"type", Token::Type},
                         {"in", Token::In}}) {
                    if (slice.compare(str) == 0) {
                        return Token(tok.kind, self.here());
                    }
                    return Token(Token::Identifier, self.here());
                }
            }

            for (auto [str, tok] : (TokenSpec[]){
                     {"+=", Token::AddAssign},
                     {"-=", Token::MinusAssign},
                     {"*=", Token::StarAssign},
                     {"/=", Token::SlashAssign},
                     {"!=", Token::NotEqual},
                     {"==", Token::Equal},
                     {"->", Token::Arrow},
                     {"=", Token::Assign},
                     {"!", Token::Not},
                     {"+", Token::Add},
                     {"-", Token::Minus},
                     {"*", Token::Star},
                     {"/", Token::Slash},
                     {";", Token::SemiColon},
                     {":", Token::Colon},
                     {",", Token::Comma},
                     {"(", Token::OpenParen},
                     {")", Token::CloseParen},
                     {"[", Token::OpenBrack},
                     {"]", Token::CloseBrack},
                     {"{", Token::OpenBrace},
                     {"}", Token::CloseBrace},
                 }) {
                if (self.eat(str)) {
                    return Token(tok.kind, self.here());
                }
            }

            UNREACHABLE();
        }
    };
} // namespace qu::lexer
