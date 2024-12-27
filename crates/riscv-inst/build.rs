use std::path::Path;

fn main() {
    let tables = riscv_c_tables::generate_tables();
    let out = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out).join("tables.rs");

    println!("cargo:warning={}", out.display());

    let mut writer = std::fs::File::create(&out).unwrap();
    match riscv_c_tables::write_tables(&mut writer, &tables) {
        Ok(_) => println!("Generated tables.rs"),
        Err(e) => panic!("Error generating tables.rs: {}", e),
    }
}
