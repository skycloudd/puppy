use crate::{ir::SourceProgram, lexer::lexer, parser::parser};

#[salsa::tracked]
pub fn compile(db: &dyn crate::Db, source_program: SourceProgram) {
    let tokens = lexer(db, source_program);

    let program = tokens.and_then(|tokens| parser(db, tokens));

    if let Some(program) = program {
        dbg!(program);
    }
}
