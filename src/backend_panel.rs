#[derive(serde::Deserialize, serde::Serialize)]
pub struct BackendPanel {
    pub open: bool,
}

impl Default for BackendPanel {
    fn default() -> Self {
        Self { open: false }
    }
}

impl BackendPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label("todo");
        ui.separator();
    }
}
