export module ast.stmt;

import std;
import ast.expr;
import ast.misc;
import ast.type_hint;
import lexer.locus;

export namespace qu::ast::stmt
{
    struct Stmt;
    using StmtRef = std::unique_ptr<Stmt>;

    struct VariableDeclaration {
        type_hint::Mutability mutability;
        misc::Name name;
        std::optional<type_hint::TypeRef> type_hint;
        expr::ExprRef initializer;
    };

    struct FunctionDefinition {
        struct Parameter {
            // Implicity Argument pass mode.
            // fn delete_buffer(move buffer: []char) {...}
            enum PassMode {
                Move,
                Copy,
                Ref,
            };

            misc::Name name;
            std::optional<type_hint::TypeRef> type_hint;
            std::optional<expr::ExprRef> default_value;
        };

        struct Prototype {
            std::vector<Parameter> parameters;
            std::optional<type_hint::TypeRef> return_type;
        };

        misc::Name name;
        Prototype prototype;
        std::optional<StmtRef> body;
    };

    struct Block {
        lexer::Locus begin, end;
        std::vector<StmtRef> stmts;
    };

    struct ExternDefinition {
        enum class ABI {
            C,
            Cpp,
            System,
        } abi;
        std::vector<StmtRef> definitions;
    };

    struct StructDefinition {};
    struct EnumDefinition {};
    struct UseDeclaration {};
    struct ModuleSpec {};

    using StmtData =
        std::variant<VariableDeclaration, FunctionDefinition, Block,
                         ExternDefinition, StructDefinition, EnumDefinition,
                         ModuleSpec, UseDeclaration>;

    struct Stmt {
        lexer::Locus locus;
        StmtData data;
    };

    auto make_stmt(lexer::Locus locus, StmtData data) -> StmtRef {
        return std::make_unique<Stmt>(locus, std::move(data));
    }
} // namespace qu::ast::stmt
