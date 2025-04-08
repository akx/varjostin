#![allow(clippy::undocumented_unsafe_blocks)]

use eframe::egui_glow;
use eframe::egui_glow::Painter;
use eframe::glow::NativeProgram;
use egui::ahash::HashMap;
use egui_glow::glow;

#[derive(Clone, Default)]
pub struct UniformsValues {
    pub int_values: HashMap<String, i32>,
    pub float_values: HashMap<String, f32>,
    pub vec2_values: HashMap<String, [f32; 2]>,
    pub vec3_values: HashMap<String, [f32; 3]>,
    pub vec4_values: HashMap<String, [f32; 4]>,
}

impl UniformsValues {
    pub fn set_float_value(&mut self, name: &str, value: f64) {
        self.float_values.insert(name.to_owned(), value as f32);
    }
    pub fn set_int_value(&mut self, name: &str, value: i64) {
        self.int_values.insert(name.to_owned(), value as i32);
    }
    pub fn set_vec3_value(&mut self, name: &str, value: [f32; 3]) {
        self.vec3_values.insert(name.to_owned(), value);
    }
    pub fn set_vec2_value(&mut self, name: &str, value: [f32; 2]) {
        self.vec2_values.insert(name.to_owned(), value);
    }
    pub fn set_vec4_value(&mut self, name: &str, value: [f32; 4]) {
        self.vec4_values.insert(name.to_owned(), value);
    }
    pub fn clear(&mut self) {
        self.int_values.clear();
        self.float_values.clear();
        self.vec2_values.clear();
        self.vec3_values.clear();
        self.vec4_values.clear();
    }
    pub(crate) fn apply(&self, _painter: &Painter, gl: &glow::Context, program: NativeProgram) {
        use glow::HasContext as _;
        #[allow(unsafe_code)]
        unsafe {
            for (name, value) in &self.int_values {
                gl.uniform_1_i32(gl.get_uniform_location(program, name).as_ref(), *value);
            }
            for (name, value) in &self.float_values {
                gl.uniform_1_f32(gl.get_uniform_location(program, name).as_ref(), *value);
            }
            for (name, &[a, b]) in &self.vec2_values {
                gl.uniform_2_f32(gl.get_uniform_location(program, name).as_ref(), a, b);
            }
            for (name, &[a, b, c]) in &self.vec3_values {
                gl.uniform_3_f32(gl.get_uniform_location(program, name).as_ref(), a, b, c);
            }
            for (name, &[a, b, c, d]) in &self.vec4_values {
                gl.uniform_4_f32(gl.get_uniform_location(program, name).as_ref(), a, b, c, d);
            }
        }
    }
}
