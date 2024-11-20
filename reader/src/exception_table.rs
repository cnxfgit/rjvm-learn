use std::ops::Range;

use crate::program_counter::ProgramCounter;

#[derive(Debug, Default, PartialEq)]
pub struct ExceptionTable {
    entries: Vec<ExceptionTableEntry>,
}

impl ExceptionTable {
    pub fn new(entries: Vec<ExceptionTableEntry>) -> Self {
        Self { entries }
    }
}

impl ExceptionTable {
    pub fn lookup(&self, pc: ProgramCounter) -> Vec<&ExceptionTableEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.range.contains(&pc))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExceptionTableEntry {
    pub range: Range<ProgramCounter>,
    pub handler_pc: ProgramCounter,
    pub catch_class: Option<String>,
}
