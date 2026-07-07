use std::error::Error;

use crate::{
    error::MachineError,
    hart::{Execute, Hart, Xlen},
    memory::Memory,
};

/// The OS personality of a machine: gets control on ecall/ebreak.
///
/// `Xlen` fixes the register width and (through the `Addr` equality bound)
/// makes it impossible to pair a hart with a memory of the wrong width.
pub trait Kernel {
    type Xlen: Execute;
    type Memory: Memory<Addr = <Self::Xlen as Xlen>::U>;
    type Error: Error;

    fn syscall(
        &mut self,
        hart: &mut Hart<Self::Xlen>,
        mem: &mut Self::Memory,
    ) -> Result<StepResult, MachineError<Self::Error>>;

    fn ebreak(
        &mut self,
        _hart: &mut Hart<Self::Xlen>,
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

pub struct Machine<K: Kernel> {
    pub hart: Hart<K::Xlen>,
    pub mem: K::Memory,
    pub kernel: K,
    pub state: MachineState,
}

impl<K: Kernel> Machine<K>
where
    K::Memory: Default,
{
    pub fn new(kernel: K) -> Self {
        Self::with_memory(kernel, K::Memory::default())
    }
}

impl<K: Kernel> Machine<K> {
    pub fn with_memory(kernel: K, mem: K::Memory) -> Self {
        Self {
            hart: Hart::new(),
            mem,
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
        if self.state == MachineState::Running {
            self.hart.run(&mut self.mem, &mut self.kernel)?;
            self.state = MachineState::Halted;
        }

        Ok(())
    }
}
