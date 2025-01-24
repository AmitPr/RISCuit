use std::path::Path;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

#[proc_macro]
/// Create one test for each input file in the artifacts directory.
pub fn generate_tests(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let test_artifacts_dir =
        Path::new(env!("CARGO_WORKSPACE_DIR")).join("./riscv/test-env/artifacts");

    let mut artifacts = test_artifacts_dir.read_dir().unwrap();

    let mut tests = Vec::new();
    while let Some(Ok(entry)) = artifacts.next() {
        let path = entry.path();
        if !path.is_file() || path.extension() == Some("dump".as_ref()) {
            continue;
        }
        if !path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("rv32")
        {
            continue;
        }

        tests.push(generate_test_for_artifact(&path));
    }
    let output = quote! {
        #[cfg(test)]
        mod tests {
            use derisc::{cpu::Hart32, elf::load_elf, riscv_inst::Reg};

            #(#tests)*
        }
    };
    output.into()
}

fn generate_test_for_artifact(artifact: &Path) -> TokenStream {
    let test_name = artifact
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("-", "_");
    let test_name = Ident::new(&test_name, Span::call_site());

    let program_bytes = std::fs::read(artifact).expect("Failed to read test artifact");

    quote! {
        #[test]
        fn #test_name() {
            let program = &[#(#program_bytes),*];
            let mut cpu = Hart32::new();
            let elf = load_elf(&mut cpu, program);
            cpu.pc = elf.entry as u32;

            let sp = 0xc0000000u32 - 0x1000;
            cpu.set_reg(Reg::Sp, sp);

            let res = cpu.run(elf);
            assert!(res.is_ok(), "Test failed: {}", res.unwrap_err());
            let exit = res.unwrap();
            assert_eq!(exit, 0, "Test failed with exit code {}", exit);
        }
    }
}
