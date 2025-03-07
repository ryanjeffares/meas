#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rsp,
    Rbp,
    Rdi,
    Rsi,
    Rdx,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Context {
    Kernel,
    User,
}

impl Register {
    pub fn to_string(self) -> &'static str {
        match self {
            Register::Rax => "rax",
            Register::Rbx => "rbx",
            Register::Rcx => "rcx",
            Register::Rsp => "rsp",
            Register::Rbp => "rbp",
            Register::Rdi => "rdi",
            Register::Rsi => "rsi",
            Register::Rdx => "rdx",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
        }
    }

    pub fn next_available(self, context: Context) -> Option<Register> {
        match context {
            Context::Kernel => match self {
                Register::Rax => Some(Register::Rdi),
                Register::Rdi => Some(Register::Rsi),
                Register::Rsi => Some(Register::R10),
                Register::R10 => Some(Register::R8),
                Register::R8 => Some(Register::R9),
                _ => None,
            },
            Context::User => match self {
                Register::Rax => Some(Register::Rdi),
                Register::Rdi => Some(Register::Rsi),
                Register::Rsi => Some(Register::Rdx),
                Register::Rdx => Some(Register::Rcx),
                Register::Rcx => Some(Register::R8),
                Register::R8 => Some(Register::R9),
                _ => None,
            },
        }
    }
}
