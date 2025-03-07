use anyhow::Result;

use crate::ast::ast_node::{AstNode, BinaryOp};

use super::register::{Context, Register};

pub fn emit_asm(
    tree: AstNode,
    asm: &mut Vec<String>,
    in_main_function: bool,
    context: Context,
    next_register: Option<Register>,
) -> Result<()> {
    match tree {
        AstNode::BinaryOp { op, lhs, rhs } => {
            let dest_register = next_register.unwrap();

            if let Err(err) = emit_asm(*lhs, asm, in_main_function, context, next_register) {
                return Err(err);
            }

            let next_register = dest_register.next_available(context);

            if let Err(err) = emit_asm(*rhs, asm, in_main_function, context, next_register) {
                return Err(err);
            }

            match op {
                BinaryOp::Minus => asm.push(format!(
                    "sub {}, {}",
                    dest_register.to_string(),
                    next_register.unwrap().to_string()
                )),
                BinaryOp::Plus => asm.push(format!(
                    "add {}, {}",
                    dest_register.to_string(),
                    next_register.unwrap().to_string()
                )),
            }

            Ok(())
        }
        AstNode::Call { callee } => {
            asm.push(format!("call {callee}"));
            Ok(())
        }
        AstNode::Function { name, statements } => {
            let in_main_function = name == "main";

            if in_main_function {
                asm.insert(0, format!(".global _start"));
                asm.push("_start:".to_owned());
            } else {
                asm.push(format!("{name}:"));
            }

            for stmt in statements {
                if let Err(err) =
                    emit_asm(stmt, asm, in_main_function, context, Some(Register::Rdi))
                {
                    return Err(err);
                }
            }

            Ok(())
        }
        AstNode::IntegerLiteral { value } => {
            asm.push(format!(
                "mov {}, {value}",
                next_register.unwrap().to_string()
            ));
            Ok(())
        }
        AstNode::Program { declarations } => {
            asm.push(".section .text".to_owned());

            for decl in declarations {
                if let Err(err) = emit_asm(
                    decl,
                    asm,
                    in_main_function,
                    Context::User,
                    Some(Register::Rdi),
                ) {
                    return Err(err);
                }
            }

            Ok(())
        }
        AstNode::ReturnStatement { expression } => {
            if let Err(err) = emit_asm(
                *expression,
                asm,
                in_main_function,
                Context::User,
                Some(Register::Rdi),
            ) {
                return Err(err);
            }

            if in_main_function {
                asm.push("mov rax, 60".to_owned());
                asm.push("syscall".to_owned());
            } else {
                asm.push("ret".to_owned());
            }
            Ok(())
        }
    }
}
