use esp_idf_n_hal_build_support::generate_bindings::generate_bindings_from_build_rs;
use std::env;

fn main() {
    let source_path = format!(
        "{}/main/bindings.h",
        env::var("CARGO_MANIFEST_DIR").unwrap_or(String::from("."))
    );
    let target_path = format!(
        "{}/src/esp_idf_v4_2_dev.rs",
        env::var("CARGO_MANIFEST_DIR").unwrap_or(String::from("."))
    );
    generate_bindings_from_build_rs(&source_path, &target_path, "crate::esp_idf::std::os::raw");
}
