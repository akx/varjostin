use crate::shader_parser::{PreparseResult, UniformSmell, UniformSpec};
use crate::uniforms_values::UniformsValues;
use egui::{Color32, Rgba, SliderClamping, Ui};
use std::ops::RangeInclusive;

pub fn uniforms_box(uv: &mut UniformsValues, ppr: &PreparseResult, ui: &mut Ui) {
    if ui.button("clear").clicked() {
        uv.clear();
    }
    for u in &ppr.uniforms {
        if matches!(u.spec, UniformSpec::Sampler2D) {
            continue;
        }
        let labels = match u.smell {
            UniformSmell::Color => ["r", "g", "b", "a"],
            UniformSmell::Unperfumed => ["x", "y", "z", "w"],
        };
        ui.group(|ui| {
            let name = &u.name;
            ui.horizontal(|ui| {
                ui.label(name.clone());
                if ui
                    .button("reset")
                    .on_hover_text("Reset to default")
                    .clicked()
                {
                    match &u.spec {
                        UniformSpec::Int(i) => uv.set_int_value(name, i.certain_default().into()),
                        UniformSpec::Float(f) => {
                            uv.set_float_value(name, f.certain_default().into())
                        }
                        UniformSpec::Vec2(v) => uv.set_vec2_value(name, v.certain_default()),
                        UniformSpec::Vec3(v) => uv.set_vec3_value(name, v.certain_default()),
                        UniformSpec::Vec4(v) => uv.set_vec4_value(name, v.certain_default()),
                        UniformSpec::Sampler2D => {}
                    }
                }
            });
            match &u.spec {
                UniformSpec::Int(spec) => {
                    let v = match uv.int_values.get(name) {
                        Some(v) => *v,
                        None => spec.default.unwrap_or(0),
                    };
                    if let Some(new_value) = single_component_slider(ui, v as f32, "", &u.range) {
                        uv.set_int_value(name, new_value as i64);
                    }
                }
                UniformSpec::Float(spec) => {
                    let v = match uv.float_values.get(name) {
                        Some(v) => *v,
                        None => spec.default.unwrap_or(0.0),
                    };
                    if let Some(new_value) = single_component_slider(ui, v, "", &u.range) {
                        uv.set_float_value(name, new_value as f64);
                    }
                }
                UniformSpec::Vec2(spec) => {
                    let mut xy = match uv.vec2_values.get(name) {
                        Some(v) => *v,
                        None => spec.default.unwrap_or_else(|| (0.0f32, 0.0f32).into()),
                    };
                    let mut changed = false;
                    for index in 0..2 {
                        if let Some(new_value) =
                            single_component_slider(ui, xy[index], labels[index], &u.range)
                        {
                            changed = true;
                            xy[index] = new_value;
                        }
                    }
                    if changed {
                        uv.set_vec2_value(name, xy);
                    }
                }
                UniformSpec::Vec3(spec) => {
                    let mut xyz = match uv.vec3_values.get(name) {
                        Some(v) => *v,
                        None => spec
                            .default
                            .unwrap_or_else(|| (0.0f32, 0.0f32, 0.0f32).into()),
                    };
                    let mut changed = false;
                    for index in 0..3 {
                        if let Some(new_value) =
                            single_component_slider(ui, xyz[index], labels[index], &u.range)
                        {
                            changed = true;
                            xyz[index] = new_value;
                        }
                    }
                    if changed {
                        uv.set_vec3_value(name, xyz);
                    }
                    if u.smell == UniformSmell::Color {
                        let rgba = Rgba::from_rgb(xyz[0], xyz[1], xyz[2]);
                        let mut edit_color = Color32::from(rgba);
                        if ui.color_edit_button_srgba(&mut edit_color).changed() {
                            let (r, g, b, _) = Rgba::from(edit_color).to_tuple();
                            uv.set_vec3_value(name, [r, g, b]);
                        }
                    }
                }
                UniformSpec::Vec4(spec) => {
                    let mut xyzw = match uv.vec4_values.get(name) {
                        Some(v) => *v,
                        None => spec
                            .default
                            .unwrap_or_else(|| (0.0f32, 0.0f32, 0.0f32, 0.0f32).into()),
                    };
                    let mut changed = false;
                    for index in 0..4 {
                        if let Some(new_value) =
                            single_component_slider(ui, xyzw[index], labels[index], &u.range)
                        {
                            changed = true;
                            xyzw[index] = new_value;
                        }
                    }
                    if changed {
                        uv.set_vec4_value(name, xyzw);
                    }
                    if u.smell == UniformSmell::Color {
                        let rgba =
                            Rgba::from_rgba_premultiplied(xyzw[0], xyzw[1], xyzw[2], xyzw[3]);
                        let mut edit_color = Color32::from(rgba);
                        if ui.color_edit_button_srgba(&mut edit_color).changed() {
                            let (r, g, b, a) = Rgba::from(edit_color).to_tuple();
                            uv.set_vec4_value(name, [r, g, b, a]);
                        }
                    }
                }
                UniformSpec::Sampler2D => {
                    unreachable!();
                }
            }
        });
    }
}

fn single_component_slider(
    ui: &mut Ui,
    current: f32,
    label: &str,
    range: &RangeInclusive<f32>,
) -> Option<f32> {
    let mut current = current;
    let slidey_boy = egui::Slider::new(&mut current, range.clone())
        .clamping(SliderClamping::Never)
        .text(label);
    if ui.add(slidey_boy).changed() {
        return Some(current);
    }
    None
}
