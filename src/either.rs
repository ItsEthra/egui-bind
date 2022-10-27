use egui::{Key, PointerButton, Modifiers, InputState};
use crate::BindTarget;
use std::ops::Deref;

/// Bind target that can be either a [`egui::Key`] or a [`egui::PointerButton`]
#[derive(Debug, Clone)]
pub enum KeyOrPointer {
    /// Key bind
    Key(Key),
    /// Pointer bind
    Pointer(PointerButton),
}

impl BindTarget for KeyOrPointer {
    const IS_KEY: bool = true;
    const IS_POINTER: bool = true;
    const CLEARABLE: bool = false;

    fn set_key(&mut self, key: Key, _: Modifiers) {
        *self = Self::Key(key);
    }

    fn set_pointer(&mut self, button: PointerButton, _: Modifiers) {
        *self = Self::Pointer(button);
    }

    fn clear(&mut self) {
        unimplemented!()
    }

    fn format(&self) -> String {
        match self {
            Self::Key(k) => k.format(),
            Self::Pointer(p) => p.format()
        }
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        match self {
            Self::Key(k) => k.down(input),
            Self::Pointer(p) => p.down(input),
        }
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        match self {
            Self::Key(k) => k.pressed(input),
            Self::Pointer(p) => p.pressed(input),
        }
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        match self {
            Self::Key(k) => k.released(input),
            Self::Pointer(p) => p.released(input),
        }
    }
}

impl BindTarget for Option<KeyOrPointer> {
    const IS_KEY: bool = true;
    const IS_POINTER: bool = true;
    const CLEARABLE: bool = true;

    fn set_key(&mut self, key: Key, _: Modifiers) {
        *self = Some(KeyOrPointer::Key(key));
    }

    fn set_pointer(&mut self, button: PointerButton, _: Modifiers) {
        *self = Some(KeyOrPointer::Pointer(button));
    }

    fn clear(&mut self) {
        *self = None
    }

    fn format(&self) -> String {
        match self {
            Some(KeyOrPointer::Key(k)) => k.format(),
            Some(KeyOrPointer::Pointer(p)) => p.format(),
            None => "None".into()
        }
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        self.as_ref().map(|v| v.down(input)).unwrap_or(false)
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        self.as_ref().map(|v| v.pressed(input)).unwrap_or(false)
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        self.as_ref().map(|v| v.released(input)).unwrap_or(false)
    }
}

