// use std::collections::HashMap;
use std::io;

// #[inline(never)]
// fn test_io() -> io::Result<()> {
//     // Test stdout/stderr
//     println!("Standard output test");
//     eprintln!("Standard error test");

//     // Test formatted output
//     println!("Formatted integers: {:#x}, {:#b}", 255, 255);
//     println!("Formatted floating point: {:.2}", 3.14159);

//     // Test string manipulation
//     let mut s = String::new();
//     s.push_str("Hello ");
//     s.push_str("RISC-V!");
//     println!("{}", s);

//     Ok(())
// }

// #[inline(never)]
// fn test_collections() {
//     // Test HashMap
//     let mut map = HashMap::new();
//     map.insert("one", 1);
//     map.insert("two", 2);
//     map.insert("three", 3);

//     println!("HashMap test:");
//     for (k, v) in map.iter() {
//         println!("  {} = {}", k, v);
//     }

//     // Test Vec
//     let mut vec = Vec::new();
//     vec.extend([1, 2, 3, 4, 5]);
//     println!("Vector: {:?}", vec);

//     // Test sorting
//     vec.sort_unstable();
//     println!("Sorted vector: {:?}", vec);
// }

// #[inline(never)]
// fn test_formatting() {
//     // Test various formatting options
//     let value = 42;
//     println!("Different number formats:");
//     println!("  Decimal:     {}", value);
//     println!("  Hexadecimal: {:#x}", value);
//     println!("  Octal:       {:#o}", value);
//     println!("  Binary:      {:#b}", value);

//     // Test padding and alignment
//     println!("Padding and alignment:");
//     println!("  Right aligned:  {:>10}", value);
//     println!("  Left aligned:   {:<10}", value);
//     println!("  Center aligned: {:^10}", value);
//     println!("  Zero padded:    {:0>5}", value);

//     // Test custom formatting
//     let person = ("Alice", 30);
//     println!("Custom formatting:");
//     println!("  Debug:     {:?}", person);
//     println!("  Pretty:    {:#?}", person);
// }

// fn test_environment() {
//     // Test environment access
//     println!("\nEnvironment information:");
//     if let Ok(path) = std::env::current_dir() {
//         println!("Current directory: {}", path.display());
//     }

//     for (key, value) in std::env::vars() {
//         println!("{}={}", key, value);
//     }

//     // Test command-line arguments
//     println!("\nCommand line arguments:");
//     for arg in std::env::args() {
//         println!("  {}", arg);
//     }
// }

fn main() {
    // println!("=== Starting RISC-V userspace test ===\n");

    // println!("Testing I/O operations:");
    // test_io()?;

    // println!("\nTesting collections:");
    // test_collections();

    // println!("\nTesting formatting:");
    // test_formatting();

    // // println!("\nTesting environment:");
    // // test_environment();

    // println!("\n=== All tests completed ===");
    // Ok(())

    let mut s = String::new();
    s.push_str("Hello ");
    s.push_str("RISC-V!");
    println!("{}", s);

}
