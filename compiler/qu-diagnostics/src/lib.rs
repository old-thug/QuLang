use std::{fmt::Display as FD};

use crate::span::Span;
pub mod span;

use miette::{
    Diagnostic as MietteDiagnostic, LabeledSpan, NamedSource, Report, SourceSpan,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

impl FD /* fmt::Display */ for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Note => write!(f, "note"),
        }
    }
}

// Convert your internal severity to miette's severity enum
impl From<Severity> for miette::Severity {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Error => miette::Severity::Error,
            Severity::Warning => miette::Severity::Warning,
            Severity::Note => miette::Severity::Advice,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    label: String,
    span: Span,
    is_primary: bool,
}

// Implement the Error trait so miette can print the top-level message
#[derive(Debug, Error, Clone)]
#[error("{message}")]
pub struct Diagnostic {
    severity: Severity,
    message: String,
    help: Option<String>,
    code: Option<String>,
    labels: Vec<Label>,
}

// Implement miette::Diagnostic to expose metadata like codes, severity, and labels
impl MietteDiagnostic for Diagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.code
            .as_ref()
            .map(|c| Box::new(c.clone()) as Box<dyn std::fmt::Display>)
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(self.severity.into())
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let spans = self.labels.iter().map(|l| {
            // Converts your custom Span into miette's SourceSpan
            let offset = l.span.start as usize;
            let length = (l.span.end - l.span.start) as usize;
            let source_span = SourceSpan::new(offset.into(), length.into());

            if l.is_primary {
                LabeledSpan::new_primary_with_span(Some(l.label.clone()), source_span)
            } else {
                LabeledSpan::new_with_span(Some(l.label.clone()), source_span)
            }
        });

        Some(Box::new(spans))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn FD + 'a>> {
        Some(Box::new(self.help.clone()?))
    }
}

impl Diagnostic {
    pub fn new(severity: Severity, message: String, span: Span, label: String) -> Self {
        Self {
            severity,
            message,
            code: None,
            help: None,
            labels: vec![Label {
                label,
                span,
                is_primary: true,
            }],
        }
    }

    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_label(mut self, label: String, span: Span) -> Self {
        self.labels.push(Label { label , span, is_primary: false});
        self
    }

    pub fn into_report(self, ctx: &qu_context::Context) -> Report {
        let source_code = ctx.get_source_file(self.labels[0].span.source_id).unwrap();
        let report = Report::new(self);
        report
            .with_source_code(NamedSource::new(
                source_code.path.clone(),
                source_code.content.clone(),
            ))
    }
}
