use std::collections::BTreeSet;

use crate::widget;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct State {
    #[serde(skip)]
    views: Vec<Box<dyn crate::View>>,
    open: std::collections::BTreeSet<String>,
    backend_panel: super::backend_panel::BackendPanel,
    manul_open: bool,
}

pub struct WalletApp {
    state: State,
}

impl WalletApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        setup_custom_fonts(&cc.egui_ctx);

        let mut slf = Self {
            state: State::default(),
        };
        let tx_constructor = Box::new(widget::TxConstructor::default());
        slf.state.views.push(tx_constructor);
        slf.state.manul_open = true;
        Self::set_open(&mut slf.state.open, slf.state.views[0].name(), true);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        slf
    }
}

impl eframe::App for WalletApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // f11 全屏
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            frame.set_fullscreen(!frame.info().window_info.fullscreen);
        }
        // 顶栏
        self.bar_contents(ctx, frame);

        // 侧边控制台
        self.backend_panel(ctx, frame);

        // 右边菜单栏
        self.manual_panel(ctx, frame);

        // 窗口
        self.show_selected_view(ctx, frame);
    }
}

impl WalletApp {
    // 顶栏
    fn bar_contents(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                // 明暗主题切换
                let style: egui::Style = (*ui.ctx().style()).clone();
                let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
                if let Some(visuals) = new_visuals {
                    ui.ctx().set_visuals(visuals);
                }
                ui.separator();

                // 控制台
                ui.toggle_value(&mut self.state.backend_panel.open, "💻 控制台");
                ui.separator();
                // 菜单
                ui.toggle_value(&mut self.state.manul_open, "菜单");
            });
        });
    }

    // 控制台侧栏
    fn backend_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let is_open = self.state.backend_panel.open;

        egui::SidePanel::left("backend_panel")
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("💻 控制台");
                });
                ui.separator();
                self.state.backend_panel.ui(ui, frame);
            });
    }

    // 右侧菜单
    fn manual_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("manual_panel")
            .resizable(false)
            .default_width(150.0)
            .show_animated(ctx, self.state.manul_open, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("菜单");
                });
                ui.separator();

                use egui::special_emojis::GITHUB;
                ui.hyperlink_to(
                    format!("{} GitHub", GITHUB),
                    "https://github.com/kirito41dd/wewallet",
                );
                ui.hyperlink_to(format!("Telegram"), "https://t.me/talk_btc");

                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        let views = &mut self.state.views;
                        let open = &mut self.state.open;
                        for view in views {
                            let mut is_open = open.contains(view.name());
                            ui.toggle_value(&mut is_open, view.name());
                            Self::set_open(open, view.name(), is_open);
                        }
                    });
                })
            });
    }

    // 选中的页面
    fn show_selected_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            let views = &mut self.state.views;
            let open = &mut self.state.open;
            for view in views {
                let mut is_open = open.contains(view.name());
                view.show(ctx, &mut is_open);
                Self::set_open(open, view.name(), is_open);
            }
        });
    }

    fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
        if is_open {
            if !open.contains(key) {
                open.insert(key.to_owned());
            }
        } else {
            open.remove(key);
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/SourceHanSansSC-Regular.otf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
