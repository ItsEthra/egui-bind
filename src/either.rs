use crate::BindTarget;
use egui::{Context, Key, Modifiers, PointerButton};

/// Bind target that can be either a [`egui::Key`] or a [`egui::PointerButton`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            Self::Pointer(p) => p.format(),
        }
    }

    fn down(&self, ctx: &Context) -> bool {
        match self {
            Self::Key(k) => k.down(ctx),
            Self::Pointer(p) => p.down(ctx),
        }
    }

    fn pressed(&self, ctx: &Context) -> bool {
        match self {
            Self::Key(k) => k.pressed(ctx),
            Self::Pointer(p) => p.pressed(ctx),
        }
    }

    fn released(&self, ctx: &Context) -> bool {
        match self {
            Self::Key(k) => k.released(ctx),
            Self::Pointer(p) => p.released(ctx),
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
            None => "None".into(),
        }
    }

    fn down(&self, ctx: &Context) -> bool {
        self.as_ref().map(|v| v.down(ctx)).unwrap_or(false)
    }

    fn pressed(&self, ctx: &Context) -> bool {
        self.as_ref().map(|v| v.pressed(ctx)).unwrap_or(false)
    }

    fn released(&self, ctx: &Context) -> bool {
        self.as_ref().map(|v| v.released(ctx)).unwrap_or(false)
    }
}
