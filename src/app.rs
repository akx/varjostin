use eframe::glow::Context;
use egui::{Margin, Vec2};
use crate::shader_frame::Custom3d;

pub struct VarjostinApp {
    custom3d: Custom3d,
}

impl VarjostinApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        Self {
            custom3d: Custom3d::new(cc).unwrap(),
        }
    }
}

impl eframe::App for VarjostinApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        if esc_pressed {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::SidePanel::right("plop").show(ctx, |ui| {
            ui.heading("uwu");
        });
        egui::CentralPanel::default(). show(ctx, |ui| {
            self.custom3d.update(ctx, ui);
        });

    }
    fn on_exit(&mut self, ctx: Option<&Context>) {
        self.custom3d.exit(ctx);
    }
}
