use eframe::{App, Frame, run_native, NativeOptions};
use egui::{Context, Window, Key};
use egui_bind::Hotkey;

#[derive(Default)]
struct ExampleApp {
    bind: Option<Key>,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Example")
            .show(ctx, |ui| {
                Hotkey::new("_test", &mut self.bind)
                    .show(ui);
            });
    }
}

fn main() {
    run_native(
        "Example",
        NativeOptions::default(),
        Box::new(|_| Box::new(ExampleApp::default()))
    );
}
