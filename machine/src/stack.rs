use super::error::StackError;

pub struct Stack {
    mem: Vec<u16>,
    pointer: usize,
}

impl Stack {
    pub fn new(n: usize) -> Self {
        Self {
            mem: vec![0; n],
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), StackError> {
        let res = self
            .mem
            .get_mut(self.pointer)
            .map(|v| *v = value)
            .ok_or(StackError::StackOverflow);
        self.pointer += 1;
        res
    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.pointer <= 0 {
            return Err(StackError::StackUnderflow);
        }
        self.pointer -= 1;
        return Ok(self.mem[self.pointer]);
    }
}

#[cfg(test)]
mod test {
    use super::{Stack, StackError};

    #[test]
    fn stack_push() {
        let mut stack = Stack::new(1024);
        assert!(stack.push(10).is_ok(), "This needs to return true")
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::new(1024);

        stack.push(10).expect("This needs to return ok.");
        let value = stack.pop().expect("This needs to return ok.");

        assert!(
            value == 10,
            "Return value from pop is not equal to push value."
        );
    }

    #[test]
    fn stack_overflow() {
        let mut stack = Stack::new(2);
        stack.push(10).expect("This needs to return ok.");
        let _ = stack
            .push(10)
            .map_err(|e| assert!(e == StackError::StackOverflow, "Error type is wrong."));
    }

    #[test]
    fn stack_underflow() {
        let mut stack = Stack::new(0);
        let _ = stack
            .pop()
            .map_err(|e| assert!(e == StackError::StackUnderflow, "Error type is wrong."));
    }
}
