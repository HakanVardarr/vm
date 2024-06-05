use crate::error::MachineError;

use super::memory::Memory;
use super::registers::Registers;
use super::stack::Stack;

const REGISTER_COUNT: usize = 7;

pub struct Machine {
    memory: Memory,
    stack: Stack,
    registers: [u16; REGISTER_COUNT],
}

impl Machine {
    pub fn new(n: usize) -> Self {
        Self {
            memory: Memory::new(n),
            stack: Stack::new(n),
            registers: [0; REGISTER_COUNT],
        }
    }
    pub fn load_program_to_memory(&mut self) {}
    pub fn step(&mut self) -> Result<(), MachineError> {
        let ip = self.registers[Registers::IP as usize];
        let instruction = self
            .memory
            .read(ip as usize)
            .map_err(|e| MachineError::MemoryError(e))?;

        // Instruction : 00000000    |   00000000
        //               Op          |   Arg
        // Or
        // Instruction: 1XXXXXXX     |   000000000 | 16 Bit Arg
        //
        // For example:
        // Instrction = 256 = 00000001 | 00000000

        let op = (instruction & 0xFF00) >> 8;
        let mut i = 1;

        match op {
            // NOP
            // 00000000| XXXXXXXX
            0 => {}
            // MVR
            // 00000001 | XXXXAABB
            // AA = First Register
            // BB = Second Register
            // Set first register to second register and clear the other register
            1 => {
                let registers = instruction & 0xF;
                let aa = registers >> 2;
                let bb = registers << 14;

                self.registers[aa as usize] = self.registers[bb as usize];
                self.registers[bb as usize] = 0;
            }
            // CPY
            // 00000010 | XXXXAABB
            // AA = First Register
            // BB = Second Register
            // Copy second register to first register
            2 => {
                let registers = instruction & 0xF;
                let aa = registers >> 2;
                let bb = registers << 14;

                self.registers[aa as usize] = self.registers[bb as usize];
            }

            // WR
            // 10000001 | XXXXXXNN | 16 Bit ARG
            // NN = Register
            129 => {
                let register = instruction & 0x3;
                self.registers[Registers::IP as usize] += 1;

                let arg = self
                    .memory
                    .read(self.registers[Registers::IP as usize] as usize)
                    .map_err(|e| MachineError::MemoryError(e))?;

                self.registers[register as usize] = arg;
            }
            _ => return Err(MachineError::UnknownOpCode(op)),
        }

        self.registers[Registers::IP as usize] += i;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Machine, Registers};

    #[test]
    fn machine_mvr() {
        let mut machine = Machine::new(1024);

        machine.memory.write(0, 0x8100).unwrap();
        machine.memory.write(1, 10).unwrap();
        machine.step().unwrap();

        machine.memory.write(2, 0x104).unwrap();
        machine.step().unwrap();

        assert!(
            machine.registers[Registers::B as usize] == 10,
            "This needs to be equal."
        );

        assert!(
            machine.registers[Registers::A as usize] == 0,
            "This needs to be equal."
        )
    }

    #[test]
    fn machine_cpy() {
        let mut machine = Machine::new(1024);

        machine.memory.write(0, 0x8100).unwrap();
        machine.memory.write(1, 10).unwrap();
        machine.step().unwrap();

        machine.memory.write(2, 0x204).unwrap();
        machine.step().unwrap();

        assert!(
            machine.registers[Registers::B as usize] == 10,
            "This needs to be equal."
        );

        assert!(
            machine.registers[Registers::A as usize] == 10,
            "This needs to be equal."
        )
    }

    #[test]
    fn machine_wr() {
        let mut machine = Machine::new(1024);

        machine.memory.write(0, 0x8100).unwrap();
        machine.memory.write(1, 10).unwrap();

        let _ = machine.step();

        assert!(
            machine.registers[Registers::A as usize] == 10,
            "This needs to be equal."
        );
    }
}
