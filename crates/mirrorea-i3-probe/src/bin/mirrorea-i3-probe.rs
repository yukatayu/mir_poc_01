//! Private I3-0 child-process entry point.
//!
//! The sole accepted argument selects `server` or `client`; all candidate,
//! endpoint, carrier, and credential data is supplied on the inherited stdin
//! pipe by the common supervisor. This binary is not a public CLI.

#![allow(unused_crate_dependencies)]

use std::process::ExitCode;

fn main() -> ExitCode {
    if mirrorea_i3_probe::run_private_child_process(std::env::args().skip(1)) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
