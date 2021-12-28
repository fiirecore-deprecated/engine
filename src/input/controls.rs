use crate::Context;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

pub mod gamepad;
pub mod keyboard;
// pub mod touchscreen;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Enum)]
pub enum Control {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
}

pub(crate) struct ControlsContext {
    pub keyboard: keyboard::KeyMap,
    pub controller: gamepad::ButtonMap,
    // pub touchscreen: touchscreen::Touchscreen,
}

impl Default for ControlsContext {
    fn default() -> Self {
        Self {
            keyboard: keyboard::default_key_map(),
            controller: gamepad::default_button_map(),
        }
    }
}

pub fn pressed(ctx: &Context, control: Control) -> bool {
    if keyboard::pressed(ctx, control) {
        return true;
    }
    if gamepad::pressed(ctx, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.pressed(&control) {
    //         return true;
    //     }
    // }
    false
}

pub fn down(ctx: &Context, control: Control) -> bool {
    if keyboard::down(ctx, control) {
        return true;
    }
    if gamepad::down(ctx, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.down(&control) {
    //         return true;
    //     }
    // }
    false
}
