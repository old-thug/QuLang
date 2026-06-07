

#[derive(Debug, Clone)]
pub enum GenericParameter {
    Type,
    Integer,
    Bool,
    String,
}

#[derive(Debug, Clone)]
pub struct Generics(pub Vec<GenericParameter>);

impl Generics {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}