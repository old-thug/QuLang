use qu_span::Span;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum TokenKind {
    Keyword(Keyword),
    Operator(Operator),
    Separator(Separator),
    Literal(Literal),
    Identifier,
    EndOfFile,
    #[default]
    Error,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
    Add,
    Minus,
    Star,
    Slash,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,

    Equal,
    NotEqual,
    Assign,
    Not,
    Bar,

    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    ShiftLeft,
    ShiftRight,
    ShiftLeftAssign,
    ShiftRightAssign,

    Pipe, // |>
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Separator {
    SemiColon,
    Colon,
    Comma,
    Dot,
    Arrow,
    OpenBrace,
    OpenBracket,
    OpenParen,
    CloseBrace,
    CloseBracket,
    CloseParen,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Keyword {
    Fn,
    Pub,
    Shared,
    Module,
    Use,
    Extern,
    Type,
    Return,
    If,
    Else,
    For,
    Break,
    Continue,
    Delete,
    Move,
    Let,
    Mut,
    Const,
    Cast,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Literal {
    String,
    Integer,
    Float,
    True,
    False,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[macro_export]
macro_rules! tok {
    (kw $kind: path) => {
        $crate::token::TokenKind::Keyword($kind)
    };
    (op $kind: path) => {
        $crate::token::TokenKind::Operator($kind)
    };
    (sp $kind: path) => {
        $crate::token::TokenKind::Separator($kind)
    };
    (lt $kind: path) => {
        $crate::token::TokenKind::Literal($kind)
    };
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::EndOfFile
    }

    pub fn is_error(&self) -> bool {
        self.kind == TokenKind::Error
    }

    pub fn is_literal(&self) -> bool {
        matches!(self.kind, TokenKind::Literal(_))
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, TokenKind::Keyword(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.kind, TokenKind::Operator(_))
    }

    pub fn is_separator(&self) -> bool {
        matches!(self.kind, TokenKind::Separator(_))
    }

    pub fn is_identifier(&self) -> bool {
        self.kind == TokenKind::Identifier
    }

    pub fn is_integer(&self) -> bool {
        matches!(self.kind, TokenKind::Literal(Literal::Integer))
    }

    pub fn is_float(&self) -> bool {
        matches!(self.kind, TokenKind::Literal(Literal::Float))
    }

    pub fn is_string(&self) -> bool {
        matches!(self.kind, TokenKind::Literal(Literal::String))
    }

    pub fn is_true(&self) -> bool {
        matches!(self.kind, TokenKind::Literal(Literal::True))
    }

    pub fn is_false(&self) -> bool {
        matches!(self.kind, TokenKind::Literal(Literal::False))
    }

    pub fn is_bool(&self) -> bool {
        self.is_true() || self.is_false()
    }

    pub fn is_sp(&self, sp: Separator) -> bool {
        self.kind == TokenKind::Separator(sp)
    }

    pub fn is_lt(&self, lt: Literal) -> bool {
        self.kind == TokenKind::Literal(lt)
    }

    pub fn is_op(&self, op: Operator) -> bool {
        self.kind == TokenKind::Operator(op)
    }

    pub fn is_kw(&self, kw: Keyword) -> bool {
        self.kind == TokenKind::Keyword(kw)
    }

    pub fn is_done(&self) -> bool {
        self.kind == TokenKind::EndOfFile
    }
}
