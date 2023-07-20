
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Term {
    
}

impl Default for Term {
    fn default() -> Self {
        Self {
            
        }
    }
}


impl super::RShellUIComponent for Term {

    fn name(&self) -> &'static str {
        "term"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
        let total_rows = 10_000;
        egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
            for row in row_range {
                let text = format!("Row {}/{}", row + 1, total_rows);
                ui.label(text);
            }
        });
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {

        let window=egui::Window::new(self.name())
            .vscroll(false)
            .resizable(false)
            .collapsible(false)
            .open(open);

        window.show(ctx, |ui| {
            self.ui(ui);
        });

    }
}