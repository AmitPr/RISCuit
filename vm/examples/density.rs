//! Hart density probe: how many resident rv64 machines fit, and what does
//! aggregate round-robin throughput look like? Exercises the elastic
//! Memory64 arena's premise that host VA tracks actual guest usage.
use riscv_kernel_linux::MockLinux64;
use riscv_vm::machine::Machine;

fn rss_mib() -> f64 {
    let s = std::fs::read_to_string("/proc/self/statm").unwrap();
    let pages: f64 = s.split_whitespace().nth(1).unwrap().parse().unwrap();
    pages * 4096.0 / (1 << 20) as f64
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: density <rv64-elf> [n] [steps]");
    let n: usize = args.next().map(|s| s.parse().unwrap()).unwrap_or(1000);
    let steps: u64 = args.next().map(|s| s.parse().unwrap()).unwrap_or(10_000);
    let elf = std::fs::read(&path).expect("Failed to read ELF file");

    let t0 = std::time::Instant::now();
    let mut machines: Vec<_> = (0..n)
        .map(|_| {
            let mut m = Machine::new(MockLinux64::new(false));
            m.kernel
                .load_static_elf(&mut m.hart, &mut m.mem, &elf, &[], &[]);
            m
        })
        .collect();
    println!(
        "created+loaded {n} rv64 machines in {:.1?} ({:.1}us each), rss={:.0} MiB",
        t0.elapsed(),
        t0.elapsed().as_secs_f64() * 1e6 / n as f64,
        rss_mib()
    );

    // Round-robin: `steps` instructions per machine per pass, until all halt
    // or we complete one sweep (whichever first) -- enough to measure
    // aggregate interpreter throughput across many resident harts.
    let t1 = std::time::Instant::now();
    let mut executed = 0u64;
    for m in &mut machines {
        for _ in 0..steps {
            if m.step().is_err() || !m.state.is_running() {
                break;
            }
        }
        executed += m.hart.inst_count;
    }
    let dt = t1.elapsed().as_secs_f64();
    println!(
        "swept {n} machines x {steps} steps: {executed} insts in {:.2}s ({:.1} MIPS aggregate), rss={:.0} MiB",
        dt,
        executed as f64 / dt / 1e6,
        rss_mib()
    );
}
