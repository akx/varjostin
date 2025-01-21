#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clap::Parser;
use std::path::PathBuf;
use varjostin::VarjostinApp;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, env = "VARJOSTIN_SHADER")]
    shader: Option<PathBuf>,
    #[arg(short, long, env = "VARJOSTIN_VSYNC", default_value_t = true)]
    vsync: bool,
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let cli = Cli::parse();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([300.0, 220.0]),
        vsync: cli.vsync,
        ..Default::default()
    };
    eframe::run_native(
        "Varjostin",
        native_options,
        Box::new(|cc| Ok(Box::new(VarjostinApp::new(cc, cli.shader)))),
    )
}
