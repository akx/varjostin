#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod file_change;
mod frame_history;
mod gl;
mod shader_frame;
mod shader_parser;
mod uniforms_box;

pub use app::VarjostinApp;
