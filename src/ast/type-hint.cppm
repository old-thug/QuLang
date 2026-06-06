export module ast.type_hint;

import std;
import lexer.locus;

namespace qu::ast::type_hint
{
    export struct Type;
    export using TypeRef = std::unique_ptr<Type>;

    export enum class Mutability {
        Mutable,
        Immutable,
    };

    export enum class IntegerWidth {
        Int8,
        Int16,
        Int32,
        Int64,
    };
    export struct SignedInteger { IntegerWidth width;  };
    export struct UnsignedInteger { IntegerWidth width; };
    export struct Void {};
    export using TypeData = std::variant<SignedInteger, UnsignedInteger, Void>;

    struct Type {
        lexer::Locus locus;
        TypeData data;
        Mutability mutability;
    };

    auto make_type(lexer::Locus locus, Mutability mutability, TypeData data) -> TypeRef {
        return std::make_unique<Type>(locus, std::move(data));
    }
} // namespace qu::ast::type
