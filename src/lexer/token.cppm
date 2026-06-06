module;
#include "../def.hpp"
export module lexer.token;

import std;
import lexer.locus;

namespace qu::lexer
{
    export class Token {
        using This = Token;
      public:
        enum Kind {
            Pub,
            Fn,
            Module,
            Use,
            Return,
            If,
            Else,
            For,
            In,
            As,
            Type,
            Const,
            Let,
            Mut,

            I8,
            I16,
            I32,
            I64,
            U8,
            U16,
            U32,
            U64,
            String,
            Char,
            Bool,

            Star,
            Add,
            Minus,
            Slash,
            StarAssign,
            AddAssign,
            MinusAssign,
            SlashAssign,
            Assign,
            Equal,
            NotEqual,
            Not,

            Bar,       // |
            BarAssign, // |=
            Amp,       // &
            AmpAssign, // &=
            Arrow,     // ->

            OpenBrace,
            CloseBrace,
            OpenParen,
            CloseParen,
            OpenBrack,
            CloseBrack,
            SemiColon,
            Colon,
            Comma,
            Dot,

            IntLiteral,
            StringLiteral,
            CharLiteral,
            True,
            False,
            Identifier,

            EndOfFile,
        };


    public:
        Locus locus;
        Kind kind;

        Token(): Token(Kind::EndOfFile) {}
        Token(Kind kind) : kind(kind) {}
        Token(Kind kind, Locus locus) : locus(locus), kind(kind) {}

        auto to_string(this This self) -> std::string_view {
            switch (self.kind) {
            case This::Pub:
                return "pub";
            case This::Return:
                return "return";
            case This::Module:
                return "module";
            case This::Use:
                return "use";
            case This::Fn:
                return "fn";
            case This::Identifier:
                return "identifier";
            case This::OpenParen:
                return "(";
            default: {
                UNREACHABLE("{}", (int)self.kind);
            } break;
            }
        }
        operator This::Kind() { return kind; }
    };
} // namespace qu::lexer
