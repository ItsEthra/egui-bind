use std::{mem::zeroed, ops::Deref};

use egui::{Key, Modifiers, PointerButton, InputState};

pub trait BindTarget: Clone {
    const IS_KEY: bool;
    const CLEARABLE: bool;

    fn set_key(&mut self, key: Key, modifiers: Modifiers);
    fn set_pointer(&mut self, button: PointerButton, modifiers: Modifiers);
    fn clear(&mut self);
    fn format(&self) -> String;
}

impl BindTarget for Key {
    const IS_KEY: bool = true;
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
            _ => format!("{self:?}"),
        }
    }

    fn clear(&mut self) {
        unimplemented!()
    }
}

impl BindTarget for Option<Key> {
    const IS_KEY: bool = true;
    const CLEARABLE: bool = true;

    fn set_key(&mut self, key: Key, _: Modifiers) {
        *self = Some(key);
    }

    fn set_pointer(&mut self, _: PointerButton, _: Modifiers) {
        unimplemented!()
    }

    fn format(&self) -> String {
        self.as_ref().map(BindTarget::format).unwrap_or_else(|| "None".into())
    }

    fn clear(&mut self) {
        *self = None;
    }
}

impl BindTarget for PointerButton {
    const IS_KEY: bool = false;
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
        }.into()
    }
}

impl BindTarget for Option<PointerButton> {
    const IS_KEY: bool = false;
    const CLEARABLE: bool = false;

    fn set_key(&mut self, _: Key, _: Modifiers) {
        unimplemented!()
    }

    fn set_pointer(&mut self, button: PointerButton, _: Modifiers) {
        *self = Some(button);
    }

    fn format(&self) -> String {
        self.as_ref().map(BindTarget::format).unwrap_or_else(|| "None".into())
    }
    
    fn clear(&mut self) {
        *self = None;
    }
}

impl<B: BindTarget> BindTarget for (B, Modifiers) {
    const IS_KEY: bool = B::IS_KEY;
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
}

impl<B: BindTarget> BindTarget for Option<(B, Modifiers)> {
    const IS_KEY: bool = B::IS_KEY;
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
        self.as_ref().map(BindTarget::format).unwrap_or_else(|| "None".into())
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
        mac_cmd: false
    };
    b.set_key(Key::Tab, mods);

    assert_eq!(b, Some((Key::Tab, mods)));
}
