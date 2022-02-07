use crate::cairo::lang::vm::relocatable::RelocatableValue;

use num_bigint::BigInt;
use std::collections::HashMap;

/// Dictionary used for VM memory. Adds the following checks:
/// * Checks that all memory addresses are valid.
/// * getitem: Checks that the memory address is initialized.
/// * setitem: Checks that memory value is not changed.
#[derive(Debug)]
pub struct MemoryDict {
    pub data: HashMap<RelocatableValue, RelocatableValue>,
    pub frozen: bool,
    /// A dict of segment relocation rules mapping a segment index to a RelocatableValue. See
    /// add_relocation_rule for more details.
    pub relocation_rules: HashMap<BigInt, RelocatableValue>,
}

impl MemoryDict {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            frozen: false,
            relocation_rules: HashMap::new(),
        }
    }
}

impl Default for MemoryDict {
    fn default() -> Self {
        Self::new()
    }
}
