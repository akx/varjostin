use crate::file_change::{has_changed, FileChangeState};
use crate::frame_history::FrameHistory;
use crate::shader_frame::{Custom3d, ShaderCompileResponse, UniformsValues};
use crate::uniforms_box;
use eframe::glow;
use egui::{Align, FontData, FontDefinitions, FontFamily, RichText};
use egui_extras::{Size, StripBuilder};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

pub struct VarjostinApp {
    custom3d: Custom3d,
    continuous: bool,
    frame_history: FrameHistory,
    shader_compile_result_inbox: mpsc::Receiver<ShaderCompileResponse>,
    shader_compile_result_outbox: mpsc::Sender<ShaderCompileResponse>,
    last_shader_compile_result: Option<ShaderCompileResponse>,
    shader_path: Option<PathBuf>,
    edit_shader_path: String,
    shader_change_state: Option<FileChangeState>,
    uniforms_values: UniformsValues,
}

fn get_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "Inter".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!("./Inter-Regular.otf"))),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "Inter".to_owned());
    fonts
}

impl VarjostinApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, shader_path: Option<PathBuf>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        cc.egui_ctx.set_fonts(get_fonts());
        let (scr_sender, scr_receiver) = mpsc::channel();
        let mut custom3d = Custom3d::new(cc).unwrap();
        custom3d.request_shader_compile(
            include_str!("test_fragment.glsl").to_owned(),
            scr_sender.clone(),
        );
        let edit_shader_path = shader_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        Self {
            custom3d,
            continuous: true,
            shader_path,
            frame_history: FrameHistory::default(),
            shader_compile_result_inbox: scr_receiver,
            shader_compile_result_outbox: scr_sender,
            last_shader_compile_result: None,
            edit_shader_path,
            shader_change_state: None,
            uniforms_values: UniformsValues::default(),
        }
    }

    fn check_shader_state(&mut self) {
        let change_res = has_changed(
            self.shader_path.as_ref().unwrap(),
            &self.shader_change_state,
            Duration::from_millis(200),
        );
        match change_res {
            Ok(Some(new_state)) => {
                eprintln!("Shader changed: {:?}", new_state);
                self.shader_change_state = Some(new_state);
                match std::fs::read_to_string(self.shader_path.as_ref().unwrap()) {
                    Ok(fragment_source) => {
                        self.custom3d.request_shader_compile(
                            fragment_source,
                            self.shader_compile_result_outbox.clone(),
                        );
                    }
                    Err(e) => {
                        self.shader_change_state = None;
                        self.last_shader_compile_result = Some(ShaderCompileResponse {
                            duration: Duration::default(),
                            preparse_result: None,
                            error: Some(eyre::eyre!(e)),
                        });
                    }
                }
            }
            Ok(None) => {}
            Err(e) => {
                self.shader_change_state = None;
                self.last_shader_compile_result = Some(ShaderCompileResponse {
                    duration: Duration::default(),
                    preparse_result: None,
                    error: Some(eyre::eyre!(e)),
                });
            }
        }
        if let Ok(result) = self.shader_compile_result_inbox.try_recv() {
            self.last_shader_compile_result = Some(result);
        }
    }
}

impl eframe::App for VarjostinApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.check_shader_state();
        let last_shader_compile_result = self.last_shader_compile_result.as_ref();
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        if esc_pressed {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(Align::Min).with_cross_justify(true),
                |ui| {
                    if ui
                        .text_edit_singleline(&mut self.edit_shader_path)
                        .lost_focus()
                    {
                        self.shader_path = Some(PathBuf::from(&self.edit_shader_path));
                        self.shader_change_state = None;
                    }
                    if ui.button("Reset time").clicked() {
                        self.custom3d.reset();
                    }
                    StripBuilder::new(ui)
                        .sizes(Size::exact(80.0), 3)
                        .horizontal(|mut strip| {
                            for lbl in &[
                                format!("Time: {:.2}", self.custom3d.curr_time()),
                                format!("Frame: {}", self.custom3d.frame),
                                format!(
                                    "Mouse: {}x{}{}",
                                    self.custom3d.mouse_x as i32,
                                    self.custom3d.mouse_y as i32,
                                    if self.custom3d.mouse_down {
                                        " (down)"
                                    } else {
                                        ""
                                    }
                                ),
                            ] {
                                strip.cell(|ui| {
                                    ui.label(lbl);
                                });
                            }
                        });
                },
            );
        });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.continuous, "Update continuously");
                ui.label(format!(
                    "Mean: {:.2} ms/f ({:.1} FPS)",
                    1e3 * self.frame_history.mean_frame_time(),
                    self.frame_history.fps()
                ));
                if let Some(result) = last_shader_compile_result {
                    ui.label(format!(
                        "Compiled{} in {:?}",
                        if result.error.is_some() {
                            " with errors"
                        } else {
                            ""
                        },
                        result.duration
                    ));
                }
            });
        });
        if let Some(result) = last_shader_compile_result {
            if let Some(Ok(ppr)) = &result.preparse_result {
                egui::SidePanel::right("settings")
                    .resizable(false)
                    .max_width(250f32)
                    .show(ctx, |ui| {
                        uniforms_box::uniforms_box(&mut self.uniforms_values, ppr, ui);
                    });
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.custom3d
                .update(ctx, ui, self.frame_history.fps(), &self.uniforms_values);
        });
        let err = last_shader_compile_result.and_then(|r| r.error.as_ref());
        let mut show_error = err.is_some();
        let error_window = egui::Window::new("Compile Error")
            .fixed_pos(&[10., 10.])
            .min_size([300., 200.])
            .resizable(true)
            .title_bar(false)
            .open(&mut show_error)
            .show(ctx, |ui| {
                if let Some(e) = err {
                    ui.label(RichText::new(e.to_string()).color(egui::Color32::RED));
                }
            });
        if self.continuous {
            ctx.request_repaint();
        }
    }
    fn on_exit(&mut self, glow_ctx: Option<&glow::Context>) {
        self.custom3d.exit(glow_ctx);
    }
}
