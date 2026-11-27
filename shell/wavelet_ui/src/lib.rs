use std::collections::HashMap;
use std::path::PathBuf;

pub const SLINT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/ui");

pub fn compiler_config() -> slint_build::CompilerConfiguration {
    slint_build::CompilerConfiguration::new()
        .with_library_paths(HashMap::from([("wavelet".to_string(), PathBuf::from(SLINT_DIR))]))
}

pub fn compile(path: &str) {
    slint_build::compile_with_config(path, compiler_config()).expect("failed to compile the Slint UI");
}
