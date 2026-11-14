#[derive(Clone)]
pub enum Operand {
    Constant(i32),
    Label(String),
    Text(String),
}

#[derive(Clone)]
pub enum Instruction {
    In,
    Out,
    Load(Operand),
    Store(Operand),
    Add(Operand),
    Subtract(Operand),
    Multiply(Operand),
    Divide(Operand),
    Line,
    Print(Operand),
    Jump(Operand),
    Jizero(Operand),
    Jineg(Operand),
    Halt,
}

pub struct Line {
    label: Option<String>,
    instruction: Instruction,
}

impl Line {
    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }
    pub fn instruction(&self) -> Instruction {
        self.instruction.clone()
    }
}
