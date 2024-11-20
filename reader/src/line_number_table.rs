use std::cmp::Ordering;

use crate::{line_number::LineNumber, program_counter::ProgramCounter};

#[derive(Debug, PartialEq)]
pub struct LineNumberTable {
    entries: Vec<LineNumberTableEntry>,
}

impl LineNumberTable {
    pub fn lookup_pc(&self, pc: ProgramCounter) -> LineNumber {
        let best_matching_entry_index = match self
            .entries
            .binary_search_by(|e| e.program_counter.cmp(&pc))
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };
        self.entries[best_matching_entry_index].line_number
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineNumberTableEntry {
    pub program_counter: ProgramCounter,
    pub line_number: LineNumber,
}

impl PartialOrd for LineNumberTableEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.program_counter.partial_cmp(&other.program_counter)
    }
}

impl Ord for LineNumberTableEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.program_counter.cmp(&other.program_counter)
    }
}