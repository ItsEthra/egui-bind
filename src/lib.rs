#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use egui::{
    style::Margin, Align2, Event, FontId, Id, Key, PointerButton, Response, Sense, Ui, Widget,
};
use std::hash::Hash;

mod target;
pub use target::*;
mod either;
pub use either::*;

/// Widget for showing the bind itself
pub struct Bind<'a, B: BindTarget> {
    id: Id,
    value: &'a mut B,
}

impl<'a, B: BindTarget> Bind<'a, B> {
    /// Creates a new bind widget
    pub fn new(id_source: impl Hash, value: &'a mut B) -> Self {
        Self {
            id: Id::new(id_source),
            value,
        }
    }
}

impl<B: BindTarget> Widget for Bind<'_, B> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id(self.id);
        let changing = ui.memory_mut(|mem| mem.data.get_temp(id).unwrap_or(false));

        let size = ui.spacing().interact_size;

        let (mut r, p) = ui.allocate_painter(size, Sense::click());
        let vis = ui.style().interact_selectable(&r, changing);

        p.rect_filled(r.rect, vis.rounding, vis.bg_fill);

        p.text(
            r.rect.center(),
            Align2::CENTER_CENTER,
            self.value.format(),
            FontId::default(),
            vis.fg_stroke.color,
        );

        if changing {
            let key = ui.input(|i| {
                i.events
                    .iter()
                    .find(|e| {
                        matches!(
                            e,
                            Event::Key { pressed: true, .. }
                                | Event::PointerButton { pressed: true, .. }
                        )
                    })
                    .cloned()
            });

            let (reset, changed) = match key {
                Some(Event::Key {
                    key: Key::Escape, ..
                }) if B::CLEARABLE => {
                    self.value.clear();
                    (true, true)
                }
                Some(Event::Key { key, modifiers, .. }) if B::IS_KEY => {
                    self.value.set_key(key, modifiers);
                    (true, true)
                }
                Some(Event::PointerButton {
                    button, modifiers, ..
                }) if B::IS_POINTER && button != PointerButton::Primary => {
                    self.value.set_pointer(button, modifiers);
                    (true, true)
                }
                _ if r.clicked_elsewhere() => (true, false),
                _ => (false, false),
            };

            if reset {
                ui.memory_mut(|mem| mem.data.insert_temp(id, false));
            }

            if changed {
                r.mark_changed();
            }
        }

        if r.clicked() {
            ui.memory_mut(|mem| mem.data.insert_temp(id, true));
        }

        r
    }
}

/// Shows bind popup when clicked with secondary pointer button.
pub fn show_bind_popup(
    ui: &mut Ui,
    bind: &mut impl BindTarget,
    popup_id_source: impl Hash,
    widget_response: &Response,
) -> bool {
    let popup_id = Id::new(popup_id_source);

    if widget_response.secondary_clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id))
    }

    let mut should_close = false;
    let was_opened = ui.memory_mut(|mem| mem.is_popup_open(popup_id));

    let mut styles = ui.ctx().style().as_ref().clone();
    let saved_margin = styles.spacing.window_margin;

    styles.spacing.window_margin = Margin::same(0.);
    ui.ctx().set_style(styles.clone());

    let out = egui::popup_below_widget(ui, popup_id, widget_response, |ui| {
        let r = ui.add(Bind::new(popup_id.with("_bind"), bind));

        if r.changed() || ui.input(|i| i.key_down(Key::Escape)) {
            ui.memory_mut(|mem| mem.close_popup());
            should_close = true;
        }

        r.changed()
    });

    styles.spacing.window_margin = saved_margin;
    ui.ctx().set_style(styles);

    if !should_close && was_opened {
        ui.memory_mut(|mem| mem.open_popup(popup_id));
    }

    out.unwrap_or(false)
}