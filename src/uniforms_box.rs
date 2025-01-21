use crate::shader_frame::UniformsValues;
use crate::shader_parser::{PreparseResult, UniformSmell, UniformSpec};
use egui::{SliderClamping, Ui};
use std::ops::RangeInclusive;

pub fn uniforms_box(uv: &mut UniformsValues, ppr: &PreparseResult, ui: &mut Ui) {
    for u in &ppr.uniforms {
        let labels = match u.smell {
            UniformSmell::Color => ["r", "g", "b", "a"],
            UniformSmell::Unperfumed => ["x", "y", "z", "w"],
        };
        ui.group(|ui| {
            let name = &u.name;
            ui.label(name.clone());
            match &u.spec {
                UniformSpec::Int(spec) => {
                    let v = match uv.int_values.get(name) {
                        Some(v) => *v,
                        None => spec.default.unwrap_or(0),
                    };
                    single_component_slider(
                        ui,
                        "",
                        0,
                        &u.range,
                        &[v as f32],
                        &mut |_index, val| uv.set_int_value(name, val as i64),
                    );
                }
                UniformSpec::Float(spec) => {
                    let v = match uv.float_values.get(name) {
                        Some(v) => *v as f32,
                        None => spec.default.unwrap_or(0.0),
                    };
                    single_component_slider(ui, "", 0, &u.range, &[v], &mut |_index, val| {
                        uv.set_float_value(name, val)
                    });
                }
                UniformSpec::Vec2(spec) => {
                    let xy = match uv.vec2_values.get(name) {
                        Some(v) => *v,
                        None => spec
                            .default
                            .unwrap_or_else(|| (0.0f32, 0.0f32).into())
                            .into(),
                    };
                    let setter = &mut |index, val| uv.set_vec3_component(name, index, val);
                    for index in 0..2 {
                        single_component_slider(ui, labels[index], index, &u.range, &xy, setter);
                    }
                }
                UniformSpec::Vec3(spec) => {
                    let xyz = match uv.vec3_values.get(name) {
                        Some(v) => *v,
                        None => spec
                            .default
                            .unwrap_or_else(|| (0.0f32, 0.0f32, 0.0f32).into())
                            .into(),
                    };
                    let setter = &mut |index, val| uv.set_vec3_component(name, index, val);
                    for index in 0..3 {
                        single_component_slider(ui, labels[index], index, &u.range, &xyz, setter);
                    }
                }
                UniformSpec::Vec4(spec) => {
                    let xyzw = match uv.vec4_values.get(name) {
                        Some(v) => *v,
                        None => spec
                            .default
                            .unwrap_or_else(|| (0.0f32, 0.0f32, 0.0f32, 0.0f32).into())
                            .into(),
                    };
                    let setter = &mut |index, val| uv.set_vec4_component(name, index, val);
                    for index in 0..4 {
                        single_component_slider(ui, labels[index], index, &u.range, &xyzw, setter);
                    }
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
            .clamping(SliderClamping::Never)
            .text(label),
        );
    });
}
