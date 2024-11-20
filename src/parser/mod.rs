mod ast;
mod pest;

pub use ast::nodes::*;
use ast::ParseMulti;

pub fn parse(input: &str) -> Program {
    let pairs = pest::parse(input);
    // println!("{:#?}", pairs);
    #[allow(clippy::let_and_return)]
    let program = Program::parse(pairs);
    // println!("{:#?}", program);
    program
}
