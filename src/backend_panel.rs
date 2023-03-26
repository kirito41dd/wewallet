use tracing::info;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct BackendPanel {}

impl crate::Ui for BackendPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("electrum server:");
            ui.colored_label(egui::Color32::DARK_GREEN, "● 25ms")
        });
        ui.separator();
        if ui.button("test").clicked() {
            btc_net_cli::get_tx_then(
                "caa13042224074e91dc71193039bce3ef7340983cb0e0cd607326d0d73243064".into(),
                |resp| info!("tx! {:#?}", resp),
            )
        }
    }
}
