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