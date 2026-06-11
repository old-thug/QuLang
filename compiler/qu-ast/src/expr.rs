use std::fmt::Binary;

use qu_span::Span;

use crate::{Name, stmt::StmtRef, type_hint::TypeRef};

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub data: ExprData,
}

pub type ExprRef = Box<Expr>;

#[derive(Debug, Clone)]
pub enum ExprData {
    Integer(i64),
    Float(f64),
    String(StringExpr),
    Bool(bool),
    Cast(TypeRef, ExprRef),
    Call(Call),
    Initializer(Vec<(Option<Name>, ExprRef)>),
    Index { reciever: ExprRef, index: ExprRef },
    MemberAccess { reciever: ExprRef, member: Name },
    Identifier(String),
    Block(Block),
    Yeild(ExprRef),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Tuple(Vec<ExprRef>),
    Unit,                       // Empty Tuple. ()
}

#[derive(Debug, Clone)]
pub struct StringExpr {
    kind: StringKind,
    value: String,
}

#[derive(Debug, Clone)]
pub enum StringKind {
    Nullterminated,
    Raw,
    Plain,
}

#[derive(Debug, Clone)]
pub enum CallArgument {
    Expr(ExprRef),
    Assign(Name, ExprRef),
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: ExprRef,
    pub arguments: Vec<CallArgument>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<StmtRef>,
}

#[derive(Debug, Clone)]
pub struct BinaryOperation {
    pub left: ExprRef,
    pub right: ExprRef,
    pub operator: BinaryOperator,
}

#[derive(Debug, Clone)]
pub struct UnaryOperation {
    operand: ExprRef,
    operator: UnaryOperator,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

// Grammar:
//      `(`, [ expr, { `,`, expr }* ], `)`
#[derive(Debug, Clone)]
pub struct Tuple {
    members: Vec<ExprRef>,
}

impl Expr {
    pub fn span(&self) -> Span {
        self.span
    }

    pub fn data(&self) -> &ExprData {
        &self.data
    }

    pub fn new(span: Span, data: ExprData) -> ExprRef {
        Box::new(Expr { span, data })
    }

    pub fn new_tuple(span: Span, members: Vec<ExprRef>) -> ExprRef {
        Self::new(span, ExprData::Tuple(members))
    }

    pub fn new_binary(
        span: Span,
        left: ExprRef,
        right: ExprRef,
        operator: BinaryOperator,
    ) -> ExprRef {
        Self::new(
            span,
            ExprData::BinaryOperation(BinaryOperation {
                left,
                right,
                operator,
            }),
        )
    }

    pub fn new_unary(span: Span, operand: ExprRef, operator: UnaryOperator) -> ExprRef {
        Self::new(
            span,
            ExprData::UnaryOperation(UnaryOperation { operand, operator }),
        )
    }

    pub fn new_call(span: Span, callee: ExprRef, arguments: Vec<CallArgument>) -> ExprRef {
        Self::new(span, ExprData::Call(Call { callee, arguments }))
    }

    pub fn new_index(span: Span, reciever: ExprRef, index: ExprRef) -> ExprRef {
        Self::new(span, ExprData::Index { reciever, index })
    }

    pub fn new_member_access(span: Span, reciever: ExprRef, member: Name) -> ExprRef {
        Self::new(span, ExprData::MemberAccess { reciever, member })
    }

    pub fn is_identifier(&self) -> bool {
        match self.data() {
            ExprData::Identifier(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::LessThan => write!(f, "<"),
            _ => todo!(),
        }
    }
}
