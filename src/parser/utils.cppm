module;
#include "../def.hpp"
export module parser.utils;

import std;
import diagnostic;
import lexer.token;
import lexer.locus;
import parser.def;
import parser.state;

export namespace qu::parser
{
    struct ParseContext {
    private:
        using This = ParseContext;
        Parser &parser;
    public:
        ParseContext(Parser &parser) : parser(parser) {}

        auto emit_diag(this This &self, qu::Diagnostic diag) {
            self.parser.diags->add(diag);
        }

        auto current(this This &self) -> lexer::Token {
            return self.parser.current_token;
        }

        auto previous(this This &self) -> lexer::Token {
            return self.parser.previous_token;
        }

        auto is_done(this This &self) -> bool {
            return self.parser.current_token == lexer::Token::EndOfFile; // or
                // self.parser.previous_token == lexer::Token::EndOfFile;
        }

        auto slice(this This &self, lexer::Locus locus) -> std::string_view {
            return
                self.parser.source.substr(locus.first_index, locus.last_index - locus.first_index);
        }

        auto match(this This &self, lexer::Token::Kind kind) -> bool {
            if (self.equals(kind)) {
                self.next();
                return true;
            }
            return false;
        }

        auto next(this This &self) -> lexer::Token {
            self.parser.previous_token = self.parser.current_token;
            auto tok                   = self.parser.lexer.next_token();
            if (tok.has_value()) {
                self.parser.current_token = tok.value();
            } else {
                self.emit_diag(tok.error());
            }
            return self.parser.previous_token;
        }

        auto equals(this This &self, lexer::Token::Kind kind) -> bool {
            return self.parser.current_token == kind;
        }

        auto eat(this This &self,
                 std::initializer_list<lexer::Token::Kind> kinds)
            -> std::optional<lexer::Token> {
            auto buffer = std::stringstream();
            auto len    = 0u;
            for (auto kind : kinds) {
                if (self.equals(kind)) {
                    auto tok = self.next();
                    return tok;
                }

                if (len != 0) {
                    buffer << ", ";
                }

                if (len != 0 && len == kinds.size() - 1) {
                    buffer << " or ";
                }

                buffer << std::format("`{}`", lexer::Token(kind).to_string());
            }

            self.emit_diag(Diagnostic(
                Severity::Error, "unexpected token", self.current().locus,
                std::format("expected {}; got `{}`", buffer.str(),
                            self.current().to_string())));
            return std::nullopt;
        }

        auto try_eat(this This &self, std::initializer_list<lexer::Token::Kind> kinds)
            -> std::optional<lexer::Token> {
            for (auto kind : kinds) {
                if (self.equals(kind)) {
                    auto tok = self.next();
                    return tok;
                }
            }
            return std::nullopt;
        }

        auto skip_to(this This &self, std::initializer_list<lexer::Token::Kind> kinds,
                     bool eat = false) -> std::optional<lexer::Token> {
            while (!self.is_done()) {
                for (auto kind : kinds) {
                    if (self.equals(kind)) {
                        auto tok = self.parser.previous_token;
                        if (eat)
                            self.next();
                        return tok;
                    }
                }
                self.next();
            }
            return std::nullopt;
        }

        template <typename Fn, typename U>
        auto with_state(this This &self, State state, Fn &&fn) -> U {
            self.parser.states.push_back(state);
            auto result = fn(self);
            self.parser.states.pop_back();
            return result;
        }

        template <typename Fn>
        auto with_state(this This &self, State state, Fn &&fn) -> void {
            self.parser.states.push_back(state);
            fn(self);
            self.parser.states.pop_back();
        }

        auto get_state(this This &self) -> State * {
            if (self.parser.states.size() == 0) return nullptr;
            return &self.parser.states[self.parser.states.size() - 1];
        }

        template <typename Fn, typename U>
        auto skip_and_do(this This &self,
                         Fn&& fn)
            -> std::optional<U> {
            Parser clone = self.parser;
            auto result  = fn(self);
            if (!result.has_value()) {
                self.parser = clone;
            }
            return result;
        }
    };
} // namespace qu::parser
