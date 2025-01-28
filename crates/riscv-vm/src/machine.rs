use std::error::Error;

use crate::{error::MachineError, hart::Hart32, memory::Memory};

pub trait Kernel {
    type Error: Error;
    fn syscall(
        &mut self,
        hart: &mut Hart32,
        mem: &mut Memory,
    ) -> Result<StepResult, MachineError<Self::Error>>;

    fn ebreak(
        &mut self,
        _hart: &mut Hart32,
        _mem: &mut Memory,
    ) -> Result<StepResult, MachineError<Self::Error>> {
        Ok(StepResult::Halt)
    }
}

pub enum StepResult {
    Ok,
    Halt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineState {
    Running,
    Halted,
}

impl MachineState {
    pub fn is_running(self) -> bool {
        self == MachineState::Running
    }
}

pub struct Machine<K: Kernel> {
    pub hart: Hart32,
    pub mem: Memory,
    pub kernel: K,
    pub state: MachineState,
}

impl<K: Kernel> Machine<K> {
    pub fn new(kernel: K) -> Self {
        Self {
            hart: Hart32::new(),
            mem: Memory::new(),
            kernel,
            state: MachineState::Running,
        }
    }

    pub fn step(&mut self) -> Result<(), MachineError<K::Error>> {
        match self.hart.step(&mut self.mem, &mut self.kernel)? {
            StepResult::Ok => Ok(()),
            StepResult::Halt => {
                self.state = MachineState::Halted;
                Ok(())
            }
        }
    }

    pub fn run(&mut self) -> Result<(), MachineError<K::Error>> {
        while self.state == MachineState::Running {
            self.step()?;
        }

        Ok(())
    }
}
