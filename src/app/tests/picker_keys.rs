use super::*;
use crate::contracts::settings::wallpaper_kind::{STATIC, VIDEO, WE};
use crate::frontend::browser::Source;
use iced::keyboard::Modifiers;
use iced::keyboard::key::Named;

fn press(app: &mut App, key: keyboard::Key, modifiers: Modifiers) {
    let _ = update(app, Message::KeyPressed(key, modifiers));
}

fn tab(app: &mut App, modifiers: Modifiers) {
    press(app, keyboard::Key::Named(Named::Tab), modifiers);
}

fn character(app: &mut App, text: &str, modifiers: Modifiers) {
    press(app, keyboard::Key::Character(text.into()), modifiers);
}

fn source(app: &App) -> Option<Source> {
    app.source_browser.browser.as_ref().map(|browser| browser.source)
}

#[test]
fn tab_cycles_wallpaper_types_and_wraps() {
    let mut app = test_app();
    for expected in [STATIC, VIDEO, WE, "", STATIC] {
        tab(&mut app, Modifiers::default());
        assert_eq!(app.library_session.filters.kind, expected);
    }
    for expected in ["", WE] {
        tab(&mut app, Modifiers::SHIFT);
        assert_eq!(app.library_session.filters.kind, expected);
    }
}

#[test]
fn type_keys_skip_hidden_chips_and_respect_overlays() {
    let mut app = test_app();
    app.config.set_key("filterBar.show.type.video", json!(false));
    tab(&mut app, Modifiers::default());
    tab(&mut app, Modifiers::default());
    assert_eq!(app.library_session.filters.kind, WE);
    let _ = update(&mut app, Message::OpenPlaylists);
    tab(&mut app, Modifiers::default());
    assert_eq!(app.library_session.filters.kind, WE);
}

#[test]
fn tab_completes_in_search_instead_of_switching_types() {
    let mut app = test_app();
    let _ = update(&mut app, Message::OpenTagCloud);
    tab(&mut app, Modifiers::default());
    tab(&mut app, Modifiers::SHIFT);
    assert_eq!(app.library_session.filters.kind, "");
    assert!(app.tags.cloud_open);
}

#[test]
fn alt_arrows_cycle_sort_chips() {
    let mut app = test_app();
    let sorts: Vec<&str> = crate::frontend::ui::SORTS.iter().map(|(mode, _)| *mode).collect();
    let _ = update(&mut app, Message::SetSort(sorts[0].to_string()));
    for expected in sorts.iter().skip(1).chain(sorts.iter().take(1)) {
        press(&mut app, keyboard::Key::Named(Named::ArrowRight), Modifiers::ALT);
        assert_eq!(&app.library_session.filters.sort, expected);
    }
    press(&mut app, keyboard::Key::Named(Named::ArrowLeft), Modifiers::ALT);
    assert_eq!(app.library_session.filters.sort, *sorts.last().unwrap());
    app.config.set_key(&format!("filterBar.show.sort.{}", sorts[0]), json!(false));
    press(&mut app, keyboard::Key::Named(Named::ArrowRight), Modifiers::ALT);
    assert_eq!(app.library_session.filters.sort, sorts[1]);
    let _ = update(&mut app, Message::OpenPlaylists);
    press(&mut app, keyboard::Key::Named(Named::ArrowRight), Modifiers::ALT);
    assert_eq!(app.library_session.filters.sort, sorts[1]);
}

#[test]
fn ctrl_r_toggles_random_rotation_only_in_the_picker() {
    let mut app = test_app();
    character(&mut app, "r", Modifiers::CTRL);
    assert!(app.config.flag_default_config("general.randomRotate"));
    let _ = update(&mut app, Message::OpenPlaylists);
    character(&mut app, "r", Modifiers::CTRL);
    assert!(app.config.flag_default_config("general.randomRotate"));
    let _ = update(&mut app, Message::ClosePlaylists);
    character(&mut app, "r", Modifiers::CTRL);
    assert!(!app.config.flag_default_config("general.randomRotate"));
}

#[test]
fn ctrl_d_toggles_downloads_and_digits_switch_sources() {
    let mut app = test_app();
    app.config.set_key("sources.bing.enabled", json!(true));
    character(&mut app, "6", Modifiers::default());
    assert_eq!(source(&app), None);
    character(&mut app, "d", Modifiers::CTRL);
    assert_eq!(source(&app), Some(Source::Wallhaven));
    character(&mut app, "6", Modifiers::default());
    assert_eq!(source(&app), Some(Source::Bing));
    character(&mut app, "3", Modifiers::default());
    assert_eq!(source(&app), Some(Source::Bing));
    character(&mut app, "1", Modifiers::default());
    assert_eq!(source(&app), Some(Source::Wallhaven));
    character(&mut app, "d", Modifiers::CTRL);
    assert_eq!(source(&app), None);
    app.config.set_key("features.wallhaven", json!(false));
    app.config.set_key("features.steam", json!(false));
    character(&mut app, "d", Modifiers::CTRL);
    assert_eq!(source(&app), None);
}

#[test]
fn ctrl_tab_switches_search_mode_inside_search() {
    let mut app = test_app();
    tab(&mut app, Modifiers::CTRL);
    assert_eq!(app.tags.search_mode, SearchMode::Tags);
    let _ = update(&mut app, Message::OpenTagCloud);
    tab(&mut app, Modifiers::CTRL);
    assert_eq!(app.tags.search_mode, SearchMode::Describe);
    tab(&mut app, Modifiers::CTRL);
    assert_eq!(app.tags.search_mode, SearchMode::Tags);
}

#[test]
fn text_fields_keep_editing_keys_and_release_the_rest() {
    let reaches = |key: keyboard::Key, modifiers| {
        crate::app::input::reaches_bindings_from_text(&key, modifiers)
    };
    let ctrl_alt = Modifiers::CTRL | Modifiers::ALT;
    assert!(reaches(keyboard::Key::Character("d".into()), Modifiers::CTRL));
    assert!(reaches(keyboard::Key::Character("r".into()), Modifiers::CTRL));
    assert!(reaches(keyboard::Key::Named(Named::ArrowRight), Modifiers::ALT));
    assert!(reaches(keyboard::Key::Character("v".into()), ctrl_alt));
    for reserved in ["a", "c", "v", "x"] {
        assert!(!reaches(keyboard::Key::Character(reserved.into()), Modifiers::CTRL), "{reserved}");
    }
    for reserved in [Named::ArrowLeft, Named::ArrowRight, Named::Backspace, Named::Delete] {
        assert!(!reaches(keyboard::Key::Named(reserved), Modifiers::CTRL), "{reserved:?}");
    }
    assert!(!reaches(keyboard::Key::Character("r".into()), Modifiers::default()));
    assert!(!reaches(keyboard::Key::Named(Named::ArrowRight), Modifiers::SHIFT));
}

#[test]
fn search_field_still_runs_modifier_shortcuts() {
    let mut app = test_app();
    let _ = update(&mut app, Message::OpenTagCloud);
    character(&mut app, "r", Modifiers::CTRL);
    assert!(app.config.flag_default_config("general.randomRotate"));
    let sorts: Vec<&str> = crate::frontend::ui::SORTS.iter().map(|(mode, _)| *mode).collect();
    let started = app.library_session.filters.sort.clone();
    let next = sorts[(sorts.iter().position(|mode| *mode == started).unwrap() + 1) % sorts.len()];
    press(&mut app, keyboard::Key::Named(Named::ArrowRight), Modifiers::ALT);
    assert_eq!(app.library_session.filters.sort, next);
    assert!(app.tags.cloud_open);
    character(&mut app, "d", Modifiers::CTRL);
    assert_eq!(source(&app), Some(Source::Wallhaven));
    assert!(!app.tags.cloud_open);
}

#[test]
fn ui_state_reports_the_sort_chip_and_download_source() {
    let mut app = test_app();
    let state = |app: &App| -> serde_json::Value {
        serde_json::from_str(&crate::app::update::ui_state_json(app)).unwrap()
    };
    press(&mut app, keyboard::Key::Named(Named::ArrowRight), Modifiers::ALT);
    assert_eq!(state(&app)["filter"]["sort"], json!(app.library_session.filters.sort));
    assert_eq!(state(&app)["downloads"]["open"], json!(false));
    character(&mut app, "d", Modifiers::CTRL);
    assert_eq!(state(&app)["downloads"]["open"], json!(true));
    assert_eq!(state(&app)["downloads"]["source"], json!("wallhaven"));
}
