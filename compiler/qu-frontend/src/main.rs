#![allow(unused)]
use std::str::FromStr;

#[allow(unused)]
use clap::{Parser, Subcommand};
use qu_backend::BackendGenerator;
use qu_context::Context;
use qu_parser::token_stream::TokenStream;
use qu_semantics::{symbol_analyzer::SymbolAnalyzer, type_checker::TypeChecker};

#[derive(clap::Parser)]
#[command(
    version = "0.1.0",
    author = "old-thug",
    about = "Frontend for the Qu language"
)]
struct Command {
    input_path: String,
    #[arg(short, value_name = "FILE", default_value("a.out"))]
    output_path: String,

    #[arg(short, default_value("exe"))]
    target: Target,
}

#[derive(Clone, Default)]
enum Target {
    #[default]
    Executable,
    Ccode,
}

impl Target {
    pub fn to_backend_target(self) -> qu_backend::TargetKind {
        match self {
            Target::Executable => qu_backend::TargetKind::Executable,
            Target::Ccode      => qu_backend::TargetKind::Ccode,
        }
    }
}

impl FromStr for Target {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c-code" => Ok(Self::Ccode),
            "exe"    => Ok(Self::Executable),
            _ => Err(format!("`{}` is not a valid target", s))
        }
    }

   type Err = String;
}

fn run() -> Result<(), String> {
    let command = Command::parse();
    let mut context = Context::new();
    let source_id = context
        .source(&command.input_path)
        .map_err(|_| format!("failed to open path `{}`", command.input_path))?;

    let _root_id = context.get_or_put_new_module("root".to_string());
    let ast = {
        let source = context
            .get_source(source_id)
            .ok_or("failed to get source".to_string())?;
        let ts = TokenStream::new(source, source_id)
            .inspect_err(|diags| {
                let count = diags.len();
                for diag in diags {
                    println!("{:?}", diag.clone().into_report(&context));
                }
            }).map_err(|diags| format!("compilation failed with {} errors", diags.len()))?;
        let mut parser = qu_parser::Parser::new(source, ts);
        match parser.parse() {
            Ok(ast) => ast,
            Err(diags) => {
                let count = diags.len();
                for diag in diags {
                    println!("{:?}", diag.into_report(&context));
                }
                return Err(format!("compilation failed with {} errors", count));
            }
        }
    };
    let module = context.get_module(_root_id).unwrap();
    let mut sym_analyzer = SymbolAnalyzer::new(module);
    let _ = match sym_analyzer.run(&ast) {
        Ok(result) => result,
        Err(diags) => {
            let count = diags.len();
            for diag in diags {
                println!("{:?}", diag.into_report(&context));
            }
            return Err(format!("compilation failed with {} errors", count));
        }
    };

    {
        let mut type_checker = TypeChecker::new(module);
        let _types = match type_checker.run(&ast) {
            Ok(result) => result,
            Err(diags) => {
                let count = diags.len();
                for diag in diags {
                    println!("{:?}", diag.into_report(&context));
                }
                return Err(format!("compilation failed with {} errors", count));
            }
        };
    }

    {
        let mut ir_module = qu_ir::IrModule::new("root".to_string());
        let mut ir_lowerer = qu_ir::lower::IrLowerer::new(&mut ir_module, module);
        match ir_lowerer.lower_ast(ast) {
            Ok(()) => {
            },
            Err(()) => todo!(),
        }

        match BackendGenerator::generate_module(&ir_module, command.target.to_backend_target(), command.output_path) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        }
    }

    Ok(())
}

fn main() {
    match run() {
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}
