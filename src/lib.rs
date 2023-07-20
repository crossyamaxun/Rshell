#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod connect;
mod storage;

pub use app::RshellApp;
pub use connect::Connect;
pub use storage::Storage;


// 所有组件实现View
pub trait RShellUIComponent {

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    fn ui(&mut self, ui: &mut egui::Ui);

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}