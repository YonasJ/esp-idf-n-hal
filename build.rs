use esp_idf_n_hal_build_support::generate_bindings::generate_bindings_from_build_rs;
use std::env;

fn main() {
    let source_path = format!("{}/src/bindings.h", env::var("CARGO_MANIFEST_DIR").unwrap_or(String::from(".")));
    let target_path = format!("{}/src/esp_idf_v4_2_dev.rs",
                              env::var("CARGO_MANIFEST_DIR").unwrap_or(String::from(".")));
    let idf_project = String::from(include!("build_idf_config.rs"));
    generate_bindings_from_build_rs(&idf_project,
                                    &source_path,
                                    &target_path,
                                    &String::from("crate::esp_idf::std::os::raw"));

}
