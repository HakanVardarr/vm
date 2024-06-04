use super::error::MemoryError;

pub struct Memory {
    mem: Vec<u16>,
}

impl Memory {
    pub fn new(n: usize) -> Self {
        Self { mem: vec![0; n] }
    }

    pub fn read(&mut self, p: usize) -> Result<u16, MemoryError> {
        self.mem.get(p).map(|v| *v).ok_or(MemoryError::ReadError)
    }

    pub fn write(&mut self, p: usize, value: u16) -> Result<(), MemoryError> {
        self.mem
            .get_mut(p)
            .map(|v| *v = value)
            .ok_or(MemoryError::WriteError)
    }
}

#[cfg(test)]
mod test {
    use super::{Memory, MemoryError};

    #[test]
    fn memory_read() {
        let mut memory = Memory::new(1024 * 1024);
        assert!(
            memory.read(0).expect("This operation needs to return ok.") == 0,
            "Return value needs to be 0."
        );
    }

    #[test]
    fn memory_write() {
        let mut memory = Memory::new(1024 * 1024);
        memory.write(0x1000, 10).expect("This needs to return ok.");
        assert!(
            memory.read(0x1000).expect("This needs to return ok.") == 10,
            "The value returned from the function is incorrect."
        )
    }

    #[test]
    fn memory_out_of_bounds() {
        let mut memory = Memory::new(1024 * 1024);
        let _ = memory
            .write(1024 * 1024, 10)
            .map_err(|e| assert!(e == MemoryError::WriteError));
        let _ = memory
            .read(1024 * 1024)
            .map_err(|e| e == MemoryError::ReadError);
    }
}
