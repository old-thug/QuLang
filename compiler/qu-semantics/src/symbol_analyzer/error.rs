use qu_ast::Name;
use qu_diagnostics::{Diagnostic, Severity};
use qu_span::Span;

use super::SymbolAnalyzer;

pub(super) fn redefinition_of_symbol(
    ctx: &mut SymbolAnalyzer,
    name: Name,
    first_defined: Option<Span>,
) {
    let mut diag = Diagnostic::new(
        Severity::Error,
        format!("name `{}` defined multiple times", name.value),
        name.span,
        format!("duplicate definition of `{}`", name.value),
    );
    if let Some(span) = first_defined {
        diag = diag.with_label("first defined here".to_string(), span);
    }
    ctx.emit_diag(diag);
}
