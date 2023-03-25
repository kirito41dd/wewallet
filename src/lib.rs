#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::WalletApp;
mod backend_panel;
mod widget;

pub trait View {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
