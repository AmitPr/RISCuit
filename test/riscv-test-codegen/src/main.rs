use std::path::Path;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

const WORKSPACE_ROOT: &str = env!("CARGO_WORKSPACE_DIR");

fn main() {
    let test_artifacts_dir = Path::new(WORKSPACE_ROOT).join("./riscv/test-env/artifacts");
    let mut artifacts = test_artifacts_dir.read_dir().unwrap();

    let mut tests = Vec::new();
    while let Some(Ok(entry)) = artifacts.next() {
        let path = entry.path();
        if !path.is_file() || path.extension() == Some("dump".as_ref()) {
            continue;
        }

        let test_name = path.file_name().unwrap().to_str().unwrap();
        match test_name {
            s if s.starts_with("rv32ua-p") => {}
            s if s.starts_with("rv32uc-p") => {}
            s if s.starts_with("rv32ui-p") => {}
            s if s.starts_with("rv32um-p") => {}
            _ => continue,
        }

        tests.push(generate_test_for_artifact(&path));
    }
    let output = syn::parse_quote! {
        #![cfg(test)]
        use riscv_kernel_linux::MockLinux;
        use riscv_vm::machine::Machine;

        #(#tests)*
    };

    let output_file = Path::new(WORKSPACE_ROOT).join("test/riscv-tests/src/isa_tests.rs");

    std::fs::write(output_file, prettyplease::unparse(&output)).unwrap();
}

fn generate_test_for_artifact(artifact: &Path) -> TokenStream {
    let test_name = artifact
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("-", "_");
    let test_name = Ident::new(&test_name, Span::call_site());

    let program_file = artifact.to_str().unwrap();

    quote! {
        #[test]
        fn #test_name() {
            let program = include_bytes!(#program_file);

            let mut machine = Machine::new(MockLinux::default());
            machine.kernel.load_static_elf(&mut machine.hart, &mut machine.mem, program, &[], &[]);
            let res = machine.run();
            assert!(res.is_ok(), "Test failed: {}", res.unwrap_err());
            assert_eq!(
                machine.kernel.exit_code(),
                Some(0),
                "Test failed with exit code {:?}",
                machine.kernel.exit_code()
            );
        }
    }
}
