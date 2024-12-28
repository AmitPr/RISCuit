mod rv32;
mod rv64;

trait Instruction {
    fn apply(&self, cpu: &mut Cpu);
}

macro_rules! execute {
    (@arg $cpu:ident, $inst:ident, let $arg:ident, $($ty:ident $args:ident),+) => {
        let $arg = $inst.$arg();
        execute!(@arg $cpu, $inst, $($ty $args),*)
    };
    (@arg $cpu:ident, $inst:ident, reg $reg:ident, $($ty:ident $args:ident),+) => {
        let $reg = $cpu.get_reg(inst.$reg() as u32);
        execute!(@arg $cpu, $inst, $($ty $args),*)
    };
    (@arg $cpu:ident, $inst:ident, reg $reg: ident) => {
        let $reg = $cpu.get_reg(inst.$reg() as u32);
    };
    (@arg $cpu:ident, $inst:ident, let $arg: ident) => {
        let $arg = $inst.$arg();
    };
    (@noop $(store)?) => {};
    (
        $inst: ident,
        $( $inst_name: ident: ($($ty:ident $args:ident),+) $(:$store:ident)? => $impl:block),*
    ) => {
        impl Instruction for $inst {
            pub fn apply(&self, cpu: &mut Cpu) {
                match self.op {
                    $(
                        $inst_name => {
                            let inst = self.inst;
                            execute!(@arg cpu, inst, $($ty $args),*)
                            let res = $impl;
                            $(
                                execute!(@noop $store)
                                cpu.set_reg(inst.rd1() as u32, res);
                            )?
                        }
                    )*
                }
            }
        }
    };
}

pub(crate) use execute;
