use qu_ir::irtype::IrType;


pub(crate) fn write_type(irtype: &IrType) -> String {
    match irtype {
        IrType::I8 => format!("int8_t"),
        IrType::I16 => format!("int16_t"),
        IrType::I32 => format!("int32_t"),
        IrType::I64 => format!("int64_t"),
        IrType::U8 => format!("uint8_t"),
        IrType::U16 => format!("uint16_t"),
        IrType::U32 => format!("uint32_t"),
        IrType::U64 => format!("uint64_t"),
        IrType::Char => format!("char"),
        IrType::Pointer => format!("_rawptr"),
        IrType::TypedPointer(inner) => format!("{}*", write_type(inner)),
        IrType::Unit => format!("_unit"),
        IrType::Function { return_type, parameter_type } => {
            let mut r = format!("{}(*)(", write_type(return_type));
            for (index, p) in parameter_type.iter().enumerate() {
                if index != 0 {
                    r += ", ";
                }
                r += &format!("{}", write_type(p));
            }
            r += ")";
            r
        },
    }
}
