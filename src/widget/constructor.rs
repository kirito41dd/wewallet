use crate::Ui;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct TxConstructor {}

impl crate::View for TxConstructor {
    fn name(&self) -> &'static str {
        "交易构造器"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(512.0, 512.0))
            .resizable(true)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

impl crate::Ui for TxConstructor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("text");
    }
}
