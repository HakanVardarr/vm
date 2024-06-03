pub struct Memory {
    mem: Vec<u8>,
}

impl Memory {
    pub fn new(n: usize) -> Self {
        Self { mem: vec![0; n] }
    }

    pub fn read(&mut self, p: usize) -> Result<u8, String> {
        self.mem
            .get(p)
            .map(|v| *v)
            .ok_or(String::from("Unable to read at that location."))
    }

    pub fn write(&mut self, p: usize, value: u8) -> Result<(), String> {
        self.mem
            .get_mut(p)
            .map(|v| *v = value)
            .ok_or(String::from("Unable to write."))
    }
}

#[cfg(test)]
mod test {
    use super::Memory;

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
        assert!(
            memory.write(1024 * 1024, 10).is_err(),
            "The index that given to the function is out of bounds this should return Err."
        );
        assert!(
            memory.read(1024 * 1024).is_err(),
            "The index that given to the function is out of bounds this should return Err."
        );
    }
}
