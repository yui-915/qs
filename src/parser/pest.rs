#[allow(unused_imports)]
pub use pest::iterators::{Pair, Pairs};
use pest::{pratt_parser::PrattParser, Parser};
use pest_derive::Parser;
use std::process::exit;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct PestParser;

pub fn parse(input: &str) -> Pairs<Rule> {
    match PestParser::parse(Rule::program, input) {
        Ok(pairs) => pairs,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::postfix(debug) | Op::postfix(print))
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
            .op(Op::prefix(negate))
    };
}
