//! Native `SilverSetup` desktop entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = silversetup_lib::run() {
        eprintln!("SilverSetup could not start: {error}");
        std::process::exit(1);
    }
}
