use crate::frame_history::FrameHistory;
use crate::shader_frame::Custom3d;
use eframe::glow;
use egui::{FontData, FontDefinitions, FontFamily};

pub struct VarjostinApp {
    custom3d: Custom3d,
    continuous: bool,
    frame_history: FrameHistory,
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
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        cc.egui_ctx.set_fonts(get_fonts());
        Self {
            custom3d: Custom3d::new(cc).unwrap(),
            continuous: true,
            frame_history: FrameHistory::default(),
        }
    }
}

impl eframe::App for VarjostinApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        if esc_pressed {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::SidePanel::right("plop")
            .min_width(200f32)
            .show(ctx, |ui| {
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
