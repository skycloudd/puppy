use crate::diagnostics::Diagnostic;
use camino::{Utf8Path, Utf8PathBuf};
use codespan_reporting::{
    files::SimpleFiles,
    term::{
        Config,
        termcolor::{ColorChoice, StandardStream},
    },
};
use lasso::ThreadedRodeo;
use std::{fs::read_to_string, sync::LazyLock};

mod compile;
mod diagnostics;
mod ir;
mod lexer;
mod parser;

pub static RODEO: LazyLock<ThreadedRodeo> = LazyLock::new(ThreadedRodeo::new);

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: Utf8PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let mut files = SimpleFiles::new();

    let source = read_to_string(&args.input).unwrap();
    let file_id = files.add(args.input.as_ref(), source.as_ref());

    let diagnostics = compile::compile(&source, file_id);

    write_diagnostics(&diagnostics, &files).unwrap();
}

fn write_diagnostics(
    diagnostics: &[Diagnostic],
    files: &SimpleFiles<&Utf8Path, &str>,
) -> Result<(), Box<dyn core::error::Error>> {
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = Config::default();

    for diagnostic in diagnostics {
        codespan_reporting::term::emit_to_write_style(
            &mut writer.lock(),
            &config,
            files,
            &diagnostic.0.report(),
        )?;
    }

    Ok(())
}
