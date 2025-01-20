use crate::file_change::{has_changed, FileChangeState};
use crate::frame_history::FrameHistory;
use crate::shader_frame::{Custom3d, ShaderCompileResponse};
use eframe::glow;
use egui::{FontData, FontDefinitions, FontFamily};
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
        }
    }
}

impl eframe::App for VarjostinApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let change_res = has_changed(
            self.shader_path.as_ref().unwrap(),
            &self.shader_change_state,
            Duration::from_millis(200),
        );
        if let Ok(Some(new_state)) = change_res {
            self.shader_change_state = Some(new_state);
            match std::fs::read_to_string(self.shader_path.as_ref().unwrap()) {
                Ok(fragment_source) => {
                    self.custom3d.request_shader_compile(
                        fragment_source,
                        self.shader_compile_result_outbox.clone(),
                    );
                }
                Err(e) => {
                    self.last_shader_compile_result = Some(ShaderCompileResponse {
                        duration: std::time::Duration::default(),
                        error: Some(eyre::eyre!(e)),
                    });
                }
            }
        }
        if let Ok(result) = self.shader_compile_result_inbox.try_recv() {
            self.last_shader_compile_result = Some(result);
        }
        let last_shader_compile_result = self.last_shader_compile_result.as_ref();

        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        if esc_pressed {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::SidePanel::right("settings")
            .resizable(false)
            .max_width(250f32)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.label("Shader");
                    ui.text_edit_singleline(&mut self.edit_shader_path);
                    if ui.button("Set").clicked() {
                        self.shader_path = Some(PathBuf::from(&self.edit_shader_path));
                    }
                    if let Some(result) = last_shader_compile_result {
                        if let Some(e) = &result.error {
                            ui.label("Error:");
                            ui.label(e.to_string());
                        } else {
                            ui.label(format!("Compiled in {:?}", result.duration));
                        }
                    }
                });
                self.frame_history.ui(ui);
                ui.checkbox(&mut self.continuous, "Update continuously");
                ui.group(|ui| {
                    ui.label(format!("Time: {:.2}", self.custom3d.curr_time()));
                    ui.label(format!("Frame: {}", self.custom3d.frame));
                    ui.label(format!(
                        "Mouse: {}x{}",
                        self.custom3d.mouse_x as i32, self.custom3d.mouse_y as i32
                    ));
                    if ui.button("Reset time").clicked() {
                        self.custom3d.reset();
                    }
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.custom3d.update(ctx, ui, self.frame_history.fps());
        });
        if self.continuous {
            ctx.request_repaint();
        }
    }
    fn on_exit(&mut self, glow_ctx: Option<&glow::Context>) {
        self.custom3d.exit(glow_ctx);
    }
}
