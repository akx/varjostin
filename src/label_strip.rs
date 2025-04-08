use egui::Ui;
use egui_extras::{Size, StripBuilder};

pub(crate) fn label_strip(ui: &mut Ui, texts: Vec<String>) {
    StripBuilder::new(ui)
        .sizes(Size::exact(80.0), texts.len())
        .horizontal(|mut strip| {
            for lbl in texts {
                strip.cell(|ui| {
                    ui.label(lbl);
                });
            }
        });
}
