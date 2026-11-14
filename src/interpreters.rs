use crate::instructions;
use crate::store;
use crate::store::Store;

pub struct Interpreter {
    lines: Vec<instructions::Line>,
    current_line: usize,
    accumulator: i32,
    store: store::Store,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_line: 0,
            accumulator: 0,
            store: Store::new(),
        }
    }
}

impl Interpreter {
    pub fn execute(&mut self) {
        todo!()
    }

    fn execute_in(&mut self) -> Result<(), String> {
        todo!()
    }
    fn execute_out(&self) -> Result<(), String> {
        todo!()
    }
}