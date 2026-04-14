//! Optional root binary entrypoint.
//!
//! Some environments expect `src/main.rs` to exist even when dedicated binaries
//! live under `src/bin/`. This file delegates to the primary CLI binary logic.
//!
//! If Cargo.toml explicitly defines only `src/bin/*`, this file can be removed.

use std::process;

fn main() {
    let code = agentmem::cli::run();
    process::exit(code);
}
