#![cfg(test)]

use serde_json::json;

use super::{bar_intent_message, filter_bar_footprint};
use crate::app::tests::test_app;

#[test]
fn theme_mode_controls_share_the_effective_persisted_value() {
    use crate::contracts::settings::SettingsSource;
    use crate::infrastructure::config::Config;

    for (root, expected) in [
        (json!({}), "dark"),
        (json!({"matugen": {"mode": "light"}}), "light"),
        (json!({"matugen": {"mode": "dark"}}), "dark"),
        (json!({"theme": {"mode": "light"}, "matugen": {"mode": "dark"}}), "light"),
        (json!({"theme": {"mode": "dark"}, "matugen": {"mode": "light"}}), "dark"),
        (json!({"theme": {"mode": "auto"}, "matugen": {"mode": "light"}}), "auto"),
        (json!({"theme": {"mode": ""}, "matugen": {"mode": "light"}}), "light"),
    ] {
        let mut app = test_app();
        app.config = Config::from_data(root);
        assert_eq!(SettingsSource::text(&app.config, skwd_config::keys::theme::MODE), expected);
        assert_eq!(super::theme_bar_model(&app, "skwd-iris", false, 1).mode, expected);
    }
}

#[test]
fn committed_theme_modes_reopen_in_the_bar_and_settings() {
    use crate::app::{App, Message, update};
    use crate::contracts::settings::SettingsSource;
    use crate::frontend::theme_designer::ThemeMsg;
    use crate::infrastructure::config::Config;

    let mut app = test_app();
    app.config.set_key(skwd_config::keys::matugen::MODE, json!("light"));
    for mode in ["dark", "light", "auto"] {
        let _ = update(
            &mut app,
            Message::Theme(ThemeMsg::Option(crate::contracts::picker::theme_setting::MODE, mode)),
        );
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&app.config.config_path).unwrap()).unwrap();
        assert_eq!(saved["theme"]["mode"], mode);
        assert_eq!(saved["matugen"]["mode"], "light");
        let mut reopened = Config::from_data(json!({}));
        reopened.config_path.clone_from(&app.config.config_path);
        assert!(reopened.reload());
        app = App::with_config_using(reopened, |_| {
            crate::infrastructure::ipc::DaemonClient::recording()
        });
        assert_eq!(super::theme_bar_model(&app, "skwd-iris", false, 1).mode, mode);
        assert_eq!(SettingsSource::text(&app.config, skwd_config::keys::theme::MODE), mode);
    }
}

#[test]
fn filter_bar_intents_translate_at_composition() {
    use crate::app::Message;
    use crate::frontend::ui::{BarAction, BarIntent};

    assert!(matches!(
        bar_intent_message(BarIntent::Hover(Some(2), Some(3))),
        Message::BarHover(Some(2), Some(3))
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::Activate(BarAction::Resolution(String::from("4k")))),
        Message::SetResolution(value) if value == "4k"
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::SelectFolder(String::from("nature"))),
        Message::SetFolder(value) if value == "nature"
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::StepVolume(-10)),
        Message::Audio(crate::frontend::audio_panel::AudioMsg::VolumeStep(-10))
    ));
    assert!(matches!(
        bar_intent_message(BarIntent::Theme(
            crate::frontend::theme_designer::ThemeMsg::BackendMenu
        )),
        Message::Theme(crate::frontend::theme_designer::ThemeMsg::BackendMenu)
    ));
}

#[test]
fn footprint_tracks_controls_and_scale() {
    let mut app = test_app();
    let (_, base_w, base_h) = filter_bar_footprint(&app, 5000.0, 3000.0);
    assert!(base_w > 0.0 && base_h > 0.0);

    for key in ["orient", "resolution", "colors", "theme", "random", "favourites", "folder"] {
        app.config.set_key(&format!("filterBar.show.{key}"), json!(false));
    }
    let (_, trimmed_w, _) = filter_bar_footprint(&app, 5000.0, 3000.0);
    assert!(trimmed_w < base_w, "{trimmed_w} vs {base_w}");

    app.config.set_key(skwd_config::keys::general::UI_SCALE, json!(2.0));
    let (_, scaled_w, scaled_h) = filter_bar_footprint(&app, 5000.0, 3000.0);
    assert!(scaled_w > trimmed_w && scaled_h > base_h, "{scaled_w}x{scaled_h}");
}

#[test]
fn open_menus_keep_footprint() {
    let mut app = test_app();
    app.library_session.folder_options =
        vec![String::from("nature"), String::from("nature/forest"), String::from("cities")];
    let closed = filter_bar_footprint(&app, 1200.0, 700.0);

    app.chrome.bar.menu = Some(crate::frontend::ui::MenuKind::Folders);
    assert_eq!(filter_bar_footprint(&app, 1200.0, 700.0), closed);

    app.theme.bar_open = true;
    let theme_closed = filter_bar_footprint(&app, 1200.0, 700.0);
    app.chrome.bar.menu = Some(crate::frontend::ui::MenuKind::Backends);
    assert_eq!(filter_bar_footprint(&app, 1200.0, 700.0), theme_closed);
}
