use crate::token::Token;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StateKind {
    GlobalDeclration,
    FunctionParameterType,
    FunctionReturnType,
    FunctionBody,
    VariableDeclration,
    VariableInitialization,
    VariableType,
}

#[derive(Debug, Clone)]
pub struct State {
    kind: StateKind,
    has_error: bool,
    begin: Option<Token>,
}

impl From<StateKind> for State {
    fn from(kind: StateKind) -> Self {
        Self {
            kind,
            has_error: false,
            begin: None,
        }
    }
}

impl State {
    pub fn new(kind: StateKind) -> Self {
        Self {
            kind,
            has_error: false,
            begin: None,
        }
    }

    pub fn kind(&self) -> StateKind {
        self.kind
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    pub fn begin(&self) -> Option<Token> {
        self.begin
    }

    pub fn with_begin(mut self, token: Token) -> Self {
        self.begin = Some(token);
        self
    }

    pub fn with_error(mut self) -> Self {
        self.has_error = true;
        self
    }

    pub fn with_kind(mut self, kind: StateKind) -> Self {
        self.kind = kind;
        self
    }
}
