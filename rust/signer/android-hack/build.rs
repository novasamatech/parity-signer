use std::{env, fs::create_dir_all, path::Path};

use definitions::error::{Active, ErrorSource};
use generate_message::{full_run, parser::Command};

fn main() -> Result<(), String> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let cold_release_dir =
        Path::new(&manifest_dir).join("../../android/app/src/main/assets/Database/");
    create_dir_all(&cold_release_dir).unwrap();
    let command = Command::MakeColdRelease(Some(cold_release_dir));

    full_run(command).map_err(|e| <Active>::show(&e))?;

    Ok(())
}
