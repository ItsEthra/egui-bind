use egui::{Widget, Ui, Response, Key, Id, vec2, Button, Event};
use std::hash::Hash;

pub struct Hotkey<'a> {
    id: Id,
    value: &'a mut Option<Key>
}

impl<'a> Hotkey<'a> {
    pub fn new(id_source: impl Hash, value: &'a mut Option<Key>) -> Self {
        Self {
            id: Id::new(id_source),
            value
        }
    }
}

impl Widget for Hotkey<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let changing = ui.memory().data.get_temp(self.id).unwrap_or(false);

        if changing {
            let mut cancel = false;
            for e in ui.input().events.iter() {
                if matches!(e, Event::Key { pressed: true, .. }) {
                    cancel = true;
                }
            }

            if cancel {
                ui.memory().data.insert_temp(self.id, false);
            }
        }

        let size = ui.spacing().interact_size;
        let r = if changing {
            ui.add_sized(size, Button::new("..."))
        } else {
            ui.add_sized(size, Button::new(self.value.map(|v| format!("{v:?}")).unwrap_or_else(|| "None".into())))
        };

        if r.clicked() {
            ui.memory().data.insert_temp(self.id, true);
        }

        r
    }
}

