#[derive(Debug, Clone, Hash)]
pub enum IrType {
    I8,
    I16,
    I32,
    I64,
    U8,
    Char,
    U16,
    U32,
    U64,
    Pointer,
    Unit,
    TypedPointer(Box<IrType>),
    Function {
        return_type: Box<IrType>,
        parameter_type: Vec<IrType>,
    },
}
