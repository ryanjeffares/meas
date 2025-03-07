#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOp {
    Minus,
    Plus,
}

pub enum AstNode {
    BinaryOp {
        op: BinaryOp,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Call {
        callee: String,
    },
    Function {
        name: String,
        statements: Vec<AstNode>,
    },
    IntegerLiteral {
        value: i32,
    },
    Program {
        declarations: Vec<AstNode>,
    },
    ReturnStatement {
        expression: Box<AstNode>,
    },
}

impl AstNode {
    pub fn print_tree(&self, level: usize) {
        match self {
            AstNode::BinaryOp { op, lhs, rhs } => {
                println!("{}Binary {op:?}:", " ".repeat(4 * level));
                println!("{}LHS:", " ".repeat(4 * (level + 1)));
                lhs.print_tree(level + 2);
                println!("{}RHS:", " ".repeat(4 * (level + 1)));
                rhs.print_tree(level + 2);
            }
            AstNode::Call { callee } => {
                println!("{}Call: {}", " ".repeat(4 * level), callee)
            }
            AstNode::Function { name, statements } => {
                println!("{}Function {}:", " ".repeat(4 * level), name);
                for stmt in statements {
                    stmt.print_tree(level + 1);
                }
            }
            AstNode::IntegerLiteral { value } => {
                println!("{}IntegerLiteral: {}", " ".repeat(4 * level), value)
            }
            AstNode::Program { declarations } => {
                println!("{}Program:", " ".repeat(4 * level));
                for decl in declarations {
                    decl.print_tree(level + 1);
                }
            }
            AstNode::ReturnStatement { expression } => {
                println!("{}ReturnStatement:", " ".repeat(4 * level));
                expression.print_tree(level + 1);
            }
        }
    }
}
