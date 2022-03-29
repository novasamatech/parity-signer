// Copyright 2015-2022 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

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
