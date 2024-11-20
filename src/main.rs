mod cli;
mod parser;
mod runtime;

use parser::Value;
use runtime::Printable;
use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

fn make_runtime() -> runtime::Runtime {
    let mut runtime = runtime::Runtime::new();

    runtime.register_fn("print", |v: Vec<Value>| {
        print!("{}", v.first().unwrap().fmt_print());
        Value::Nil
    });

    runtime
}

fn main() {
    let cli = cli::parse();
    let mut runtime = make_runtime();

    if let Some(file) = &cli.file {
        let src = read_to_string(file).unwrap();
        let prog = parser::parse(&src);
        #[cfg(debug_assertions)]
        output_ast(&prog);
        runtime.run(prog);
    } else {
        loop {
            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let prog = parser::parse(&input);
            let v = runtime.run(prog);
            println!("{}", v.fmt_print());
        }
    }
}

#[cfg(debug_assertions)]
fn output_ast(ast: &parser::Program) {
    let json = serde_json::to_string_pretty(&ast).unwrap();
    let mut cmd = std::process::Command::new("jq")
        .args(["--indent", "3"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    cmd.stdin
        .as_mut()
        .unwrap()
        .write_all(json.as_bytes())
        .unwrap();
    let stdout = cmd.wait_with_output().unwrap().stdout;
    std::fs::write("ast.json", stdout).unwrap();
}
