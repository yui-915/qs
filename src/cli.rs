use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub file: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
