#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod storage;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rshell",
        native_options,
        Box::new(|cc| Box::new(Rshell::RshellApp::new(cc))),
    )

}

// extern crate term;
// use std::io::prelude::*;

// fn main() {
//     let mut t = term::stdout().unwrap();

//     t.fg(term::color::GREEN).unwrap();
//     write!(t, "hello, ").unwrap();

//     t.fg(term::color::RED).unwrap();
//     writeln!(t, "world!").unwrap();

//     t.reset().unwrap();
// }