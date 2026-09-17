mod action;
mod map;
mod trigger;

pub use action::{ActiveScopes, InputAction};
pub use map::InputMap;
pub use trigger::{
    KeyId, KeySpec, Mods, MouseButton, MouseSpec, Trigger, binding_config, binding_label,
    parse_binding,
};

#[cfg(test)]
mod tests;
