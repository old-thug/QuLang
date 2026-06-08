#[allow(unused)]
use clap::Parser;
use qu_context::Context;
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
        let mut parser = qu_parser::Parser::new(source, source_id);
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

    let mut sym_analyzer = SymbolAnalyzer::new();

    let (mut symbols, scopes) = match sym_analyzer.run(&ast) {
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
        let mut type_checker = TypeChecker::new(&scopes, &mut symbols);
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
