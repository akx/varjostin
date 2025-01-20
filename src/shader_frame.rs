#![allow(clippy::undocumented_unsafe_blocks)]

use crate::gl::compile_program;
use eframe::egui_glow;
use eframe::epaint::PaintCallbackInfo;
use egui::mutex::Mutex;
use egui::Ui;
use egui_glow::glow;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ShaderCompileResponse {
    pub duration: Duration,
    pub error: Option<eyre::Error>,
}

pub struct ShaderCompileRequest {
    pub fragment_source: String,
    pub response_sender: Sender<ShaderCompileResponse>,
}

pub struct Custom3d {
    shader_frame: Arc<Mutex<ShaderFrame>>,
    init_time: Instant,
    shader_compile_request: Option<ShaderCompileRequest>,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub frame: u64,
}

struct DrawInfo {
    mouse_x: f32,
    mouse_y: f32,
    mouse_down: bool,
    curr_time: f32,
    frame: u64,
    fps: f32,
}

impl Custom3d {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref()?;
        Some(Self {
            shader_frame: Arc::new(Mutex::new(ShaderFrame::new(gl)?)),
            shader_compile_request: None,
            mouse_x: 0.0,
            mouse_y: 0.0,
            frame: 0,
            init_time: Instant::now(),
        })
    }

    pub(crate) fn reset(&mut self) {
        self.init_time = Instant::now();
        self.frame = 0;
    }

    pub(crate) fn request_shader_compile(
        &mut self,
        fragment_source: String,
        response_sender: Sender<ShaderCompileResponse>,
    ) {
        self.shader_compile_request = Some(ShaderCompileRequest {
            fragment_source,
            response_sender,
        });
    }

    pub fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui, fps: f32) {
        egui::Frame::none().show(ui, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            match response.hover_pos() {
                Some(pos) => {
                    self.mouse_x = pos.x;
                    self.mouse_y = pos.y;
                }
                None => {}
            }
            let draw_info = DrawInfo {
                mouse_x: self.mouse_x,
                mouse_y: self.mouse_y,
                mouse_down: response.clicked(),
                curr_time: self.curr_time(),
                frame: self.frame,
                fps,
            };
            let shader_compile_request = self.shader_compile_request.take();
            self.frame += 1;
            let f = self.shader_frame.clone();

            let cb = egui_glow::CallbackFn::new(move |info, painter| {
                let mut fl = f.lock();
                if let Some(request) = &shader_compile_request {
                    let t0 = Instant::now();
                    let fr = fl.set_shader(painter.gl(), &request.fragment_source);
                    let duration = Instant::now().duration_since(t0);
                    request
                        .response_sender
                        .send(ShaderCompileResponse {
                            duration,
                            error: fr.err(),
                        })
                        .ok();
                }
                fl.paint(painter.gl(), &info, &draw_info);
            });

            let callback = egui::PaintCallback {
                rect,
                callback: Arc::new(cb),
            };
            ui.painter().add(callback);
        });
    }

    pub fn curr_time(&mut self) -> f32 {
        self.init_time.elapsed().as_secs_f32()
    }

    pub fn exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.shader_frame.lock().destroy(gl);
        }
    }
}

struct ShaderFrame {
    program: Option<glow::Program>,
    vertex_array: glow::VertexArray,
}

#[allow(unsafe_code)] // we need unsafe code to use glow
impl ShaderFrame {
    fn new(gl: &glow::Context) -> Option<Self> {
        use glow::HasContext as _;

        unsafe {
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Some(Self {
                program: None,
                vertex_array,
            })
        }
    }

    fn set_shader(&mut self, gl: &glow::Context, fragment_source: &str) -> eyre::Result<()> {
        let program = compile_program(&gl, fragment_source)?;
        self.program = Some(program);
        Ok(())
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            if let Some(program) = self.program {
                gl.delete_program(program);
            }
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, pci: &PaintCallbackInfo, info: &DrawInfo) {
        use glow::HasContext as _;
        if let Some(program) = self.program {
            unsafe {
                gl.use_program(Some(program));
                gl.uniform_3_f32(
                    gl.get_uniform_location(program, "iResolution").as_ref(),
                    pci.screen_size_px[0] as f32,
                    pci.screen_size_px[1] as f32,
                    1.0,
                );
                gl.uniform_1_f32(
                    gl.get_uniform_location(program, "iTime").as_ref(),
                    info.curr_time,
                );
                gl.uniform_1_i32(
                    gl.get_uniform_location(program, "iFrame").as_ref(),
                    info.frame as i32,
                );
                gl.uniform_1_f32(
                    gl.get_uniform_location(program, "iFrameRate").as_ref(),
                    info.fps,
                );
                gl.uniform_4_f32(
                    gl.get_uniform_location(program, "iMouse").as_ref(),
                    info.mouse_x,
                    info.mouse_y,
                    0.0,
                    0.0,
                );

                gl.bind_vertex_array(Some(self.vertex_array));
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }
        }
    }
}
