use crate::io::file::read_entire_file;
use clap::Parser;
use compiler::compiler::Compiler;

mod ast;
mod compiler;
mod io;
mod scanner;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();

    let file = &args.file;

    match read_entire_file(file) {
        Ok(lines) => {
            let mut compiler = Compiler::new(lines);
            match compiler.compile() {
                Ok(tree) => tree.print_tree(0),
                Err(err) => {
                    eprintln!("Error compiling {file}: {err}");
                }
            }
        }
        Err(err) => eprintln!("Error reading {file}: {err}"),
    }
}
