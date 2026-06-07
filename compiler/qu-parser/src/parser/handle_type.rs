use qu_ast::{type_hint::{IntegerWidth, Type, TypeData, TypeRef}};

use crate::{PResult, parse_context::ParseContext, tok, token::{Keyword, Operator, TokenKind}};


pub(super) enum TypeContext {
    Parameter,
    Return,
    Variable,
}

pub(super) fn parse_type_hint(
    ctx: &mut ParseContext,
    context: TypeContext,
) -> PResult<TypeRef> {
    let mutability = if ctx.try_eat(tok!(kw Keyword::Mut))? {
        qu_ast::type_hint::Mutability::Mutable
    } else {
        ctx.try_eat(tok!(kw Keyword::Const))?;
        qu_ast::type_hint::Mutability::Immutable
    };

    let data = match ctx.current_kind() {
        // function-type: 'fn', signature;
        // signature: `(`, type, { `,` type }*, `)`, [ `->`, type ];
        tok!(kw Keyword::Fn) => todo!("parse-function-sig"),
        // pointer-type: '*', type;
        TokenKind::Operator(Operator::Star) => {
            let inner = parse_type_hint(ctx, TypeContext::Parameter)?;
            TypeData::Pointer(inner)
        }
        TokenKind::Identifier => {
            let span = ctx.current().span;
            let name = ctx.slice(span);
            ctx.next()?;
            match name.as_str() {
                "u8" => TypeData::UnsignedInteger(IntegerWidth::Int8),
                "u16" => TypeData::UnsignedInteger(IntegerWidth::Int16),
                "u32" => TypeData::UnsignedInteger(IntegerWidth::Int32),
                "u64" => TypeData::UnsignedInteger(IntegerWidth::Int64),
                "i8" => TypeData::SignedInteger(IntegerWidth::Int8),
                "i16" => TypeData::SignedInteger(IntegerWidth::Int16),
                "i32" => TypeData::SignedInteger(IntegerWidth::Int32),
                "i64" => TypeData::SignedInteger(IntegerWidth::Int64),
                "char" => TypeData::Char,
                "bool" => TypeData::Bool,
                "string" => TypeData::String,
                "void" => TypeData::Void,
                // TODO: parse generic arguments and qualifier-path
                //  e.g. `Vec[u8]`, `Std.Vec.Vec[u8]`, `Vec[Vec[u8]]`
                _ => TypeData::Named(qu_ast::Name { span, value: name }),
            }
        }
        _ => todo!()
    };
    
    Some(Type::new(ctx.current().span, data, mutability))
}
