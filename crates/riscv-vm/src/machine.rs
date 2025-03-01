use std::error::Error;

use crate::{
    error::MachineError,
    hart::Hart32,
    memory::{Memory, Memory32},
};

pub trait Kernel {
    type Error: Error;
    type Memory: Memory;
    fn syscall(
        &mut self,
        hart: &mut Hart32,
        mem: &mut Self::Memory,
    ) -> Result<StepResult, MachineError<Self::Error>>;

    fn ebreak(
        &mut self,
        _hart: &mut Hart32,
        _mem: &mut Self::Memory,
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

pub struct Machine<K: Kernel<Memory: Memory<Addr = u32>>> {
    pub hart: Hart32,
    pub mem: K::Memory,
    pub kernel: K,
    pub state: MachineState,
}

impl<K: Kernel<Memory = Memory32>> Machine<K> {
    pub fn new(kernel: K) -> Self {
        Self {
            hart: Hart32::new(),
            mem: Memory32::new(),
            kernel,
            state: MachineState::Running,
        }
    }
}

impl<K: Kernel<Memory: Memory<Addr = u32>>> Machine<K> {
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
