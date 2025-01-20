use crate::shader_frame::UniformsValues;
use crate::shader_parser::{PreparseResult, UniformSpec};
use egui::Ui;

pub fn uniforms_box(uv: &mut UniformsValues, ppr: &PreparseResult, ui: &mut Ui) {
    for unif in &ppr.uniforms {
        ui.group(|ui| {
            ui.label(unif.name.clone());
            match &unif.spec {
                UniformSpec::Int(spec) => {
                    let v = spec.default.unwrap_or(0);
                    let mut val = v;
                    let range = v.min(0)..=v.max(200);
                    ui.add(egui::Slider::new(&mut val, range));
                }
                UniformSpec::Float(spec) => {
                    let v = spec.default.unwrap_or(0.0);
                    let mut val = v;
                    let range = v.min(0.0)..=v.max(1.0);
                    ui.add(egui::Slider::new(&mut val, range));
                }
                UniformSpec::Vec3(spec) => {
                    let xyz = match uv.vec3_values.get(&unif.name) {
                        Some(v) => *v,
                        None => spec
                            .default
                            .unwrap_or_else(|| (0.0f32, 0.0f32, 0.0f32).into())
                            .into(),
                    };
                    let range = 0.0..=1.0;
                    ui.add(
                        egui::Slider::from_get_set(range.clone(), &mut |set_val: Option<f64>| {
                            if let Some(val) = set_val {
                                uv.set_vec3_component(&unif.name, 0, val);
                                return val;
                            }
                            xyz[0] as f64
                        })
                        .text("x"),
                    );
                    ui.add(
                        egui::Slider::from_get_set(range.clone(), &mut |set_val: Option<f64>| {
                            if let Some(val) = set_val {
                                uv.set_vec3_component(&unif.name, 1, val);
                                return val;
                            }
                            xyz[1] as f64
                        })
                        .text("y"),
                    );
                    ui.add(
                        egui::Slider::from_get_set(range, &mut |set_val: Option<f64>| {
                            if let Some(val) = set_val {
                                uv.set_vec3_component(&unif.name, 2, val);
                                return val;
                            }
                            xyz[2] as f64
                        })
                        .text("z"),
                    );
                }
                _ => {
                    ui.label("Unsupported uniform type");
                }
            }
        });
    }
}
