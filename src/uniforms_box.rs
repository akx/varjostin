use crate::shader_frame::UniformsValues;
use crate::shader_parser::{PreparseResult, UniformSpec};
use egui::Ui;
use std::ops::RangeInclusive;

pub fn uniforms_box(uv: &mut UniformsValues, ppr: &PreparseResult, ui: &mut Ui) {
    for u in &ppr.uniforms {
        ui.group(|ui| {
            let name = &u.name;
            ui.label(name.clone());
            match &u.spec {
                UniformSpec::Int(spec) => {
                    let v = match uv.int_values.get(name) {
                        Some(v) => *v,
                        None => spec.default.unwrap_or(0),
                    };
                    let range = (v as f64).min(0.0)..=(v as f64).max(1.0);
                    single_component_slider(ui, "", 0, &range, &[v as f32], &mut |_index, val| {
                        uv.set_int_value(name, val as i64)
                    });
                }
                UniformSpec::Float(spec) => {
                    let v = match uv.float_values.get(name) {
                        Some(v) => *v as f32,
                        None => spec.default.unwrap_or(0.0),
                    };
                    let range = (v as f64).min(0.0)..=(v as f64).max(1.0);
                    single_component_slider(ui, "", 0, &range, &[v], &mut |_index, val| {
                        uv.set_float_value(name, val)
                    });
                }
                UniformSpec::Vec3(spec) => {
                    let xyz = match uv.vec3_values.get(name) {
                        Some(v) => *v,
                        None => spec
                            .default
                            .unwrap_or_else(|| (0.0f32, 0.0f32, 0.0f32).into())
                            .into(),
                    };
                    let range = 0.0..=1.0;
                    let setter = &mut |index, val| uv.set_vec3_component(name, index, val);
                    single_component_slider(ui, "x", 0, &range, &xyz, setter);
                    single_component_slider(ui, "y", 1, &range, &xyz, setter);
                    single_component_slider(ui, "z", 2, &range, &xyz, setter);
                }
                _ => {
                    ui.label("Unsupported uniform type");
                }
            }
        });
    }
}

fn single_component_slider<const N: usize>(
    ui: &mut Ui,
    label: &str,
    index: usize,
    range: &RangeInclusive<f64>,
    current: &[f32; N],
    setter: &mut impl FnMut(usize, f64),
) {
    if index >= N {
        panic!("Index {index} out of bounds for Vec{}", N);
    }
    ui.horizontal_wrapped(|ui| {
        ui.add(
            egui::Slider::from_get_set(range.clone(), &mut |set_val| {
                if let Some(val) = set_val {
                    setter(index, val);
                    return val;
                }
                current[index] as f64
            })
            .text(label),
        );
    });
}
