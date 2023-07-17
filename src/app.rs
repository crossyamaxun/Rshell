use std::collections::BTreeSet;
use crate::{Connect};

use super::RShellUIComponent;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RshellApp {

    //#[cfg_attr(feature = "serde", serde(skip))]
    state: Vec<Box<dyn RShellUIComponent>>,

    open: BTreeSet<String>,
    

    
}

impl Default for RshellApp {
    fn default() -> Self {

        RshellApp::from(vec![
            Box::<super::connect::Connect>::default(),
        ])

    }
}

impl RshellApp {


    pub fn from(state:Vec<Box<dyn RShellUIComponent>>) -> Self {
        let open=BTreeSet::new();
        Self { state, open }
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let mut fonts = eframe::egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert("my_font".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../assets/fonts/AlibabaPuHuiTi-3-35-Thin.ttf"))); // .ttf and .otf supported

        // Put my font first (highest priority):
        fonts.families.get_mut(&eframe::egui::FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&eframe::egui::FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Default::default()
    }
}

impl eframe::App for RshellApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { state,open} = self;


        for com in state {
            let mut is_open = open.contains(com.name());
            com.show(ctx, &mut is_open);
            set_open(open, com.name(), is_open);
        }

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            _frame.set_fullscreen(!_frame.info().window_info.fullscreen);
        }

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                if ui.button("New").clicked() {
                    set_open(open, "connect", true);
                }

                if ui.button("Quit").clicked() {
                    _frame.close();
                }

            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("连接");

            ui.link("腾讯服务器-1");

            

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     ui.horizontal(|ui| {
            //         ui.spacing_mut().item_spacing.x = 0.0;
            //         ui.label("powered by ");
            //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            //         ui.label(" and ");
            //         ui.hyperlink_to(
            //             "eframe",
            //             "https://github.com/emilk/egui/tree/master/crates/eframe",
            //         );
            //         ui.label(".");
            //     });
            // });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Rshell 正在开发当中...");
            egui::warn_if_debug_build(ui);
        });
        
        
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {

    eprintln!("is_open is {is_open}");

    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}