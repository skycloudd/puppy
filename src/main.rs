use crate::diagnostics::Diagnostic;
use camino::{Utf8Path, Utf8PathBuf};
use codespan_reporting::{
    files::{Files as _, SimpleFiles},
    term,
};
use core::error::Error;
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
    inputs: Vec<Utf8PathBuf>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let mut files = SimpleFiles::new();

    for filename in &args.inputs {
        let file_id = files.add(filename.as_ref(), read_to_string(filename).unwrap());

        let diagnostics = compile::compile(files.source(file_id).unwrap(), file_id);

        write_diagnostics(&diagnostics, &files).unwrap();
    }
}

fn write_diagnostics(
    diagnostics: &[Diagnostic],
    files: &SimpleFiles<&Utf8Path, String>,
) -> Result<(), Box<dyn Error>> {
    let writer = term::termcolor::StandardStream::stderr(term::termcolor::ColorChoice::Auto);
    let config = term::Config::default();

    for diagnostic in diagnostics {
        term::emit_to_write_style(&mut writer.lock(), &config, files, &diagnostic.0.report())?;
    }

    Ok(())
}
