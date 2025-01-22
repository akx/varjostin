#![warn(clippy::all, rust_2018_idioms)]
mod app;
mod file_change;
mod file_collection;
mod frame_history;
mod gl;
mod label_strip;
mod options;
mod shader_frame;
mod shader_parser;
mod uniforms_box;
mod uniforms_values;

pub use app::{Options, VarjostinApp};
