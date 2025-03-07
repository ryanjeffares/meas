use std::{path::PathBuf, process::Command};

use crate::io::file::{read_entire_file, write_file_lines};
use anyhow::Result;
use clap::Parser;
use codegen::{
    codegen::emit_asm,
    register::{Context, Register},
};
use compiler::compiler::Compiler;

mod ast;
mod codegen;
mod compiler;
mod io;
mod scanner;

#[derive(Parser)]
struct Args {
    file: String,

    #[arg(short, long, default_value_t = false)]
    run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = &args.file;
    let code = read_entire_file(file)?;
    println!("Code:\n");
    println!("{code}");

    let mut compiler = Compiler::new(code);

    println!("\n---\n");
    println!("Tokens:\n");
    let tree = compiler.compile()?;

    println!("\n---\n");
    println!("AST:\n");

    tree.print_tree(0);

    println!("\n---\n");
    println!("ASM:\n");

    let mut asm = Vec::<String>::new();
    emit_asm(tree, &mut asm, false, Context::User, Some(Register::Rax))?;
    for line in &asm {
        println!("{line}");
    }

    let asm_path = PathBuf::from(file).with_extension("asm");
    write_file_lines(&asm_path, &asm)?;

    println!("\n---\n");
    println!("Wrote ASM to {asm_path:?}");

    let obj_path = PathBuf::from(file).with_extension("o");
    let exe_path = PathBuf::from(file).with_extension("");

    if args.run {
        Command::new("as")
            .arg("-msyntax=intel")
            .arg("-mnaked-reg")
            .arg(&asm_path)
            .arg("-o")
            .arg(&obj_path)
            .spawn()?;
        println!("Wrote obj to {obj_path:?}");
        Command::new("ld")
            .arg("-s")
            .arg("-o")
            .arg(&exe_path)
            .arg(&obj_path)
            .spawn()?;
        println!("Wrote executable to {exe_path:?}");
        let exit_code = Command::new(&exe_path).output()?.status.code().unwrap();
        println!("Ran with exit code {exit_code}");
    }

    Ok(())
}
