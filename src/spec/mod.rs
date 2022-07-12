use self::{
    etable::EventTableEntry, imtable::InitMemoryTableEntry, itable::InstructionTableEntry,
    jtable::JumpTableEntry, mtable::MemoryTableEntry,
};

pub mod etable;
pub mod imtable;
pub mod itable;
pub mod jtable;
pub mod mtable;

#[derive(Default)]
pub struct CompileTable {
    pub itable: Vec<InstructionTableEntry>,
    pub imtable: Vec<InitMemoryTableEntry>,
}

#[derive(Default)]
pub struct ExecutionTable {
    pub etable: Vec<EventTableEntry>,
    pub mtable: Vec<MemoryTableEntry>,
    pub jtable: Vec<JumpTableEntry>,
}