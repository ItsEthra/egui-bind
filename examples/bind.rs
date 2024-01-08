use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui::{Context, Modifiers, Window};
use egui_bind::{Bind, BindTarget, KeyOrPointer};

type Binding = Option<(KeyOrPointer, Modifiers)>;

#[derive(Default)]
struct ExampleApp {
    bind: Binding,
    count: usize,
    check: bool,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Example").show(ctx, |ui| {
            // Order matters, If you were to put this if case
            // after the bind was shown, then it would trigger `self.count += 1`
            // on the same frame user assigned a new bind, which may not be the
            // desired behavior. But you can mitigate this by using the return
            // value of a `Bind::show` as shown below with `println!`.
            if self.bind.pressed(ctx) {
                self.count += 1;
                self.check = !self.check;
            }

            let r = ui.checkbox(&mut self.check, "Check");
            if egui_bind::show_bind_popup(ui, &mut self.bind, "check_popup", &r) {
                println!("Rebinded from popup");
            }

            // `Bind::new` accepts a reference to a type that implements `BindTarget`
            // Most common of those are:
            // `Key`, `PointerButton`, `KeyOrPointer`, `(BindTarget, Modifiers)`
            // Or `Option<T>` where `T` is the type mentioned above.
            if ui.add(Bind::new("_test", &mut self.bind)).changed() {
                println!("Rebinded!");
            }

            ui.label(format!("Counter: {}", self.count));
        });
    }
}

#[allow(clippy::box_default)]
fn create(CreationContext { egui_ctx: ctx, .. }: &CreationContext) -> Box<dyn App> {
    ctx.set_pixels_per_point(1.5);
    Box::new(ExampleApp::default())
}

fn main() -> Result<(), eframe::Error> {
    run_native("Example", NativeOptions::default(), Box::new(create))
}
