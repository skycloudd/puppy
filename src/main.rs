use crate::{db::PuppyDatabaseImpl, diagnostics::Diagnostic, ir::SourceProgram, span::Ctx};
use camino::{Utf8Path, Utf8PathBuf};
use codespan_reporting::{
    files::SimpleFiles,
    term::{
        Config,
        termcolor::{ColorChoice, StandardStream},
    },
};
use salsa::Database as Db;
use std::fs::read_to_string;

mod compile;
mod db;
mod diagnostics;
mod ir;
mod lexer;
mod parser;
mod span;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: Utf8PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let db = PuppyDatabaseImpl::default();

    let mut files = SimpleFiles::new();

    let source = read_to_string(&args.input).unwrap();
    let id = files.add(args.input.as_ref(), source.as_ref());

    let source_program = SourceProgram::new(&db, source.clone(), Ctx(id));

    compile::compile(&db, source_program);

    let diagnostics = compile::compile::accumulated::<Diagnostic>(&db, source_program);

    write_diagnostics(&diagnostics, &files).unwrap();
}

fn write_diagnostics(
    diagnostics: &[&Diagnostic],
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
