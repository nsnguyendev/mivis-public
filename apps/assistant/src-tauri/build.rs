use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Define the source path of the batch script relative to the Cargo.toml of the tauri app
    // apps/assistant/src-tauri/ (location of build.rs and Cargo.toml)
    // to packages/stt/run_stt_service.bat
    // So, ../../../packages/stt/run_stt_service.bat
    let bat_file_source_str = "../../../packages/stt/run_stt_service.bat";
    let bat_file_source = PathBuf::from(bat_file_source_str);

    // Get the output directory (e.g., target/debug/build/assistant-xxxx/out)
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Navigate to the target profile directory (e.g., target/debug/ or target/release/)
    // OUT_DIR is usually something like target/{profile}/build/{crate-name}-{hash}/out
    // We want to go up three levels from /out to get to target/{profile}/
    let target_profile_dir = out_dir
        .parent() // target/{profile}/build/{crate-name}-{hash}/
        .and_then(|p| p.parent()) // target/{profile}/build/
        .and_then(|p| p.parent()) // target/{profile}/
        .expect("Failed to determine target profile directory from OUT_DIR");

    // Define the destination path for the batch script
    let bat_file_dest = target_profile_dir.join("run_stt_service.bat");

    // Copy the file
    if bat_file_source.exists() {
        match fs::copy(&bat_file_source, &bat_file_dest) {
            Ok(_) => {
                println!(
                    "cargo:rerun-if-changed={}",
                    bat_file_source.to_string_lossy()
                );
                println!(
                    "Successfully copied {} to {}",
                    bat_file_source.display(),
                    bat_file_dest.display()
                );
            }
            Err(e) => {
                panic!(
                    "Failed to copy {} to {}: {}",
                    bat_file_source.display(),
                    bat_file_dest.display(),
                    e
                );
            }
        }
    } else {
        panic!("Source file {} does not exist.", bat_file_source.display());
    }

    tauri_build::build()
}
