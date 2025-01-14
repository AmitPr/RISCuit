mod tables {
    include!(concat!(env!("OUT_DIR"), "/tables.rs"));
}

pub use tables::{RISCV_C2G_C0, RISCV_C2G_C1, RISCV_C2G_C2};
