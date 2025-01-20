#![allow(clippy::undocumented_unsafe_blocks)]

use eframe::egui_glow;
use egui::mutex::Mutex;
use egui::Ui;
use egui_glow::glow;
use std::sync::Arc;

pub struct Custom3d {
    shader_frame: Arc<Mutex<ShaderFrame>>,
    angle: f32,
}

impl Custom3d {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref()?;
        Some(Self {
            shader_frame: Arc::new(Mutex::new(ShaderFrame::new(gl)?)),
            angle: 0.0,
        })
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(egui::Vec2::splat(512.0), egui::Sense::drag());

            self.angle += response.drag_motion().x * 0.01;

            // Clone locals so we can move them into the paint callback:
            let angle = self.angle;
            let f = self.shader_frame.clone();

            let cb = egui_glow::CallbackFn::new(move |_info, painter| {
                f.lock().paint(painter.gl(), angle);
            });

            let callback = egui::PaintCallback {
                rect,
                callback: Arc::new(cb),
            };
            ui.painter().add(callback);
        });
    }

    pub fn exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.shader_frame.lock().destroy(gl);
        }
    }
}

struct ShaderFrame {
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

#[allow(unsafe_code)] // we need unsafe code to use glow
impl ShaderFrame {
    fn new(gl: &glow::Context) -> Option<Self> {
        use glow::HasContext as _;

        let shader_version = egui_glow::ShaderVersion::get(gl);

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            if !shader_version.is_new_shader_interface() {
                panic!(
                    "Custom 3D painting hasn't been ported to {:?}",
                    shader_version
                );
            }

            let shader_sources = [
                (glow::VERTEX_SHADER, include_str!("vertex.glsl")),
                (glow::FRAGMENT_SHADER, include_str!("fragment.glsl")),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(
                        shader,
                        &format!(
                            "{}\n{}",
                            shader_version.version_declaration(),
                            shader_source
                        ),
                    );
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile custom_3d_glow {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );

                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Some(Self {
                program,
                vertex_array,
            })
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_angle").as_ref(),
                angle,
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
