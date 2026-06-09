use qu_span::Span;

use crate::{
    Name, Visibility,
    expr::{self, ExprRef},
    generics::Generics,
    type_hint::{Mutability, TypeRef},
};

#[derive(Debug, Clone)]
pub struct Stmt {
    span: Span,
    data: StmtData,
}

pub type StmtRef = Box<Stmt>;

#[derive(Debug, Clone)]
pub enum StmtData {
    VariableDecl(VariableDecl),
    FunctionDefinition(FunctionDefinition),
    TypeDefinition(TypeDefinition),
    UseDecl(UseDecl),
    ModuleSpec(ModuleSpec),
    Return(Return),
    Expr(ExprRef),
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub mutability: Mutability,
    pub name: Name,
    pub type_hint: Option<TypeRef>,
    pub initializer: ExprRef,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub takes_ownership: bool,
    pub name: Name,
    pub type_hint: Option<TypeRef>,
    pub default_value: Option<ExprRef>,
}

#[derive(Debug, Clone)]
pub struct FunctionPrototype {
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<TypeRef>,
}

#[derive(Debug, Clone)]
pub enum FunctionDeclKind {
    Extern,
    Free,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub kind: FunctionDeclKind,
    pub name: Name,
    pub generics: Generics,
    pub prototype: FunctionPrototype,
    pub body: Option<ExprRef>,
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    name: Name,
    is_distinct: bool,
    is_opaque: bool,
    value: TypeRef,
}

#[derive(Debug, Clone)]
pub enum UsePath {
    // use std;
    Name(Name),
    // use Std.Fs;
    // use Std.Fs.Path;
    // use Std.String.{Builder, Util};
    Pair(Name, Box<UsePath>),
    // use Std.{Fs, String, Vec};
    Many(Vec<UsePath>),
}

#[derive(Debug, Clone)]
pub struct UseDecl {
    path: UsePath,
}

#[derive(Debug, Clone)]
pub struct ModuleSpec {
    name: Name,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub expr: ExprRef,
}

impl Stmt {
    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn data(&self) -> &StmtData {
        &self.data
    }

    pub fn is_expr(&self) -> bool {
        matches!(self.data, StmtData::Expr(_))
    }

    pub fn new(span: Span, data: StmtData) -> StmtRef {
        Box::new(Stmt { span, data })
    }

    pub fn new_return(span: Span, expr: ExprRef) -> StmtRef {
        Self::new(span, StmtData::Return(Return { expr }))
    }

    pub fn new_expr(span: Span, expr: ExprRef) -> StmtRef {
        Self::new(span, StmtData::Expr(expr))
    }

    pub fn new_function_definition(
        span: Span,
        visibility: Visibility,
        // Mutable -> Error
        // ImplicitlyMutable -> No-Op
        // Immutable (keyword const) -> Compiletime
        mutability: Mutability,
        kind: FunctionDeclKind,
        name: Name,
        generics: Generics,
        prototype: FunctionPrototype,
        body: Option<ExprRef>,
    ) -> StmtRef {
        Self::new(
            span,
            StmtData::FunctionDefinition(FunctionDefinition {
                visibility,
                mutability,
                kind,
                name,
                generics,
                prototype,
                body,
            }),
        )
    }

    pub fn new_variable_decl(
        span: Span,
        mutability: Mutability,
        name: Name,
        type_hint: Option<TypeRef>,
        initializer: ExprRef,
    ) -> StmtRef {
        Self::new(
            span,
            StmtData::VariableDecl(VariableDecl {
                mutability,
                name,
                type_hint,
                initializer,
            }),
        )
    }

    pub fn new_module_spec(span: Span, name: Name) -> StmtRef {
        Self::new(span, StmtData::ModuleSpec(ModuleSpec { name }))
    }

    pub fn new_use_decl(span: Span, path: UsePath) -> StmtRef {
        Self::new(span, StmtData::UseDecl(UseDecl { path }))
    }

    pub fn new_type_definition(
        span: Span,
        name: Name,
        is_distinct: bool,
        is_opaque: bool,
        value: TypeRef,
    ) -> StmtRef {
        Self::new(
            span,
            StmtData::TypeDefinition(TypeDefinition {
                name,
                is_distinct,
                is_opaque,
                value,
            }),
        )
    }
}

impl FunctionPrototype {
    pub fn new(parameters: Vec<FunctionParameter>, return_type: Option<TypeRef>) -> Self {
        Self {
            parameters,
            return_type,
        }
    }
}
