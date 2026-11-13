pub enum Operand {
    Constant(i32),
    Label(String),
    Text(String),
}

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