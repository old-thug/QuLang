use qu_common::extract;
use qu_ir::{IrModule, function::IrFunction, global::GlobalValue, instruction::{self, InstructionKind}, irtype::IrType, value::{self, Value, ValueRef}};

use crate::type_writer::write_type;



#[derive(Debug, Clone)]
pub struct Cgenerator<'a> {
    headers: String,
    declarations: String,
    definitions: String,
    module: &'a IrModule,
}

impl<'a> Cgenerator<'a> {
    pub fn generate(module: &'a IrModule) -> String {
        let mut generator = Self {
            headers: String::new(),
            definitions: String::new(),
            declarations: String::new(),
            module,
        };
        generator.write_hd("#include <stdint.h>\n".to_string());
        generator.write_decl("typedef struct {} _unit;\n".to_string());
        generator.write_decl("typedef void* _rawptr;\n".to_string());

        for global in module.get_globals() {
            match global {
                GlobalValue::Function(function) => generator.declare_function(function),
                _ => todo!(),
            }
        }

        for global in module.get_globals() {
            match global {
                GlobalValue::Function(function) => generator.gen_function(function),
                _ => todo!(),
            }
        }

        let mut out = String::new();
        out += generator.headers.as_str();
        out += "/* =================================================================== */\n";
        out += generator.declarations.as_str();
        out += "/* =================================================================== */\n";
        out += generator.definitions.as_str();
        out
    }

    fn write_decl(&mut self, s: String) {
        self.declarations += &s;
    }

    fn write_def(&mut self, s: String) {
        self.definitions += &s;
    }

    fn write_hd(&mut self, s: String) {
        self.headers += &s;
    }

    fn declare_function(&mut self, function: &IrFunction) {
        let type_ = &function.type_;
        extract!(type_, IrType::Function { return_type, parameter_type });
        self.write_decl(format!("\n{} {}(", write_type(&return_type), function.get_name()));
        for (index, param) in parameter_type.iter().enumerate() {
            if index != 0 {
                self.write_decl(format!(", "));
            }
            self.write_decl(format!("{} param_{index}", write_type(param)));
        }
        self.write_decl(format!(");\n"));
    }

    fn gen_function(&mut self, function: &IrFunction) {
        let type_ = &function.type_;
        extract!(type_, IrType::Function { return_type, parameter_type });
        self.write_def(format!("\n{} {}(", write_type(&return_type), function.get_name()));
        for (index, param) in parameter_type.iter().enumerate() {
            if index != 0 {
                self.write_def(format!(", "));
            }
            self.write_def(format!("{} param_{index}", write_type(param)));
        }
        self.write_def(format!(") {{\n"));
        for instruction in &function.instructions {
            //println!("Here: {:?}", instruction);
            self.gen_instruction(instruction);
        }
        self.write_def(format!("}}\n"));
    }

    fn gen_instruction(&mut self, instruction: &instruction::Instruction) {
        let target = instruction.0;
        match &instruction.1 {
            InstructionKind::Alloca { type_ } => {
                if let Some(target) = target {
                    self.write_def(format!("    auto {} = ", self.write_value(target)));
                }
                self.write_def(format!("({}){{0}};\n", write_type(type_)));
            },
            InstructionKind::Store(v) => {
                let target = target.unwrap();
                self.write_def(format!("    {} = {};\n", self.write_value(target), self.write_value(*v)));
            }
            InstructionKind::Return(v) => {
                self.write_def(format!("    return {};\n", self.write_value(*v)));
            },
            InstructionKind::Call { callee, args } => {
                let target = target.unwrap();
                self.write_def(format!("    {} = {}(", self.write_value(target), self.write_value(*callee)));
                for (idx, arg) in args.iter().enumerate() {
                    if idx != 0 {
                        self.write_def(", ".to_string());
                    }
                    self.write_def(format!("{}", self.write_value(*arg)));
                }
                self.write_def(format!(");\n"));
            },
            InstructionKind::Binop { op, lhs, rhs } => {
                self.write_def(
                    format!(
                        "    {} = {} {} {};\n",
                        self.write_value(target.unwrap()),
                        self.write_value(*lhs),
                        op,
                        self.write_value(*rhs),
                    )
                );
            },
            _ => todo!("{:?}", instruction.1),
        }
    }

    fn write_value(&self, value: ValueRef) -> String {
        let value = &self.module.value_pool()[value.0];
        match value {
            Value::ConstantInt(v) => format!("{v}"),
            Value::ConstantString(v) => format!("{v}"),
            Value::Ref(id) => format!("local_{}", id.0),
            Value::RefParam(id) => format!("param_{id}"),
            Value::RefGlobal(id) => {
                let global = &self.module.get_globals()[id.0];
                match global {
                    GlobalValue::Constant(name) => format!("{name}"),
                    GlobalValue::Function(function) => format!("{}", function.get_name()),
                }
            },
            Value::True => format!("true"),
            Value::False => format!("false"),
            Value::Unit => format!("(_unit){{}}"),
            _ => todo!(),
        }
    }
}
