use super::memory::Memory;
use super::stack::Stack;
pub struct Machine {
    memory: Memory,
    stack: Stack,
}

impl Machine {
    pub fn new(n: usize) -> Self {
        Self {
            memory: Memory::new(n),
            stack: Stack::new(n),
        }
    }
}
