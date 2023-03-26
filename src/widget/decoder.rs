use bitcoin::address;
use egui::Widget;
use poll_promise::Promise;

use crate::Ui;

#[derive(Default)]
pub struct TxDecoder {
    txid: String,
    tx: Option<Promise<anyhow::Result<bitcoin::Transaction, anyhow::Error>>>,
}

impl crate::View for TxDecoder {
    fn name(&self) -> &'static str {
        "交易查找"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(900.0, 800.0))
            .resizable(true)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

impl crate::Ui for TxDecoder {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("交易id:");
            egui::TextEdit::singleline(&mut self.txid)
                .clip_text(false)
                .min_size(egui::Vec2 { x: 400.0, y: 1.0 })
                .ui(ui);

            if ui.button("查找").clicked() {
                tracing::trace!("search txid: {}", &self.txid);
                let ctx = ui.ctx().clone();
                let (sender, promise) = Promise::new();
                btc_net_cli::get_tx_then(self.txid.clone(), move |resp| {
                    sender.send(resp);
                    ctx.request_repaint();
                });
                _ = self.tx.insert(promise);
            }
        });
        if let Some(tx) = &self.tx {
            match tx.ready() {
                None => {}
                Some(Err(e)) => {
                    ui.colored_label(ui.visuals().error_fg_color, e.to_string());
                }
                Some(Ok(tx)) => {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("hash:");
                            egui::TextEdit::singleline(&mut tx.wtxid().to_string().as_str())
                                .clip_text(false)
                                .ui(ui);
                        });
                        ui.label(format!("version: {}", tx.version));
                        ui.label(format!("size: {}", tx.size()));
                        ui.label(format!("vsize: {}", tx.vsize()));
                        ui.label(format!("weight: {}", tx.weight()));
                        ui.label(format!("lock_time: {}", tx.lock_time));
                        ui.separator();
                        ui.label("输入:");
                        for ele in tx.input.iter() {
                            ui.horizontal(|ui| {
                                ui.label("txid:");
                                egui::TextEdit::singleline(
                                    &mut ele.previous_output.txid.to_string().as_str(),
                                )
                                .clip_text(false)
                                .ui(ui);
                                ui.label(format!("vout: {}", ele.previous_output.vout));
                            });
                            ui.horizontal(|ui| {
                                ui.label("script sig:");
                                egui::TextEdit::singleline(
                                    &mut ele.script_sig.to_string().as_str(),
                                )
                                .clip_text(false)
                                .ui(ui);
                            });
                        }
                        ui.separator();
                        ui.label("输出:");
                        for ele in tx.output.iter() {
                            ui.horizontal(|ui| {
                                ui.label("value:");
                                egui::TextEdit::singleline(&mut ele.value.to_string().as_str())
                                    .clip_text(false)
                                    .ui(ui);
                            });
                            ui.horizontal(|ui| {
                                ui.label("script:");
                                egui::TextEdit::singleline(
                                    &mut ele.script_pubkey.to_string().as_str(),
                                )
                                .clip_text(false)
                                .ui(ui);
                            });

                            ui.horizontal(|ui| {
                                ui.label("addr:");
                                let addr = address::Address::from_script(
                                    &ele.script_pubkey,
                                    bitcoin::Network::Bitcoin,
                                )
                                .unwrap()
                                .to_string();
                                egui::TextEdit::singleline(&mut addr.as_str())
                                    .clip_text(false)
                                    .ui(ui);
                            });
                        }
                    });
                }
            }
        }
    }
}
