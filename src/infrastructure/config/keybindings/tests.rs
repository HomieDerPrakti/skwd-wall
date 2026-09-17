#![cfg(test)]

use serde_json::json;

use super::*;
use crate::domain::input::{InputAction, KeyId, Mods, MouseButton, MouseSpec};

const PICKER: crate::domain::input::ActiveScopes =
    crate::domain::input::ActiveScopes { fields: false, search: false, downloads: false };

#[test]
fn key_overrides_decode() {
    let config = Config::from_data(json!({
        "keys": {
            "playlists": "x",
            "flip": "banana+q",
            "effects": "ctrl+e",
            "select": "middle-click",
            "themePanel": "ctrl+c",
        }
    }));
    let bindings = load_bindings(&config);
    assert_eq!(
        bindings.lookup_key(&KeyId::Char("x".into()), Mods::NONE, PICKER),
        Some(InputAction::Playlists)
    );
    assert_eq!(
        bindings.lookup_mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Right }, PICKER),
        Some(InputAction::Flip)
    );
    assert_eq!(
        bindings.lookup_key(&KeyId::Char("e".into()), Mods::new(true, false, false), PICKER),
        Some(InputAction::Effects)
    );
    assert_eq!(
        bindings.lookup_mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Middle }, PICKER),
        Some(InputAction::Select)
    );
    assert_eq!(
        bindings.lookup_key(&KeyId::Char("c".into()), Mods::new(true, false, false), PICKER),
        Some(InputAction::ThemePanel)
    );
}
