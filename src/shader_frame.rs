#![allow(clippy::undocumented_unsafe_blocks)]

use crate::gl::compile_program;
use crate::shader_parser::{preparse_shader, PreparseResult};
use crate::uniforms_values::UniformsValues;
use eframe::egui_glow;
use eframe::egui_glow::Painter;
use eframe::epaint::{PaintCallbackInfo, TextureHandle};
use egui::mutex::Mutex;
use egui::Ui;
use egui_glow::glow;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ShaderCompileResponse {
    pub duration: Duration,
    pub preparse_result: Option<eyre::Result<PreparseResult>>,
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
    pub mouse_down: bool,
    pub frame: u64,
    last_mouse_down_time: Instant,
}

struct DrawInfo {
    mouse_x: f32,
    mouse_y: f32,
    mouse_down_seconds: f32,
    curr_time: f32,
    frame: u64,
    fps: f32,
    uniforms_values: UniformsValues,
}

impl Custom3d {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref()?;
        Some(Self {
            shader_frame: Arc::new(Mutex::new(ShaderFrame::new(gl)?)),
            shader_compile_request: None,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_down: false,
            frame: 0,
            init_time: Instant::now(),
            last_mouse_down_time: Instant::now(),
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

    pub fn update(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut Ui,
        fps: f32,
        uniforms_values: &UniformsValues,
        texture: &TextureHandle,
    ) {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        if let Some(pos) = response.hover_pos() {
            // TODO: Why do we need the fudge here?!
            self.mouse_x = pos.x - rect.min.x;
            self.mouse_y = pos.y - rect.min.y;
        }
        let mouse_down = response.is_pointer_button_down_on();
        if !self.mouse_down && mouse_down {
            self.last_mouse_down_time = Instant::now();
        }
        self.mouse_down = mouse_down;
        let draw_info = DrawInfo {
            mouse_x: self.mouse_x,
            mouse_y: self.mouse_y,
            mouse_down_seconds: if mouse_down {
                self.last_mouse_down_time.elapsed().as_secs_f32()
            } else {
                0.0
            },
            curr_time: self.curr_time(),
            frame: self.frame,
            fps,
            uniforms_values: uniforms_values.clone(),
        };
        let shader_compile_request = self.shader_compile_request.take();
        self.frame += 1;
        let f = self.shader_frame.clone();
        let texture = texture.clone();

        let cb = egui_glow::CallbackFn::new(move |info, painter| {
            let mut fl = f.lock();
            if let Some(request) = &shader_compile_request {
                let t0 = Instant::now();
                let prep = preparse_shader(&request.fragment_source);
                let sampler_uniform_names = prep
                    .as_ref()
                    .map(|prep| prep.sampler_uniform_names())
                    .unwrap_or_default();
                let fr = fl.set_shader(
                    painter.gl(),
                    &request.fragment_source,
                    sampler_uniform_names,
                );
                let duration = Instant::now().duration_since(t0);
                request
                    .response_sender
                    .send(ShaderCompileResponse {
                        duration,
                        preparse_result: Some(prep),
                        error: fr.err(),
                    })
                    .ok();
            }
            fl.paint(painter, &info, &draw_info, &texture);
        });

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(cb),
        };
        ui.painter().add(callback);
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
    sampler_uniform_names: Vec<String>,
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
                sampler_uniform_names: Vec::new(),
            })
        }
    }

    fn set_shader(
        &mut self,
        gl: &glow::Context,
        fragment_source: &str,
        sampler_uniform_names: Vec<String>,
    ) -> eyre::Result<()> {
        let program = compile_program(gl, fragment_source)?;
        self.program = Some(program);
        self.sampler_uniform_names = sampler_uniform_names;
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

    fn paint(
        &self,
        painter: &Painter,
        pci: &PaintCallbackInfo,
        info: &DrawInfo,
        texture: &TextureHandle,
    ) {
        use glow::HasContext as _;
        let gl = painter.gl();
        if let Some(program) = self.program {
            unsafe {
                // Egui will have configured the viewport already,
                // so we don't do that.

                gl.use_program(Some(program));

                let view = pci.viewport_in_pixels();
                let scale = pci.pixels_per_point;

                let vp = (
                    view.left_px,
                    view.from_bottom_px,
                    view.width_px + view.left_px,
                    view.from_bottom_px + view.height_px,
                );
                let mouse = (
                    info.mouse_x * scale,
                    view.height_px as f32 - (info.mouse_y * scale),
                );

                gl.uniform_3_f32(
                    gl.get_uniform_location(program, "iResolution").as_ref(),
                    view.width_px as f32,
                    view.height_px as f32,
                    1.0,
                );
                gl.uniform_4_f32(
                    gl.get_uniform_location(program, "iViewport").as_ref(),
                    vp.0 as f32,
                    vp.1 as f32,
                    vp.2 as f32,
                    vp.3 as f32,
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
                    mouse.0,
                    mouse.1,
                    info.mouse_down_seconds,
                    0.0,
                );
                for (index, name) in self.sampler_uniform_names.iter().enumerate() {
                    let texture_id = texture.id();
                    match painter.texture(texture_id) {
                        Some(texture) => {
                            gl.active_texture(glow::TEXTURE1 + index as u32);
                            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                            gl.uniform_1_i32(
                                gl.get_uniform_location(program, name).as_ref(),
                                (index + 1) as i32,
                            );
                        }
                        None => {
                            eprintln!("Texture not found: {:?}", texture_id);
                        }
                    }
                }
                info.uniforms_values.apply(painter, gl, program);
                gl.bind_vertex_array(Some(self.vertex_array));
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }
        }
    }
}
