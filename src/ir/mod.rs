pub mod ast;
pub mod token;

#[salsa::input(debug)]
pub struct SourceProgram {
    #[returns(ref)]
    pub text: String,

    pub file_ctx: usize,
}
