use egui::{InputState, Key, Modifiers, PointerButton};
use std::{mem::zeroed, ops::Deref};

/// Type that can be used as a bind target
pub trait BindTarget: Clone {
    /// Can accept key bind?
    const IS_KEY: bool;
    /// Can accept pointer bind?
    const IS_POINTER: bool;

    /// Can be cleared?
    const CLEARABLE: bool;

    /// Sets new key bind
    fn set_key(&mut self, key: Key, modifiers: Modifiers);

    /// Sets new pointer bind
    fn set_pointer(&mut self, button: PointerButton, modifiers: Modifiers);

    /// Clears the bind
    fn clear(&mut self);

    /// Formats a bind to a string
    fn format(&self) -> String;

    /// Is bind down?
    fn down(&self, input: impl Deref<Target = InputState>) -> bool;

    /// Was bind pressed this frame?
    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool;

    /// Was bind released this frame?
    fn released(&self, input: impl Deref<Target = InputState>) -> bool;
}

impl BindTarget for Key {
    const IS_KEY: bool = true;
    const IS_POINTER: bool = false;
    const CLEARABLE: bool = false;

    fn set_key(&mut self, key: Key, _: Modifiers) {
        *self = key;
    }

    fn set_pointer(&mut self, _: PointerButton, _: Modifiers) {
        unimplemented!()
    }

    fn format(&self) -> String {
        match self {
            Self::Backspace => "BKSP".into(),
            Self::Escape => "ESC".into(),
            Self::Enter => "RET".into(),
            Self::Insert => "INS".into(),
            Self::Delete => "DEL".into(),
            Self::PageUp => "PGU".into(),
            Self::PageDown => "PGD".into(),
            Self::PlusEquals => "=".into(),
            Self::Minus => "-".into(),
            Self::Num0 => "0".into(),
            Self::Num1 => "1".into(),
            Self::Num2 => "2".into(),
            Self::Num3 => "3".into(),
            Self::Num4 => "4".into(),
            Self::Num5 => "5".into(),
            Self::Num6 => "6".into(),
            Self::Num7 => "7".into(),
            Self::Num8 => "8".into(),
            Self::Num9 => "9".into(),
            _ => format!("{self:?}"),
        }
    }

    fn clear(&mut self) {
        unimplemented!()
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        input.key_down(*self)
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        input.key_pressed(*self)
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        input.key_released(*self)
    }
}

macro_rules! option_through {
    ($check:expr, $input:expr, $($path:tt)*) => {
        if let Some(v) = $check {
            v.$($path)*($input)
        } else {
            false
        }
    };
}

impl BindTarget for Option<Key> {
    const IS_KEY: bool = true;
    const IS_POINTER: bool = false;
    const CLEARABLE: bool = true;

    fn set_key(&mut self, key: Key, _: Modifiers) {
        *self = Some(key);
    }

    fn set_pointer(&mut self, _: PointerButton, _: Modifiers) {
        unimplemented!()
    }

    fn format(&self) -> String {
        self.as_ref()
            .map(BindTarget::format)
            .unwrap_or_else(|| "None".into())
    }

    fn clear(&mut self) {
        *self = None;
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        option_through!(self, input, down)
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        option_through!(self, input, pressed)
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        option_through!(self, input, released)
    }
}

impl BindTarget for PointerButton {
    const IS_KEY: bool = false;
    const IS_POINTER: bool = true;
    const CLEARABLE: bool = false;

    fn set_key(&mut self, _: Key, _: Modifiers) {
        unimplemented!()
    }

    fn set_pointer(&mut self, button: PointerButton, _: Modifiers) {
        *self = button;
    }

    fn clear(&mut self) {
        unimplemented!()
    }

    fn format(&self) -> String {
        match self {
            PointerButton::Extra2 => "M5",
            PointerButton::Extra1 => "M4",
            PointerButton::Middle => "M3",
            PointerButton::Secondary => "M2",
            PointerButton::Primary => "M1",
        }
        .into()
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        input.pointer.button_down(*self)
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        input.pointer.button_clicked(*self)
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        input.pointer.button_released(*self)
    }
}

impl BindTarget for Option<PointerButton> {
    const IS_KEY: bool = false;
    const IS_POINTER: bool = true;
    const CLEARABLE: bool = false;

    fn set_key(&mut self, _: Key, _: Modifiers) {
        unimplemented!()
    }

    fn set_pointer(&mut self, button: PointerButton, _: Modifiers) {
        *self = Some(button);
    }

    fn format(&self) -> String {
        self.as_ref()
            .map(BindTarget::format)
            .unwrap_or_else(|| "None".into())
    }

    fn clear(&mut self) {
        *self = None;
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        option_through!(self, input, down)
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        option_through!(self, input, pressed)
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        option_through!(self, input, released)
    }
}

impl<B: BindTarget> BindTarget for (B, Modifiers) {
    const IS_KEY: bool = B::IS_KEY;
    const IS_POINTER: bool = B::IS_POINTER;
    const CLEARABLE: bool = false;

    fn set_key(&mut self, key: Key, modifiers: Modifiers) {
        self.0.set_key(key, modifiers);
        self.1 = modifiers;
    }

    fn set_pointer(&mut self, button: PointerButton, modifiers: Modifiers) {
        self.0.set_pointer(button, modifiers);
        self.1 = modifiers;
    }

    fn clear(&mut self) {
        unimplemented!();
    }

    fn format(&self) -> String {
        let mut prefix = String::with_capacity(4);
        if self.1.ctrl || self.1.command {
            prefix.push('^');
        }

        if self.1.shift {
            prefix.push('_');
        }

        if self.1.alt {
            prefix.push('*');
        }

        prefix + &self.0.format()
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        input.modifiers.matches(self.1) && self.0.down(input)
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        input.modifiers.matches(self.1) && self.0.pressed(input)
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        input.modifiers.matches(self.1) && self.0.released(input)
    }
}

impl<B: BindTarget> BindTarget for Option<(B, Modifiers)> {
    const IS_KEY: bool = B::IS_KEY;
    const IS_POINTER: bool = B::IS_POINTER;
    const CLEARABLE: bool = true;

    fn set_key(&mut self, key: Key, modifiers: Modifiers) {
        unsafe {
            (self as *mut Self).write(Some(zeroed()));
        }

        if let Some((b, m)) = self {
            b.set_key(key, modifiers);
            *m = modifiers;
        }
    }

    fn set_pointer(&mut self, button: PointerButton, modifiers: Modifiers) {
        unsafe {
            (self as *mut Self).write(Some(zeroed()));
        }

        if let Some((b, m)) = self {
            b.set_pointer(button, modifiers);
            *m = modifiers;
        }
    }

    fn clear(&mut self) {
        *self = None;
    }

    fn format(&self) -> String {
        self.as_ref()
            .map(BindTarget::format)
            .unwrap_or_else(|| "None".into())
    }

    fn down(&self, input: impl Deref<Target = InputState>) -> bool {
        if let Some(v) = self {
            v.down(input)
        } else {
            false
        }
    }

    fn pressed(&self, input: impl Deref<Target = InputState>) -> bool {
        if let Some(v) = self {
            v.pressed(input)
        } else {
            false
        }
    }

    fn released(&self, input: impl Deref<Target = InputState>) -> bool {
        if let Some(v) = self {
            v.released(input)
        } else {
            false
        }
    }
}

#[test]
fn test_set_opt() {
    let mut b: Option<(Key, Modifiers)> = None;
    let mods = Modifiers {
        alt: true,
        shift: true,
        ctrl: false,
        command: false,
        mac_cmd: false,
    };
    b.set_key(Key::Tab, mods);

    assert_eq!(b, Some((Key::Tab, mods)));
}
