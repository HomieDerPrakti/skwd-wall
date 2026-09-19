use super::{
    App, Message, Pending, SearchMode, drain_calls, filtered_names, seed, test_app, update, wall,
};
use serde_json::{Value, json};

fn command(app: &mut App, value: &str) {
    let _ = update(
        app,
        Message::Daemon(crate::infrastructure::runtime::Wake::Command(value.to_owned())),
    );
}

fn snapshot(app: &App) -> Value {
    serde_json::from_str(&crate::app::update::ui_state_json(app)).unwrap()
}

fn library() -> App {
    let mut app = test_app();
    seed(
        &mut app,
        &[
            wall("member.png", "static", 10, 0),
            wall("outside.png", "static", 20, 0),
            wall("member.mp4", "video", 30, 0),
        ],
    );
    app
}

#[test]
fn library_demo_requires_an_active_session_and_valid_keys() {
    let mut app = library();
    for value in ["library-demo keys []", "library-demo add member.png"] {
        command(&mut app, value);
        assert!(snapshot(&app)["demo"]["library_scope"].is_null());
        assert_eq!(app.library_session.filtered.len(), 3);
    }
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"member.png\"]");
    let before = snapshot(&app)["demo"].clone();
    for value in [
        "library-demo keys nope",
        "library-demo keys [1]",
        "library-demo keys [\"\"]",
        "library-demo keys [\"   \"]",
        "library-demo add",
        "library-demo reset",
    ] {
        command(&mut app, value);
        assert_eq!(snapshot(&app)["demo"], before, "{value}");
        assert_eq!(filtered_names(&app), ["member.png"]);
    }
}

#[test]
fn library_demo_empty_scope_and_staged_adds_are_explicit_and_deduplicated() {
    let mut app = library();
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys []");
    assert!(app.library_session.filtered.is_empty());
    assert_eq!(snapshot(&app)["demo"]["library_scope"], json!({"keys": [], "count": 0}));
    for value in [
        "library-demo add member.mp4",
        "library-demo add member.png",
        "library-demo add member.png",
    ] {
        command(&mut app, &format!("batch-stage scope {}", json!(value)));
    }
    command(&mut app, "batch-commit scope");
    assert_eq!(filtered_names(&app), ["member.png", "member.mp4"]);
    assert_eq!(
        snapshot(&app)["demo"]["library_scope"],
        json!({"keys": ["member.mp4", "member.png"], "count": 2})
    );
}

#[test]
fn library_demo_scope_survives_filter_resets_and_playlist_panel_changes() {
    let mut app = library();
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"member.png\",\"member.mp4\"]");
    for value in [
        "kind video",
        "clear",
        "kind any",
        "open playlists",
        "playlist-demo reset",
        "dismiss",
        "open tags",
        "dismiss",
        "demo begin",
    ] {
        command(&mut app, value);
        app.refilter();
        assert!(!filtered_names(&app).contains(&"outside.png".to_string()), "{value}");
        assert_eq!(snapshot(&app)["demo"]["library_scope"]["count"], 2, "{value}");
    }
    assert_eq!(filtered_names(&app), ["member.png", "member.mp4"]);
}

#[test]
fn library_demo_select_cannot_expand_scope_or_clear_filters_for_outside_keys() {
    let mut app = library();
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"member.mp4\"]");
    command(&mut app, "kind video");
    command(&mut app, "select outside.png");
    assert_eq!(filtered_names(&app), ["member.mp4"]);
    assert_eq!(app.library_session.filters.kind, "video");
    assert_eq!(snapshot(&app)["selection"], "member.mp4");
    command(&mut app, "library-demo add outside.png");
    command(&mut app, "select outside.png");
    assert_eq!(filtered_names(&app), ["outside.png"]);
    assert_eq!(snapshot(&app)["selection"], "outside.png");
}

#[test]
fn library_demo_scope_filters_arrivals_removals_and_catalog_replacement() {
    let mut app = library();
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"member.png\",\"arriving.png\"]");
    app.on_event("skwd.wall.cached", &wall("unwanted.png", "static", 5, 0));
    assert_eq!(filtered_names(&app), ["member.png"]);
    app.on_event("skwd.wall.cached", &wall("arriving.png", "static", 15, 0));
    assert_eq!(filtered_names(&app), ["member.png", "arriving.png"]);
    assert_eq!(app.library_session.visible_count, 2);
    app.on_event("skwd.wall.removed", &json!({"key": "member.png"}));
    assert_eq!(filtered_names(&app), ["arriving.png"]);
    app.on_result(Pending::List, &json!({"wallpapers": [wall("arriving.png", "static", 15, 0), wall("unwanted.png", "static", 5, 0)]}));
    assert_eq!(filtered_names(&app), ["arriving.png"]);
    app.on_event("skwd.wall.file_removed", &json!({"name": "arriving.png"}));
    assert!(app.library_session.filtered.is_empty());
}

#[test]
fn library_demo_scope_intersects_semantic_results_and_later_arrivals() {
    let mut app = library();
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"member.png\",\"late.png\",\"unranked.png\"]");
    app.tags.search_mode = SearchMode::Describe;
    app.tags.semantic.search = "forest".into();
    app.apply_semantic_result(crate::infrastructure::semantic::SemanticResult {
        generation: app.tags.semantic.generation,
        keys: vec![
            "outside.png".into(),
            "member.png".into(),
            "late.png".into(),
            "outsider.png".into(),
        ],
        exclusions: Vec::new(),
        query_ms: 1.0,
        search_ms: 1.0,
        error: None,
    });
    assert_eq!(filtered_names(&app), ["member.png"]);
    for name in ["outsider.png", "unranked.png", "late.png"] {
        app.on_event("skwd.wall.cached", &wall(name, "static", 15, 0));
    }
    assert_eq!(filtered_names(&app), ["member.png", "late.png"]);
    command(&mut app, "clear");
    app.refilter();
    assert_eq!(filtered_names(&app).len(), 3);
    assert!(
        filtered_names(&app)
            .iter()
            .all(|name| ["member.png", "late.png", "unranked.png"].contains(&name.as_str()))
    );
}

#[test]
fn library_demo_scope_uses_keys_independently_of_wallpaper_kind() {
    let mut app = test_app();
    seed(
        &mut app,
        &[
            json!({"key": "we:123", "we_id": "123", "name": "Workshop video", "thumb": "/thumbs/workshop.jpg", "type": "video", "video_file": "/vids/scene.mp4"}),
            wall("outside.mp4", "video", 1, 0),
        ],
    );
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"we:123\"]");
    command(&mut app, "kind video");
    app.refilter();
    assert_eq!(filtered_names(&app), ["Workshop video"]);
    assert_eq!(snapshot(&app)["selection"], "we:123");
}

#[test]
fn library_demo_end_restores_the_original_catalog_filters_and_selection() {
    let mut app = library();
    command(&mut app, "kind static");
    app.refilter();
    command(&mut app, "select outside.png");
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys []");
    command(&mut app, "demo end");
    assert!(snapshot(&app)["demo"]["library_scope"].is_null());
    assert_eq!(filtered_names(&app), ["member.png", "outside.png"]);
    assert_eq!(snapshot(&app)["selection"], "outside.png");
    command(&mut app, "kind any");
    app.refilter();
    assert_eq!(filtered_names(&app).len(), 3);
}

#[test]
fn library_demo_apply_override_cannot_escape_the_explicit_scope() {
    let mut app = library();
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"member.png\"]");
    drain_calls(&app);
    command(&mut app, "apply-source outside.png");
    command(&mut app, "apply * crossfade 360");
    assert!(drain_calls(&app).iter().all(|(method, _)| method != "wall.apply"));
    command(&mut app, "apply-source outside.png");
    command(&mut app, "stage-blur 10");
    assert!(drain_calls(&app).iter().all(|(method, _)| method != "wall.apply"));
    assert!(app.runtime_state.demo.as_ref().unwrap().opening_blur.is_none());
    command(&mut app, "library-demo add outside.png");
    command(&mut app, "apply-source outside.png");
    command(&mut app, "apply * crossfade 360");
    assert!(
        drain_calls(&app)
            .iter()
            .any(|(method, params)| method == "wall.apply" && params["path"] == "/wp/outside.png")
    );
}

#[test]
fn library_demo_renamed_artwork_does_not_inherit_membership_implicitly() {
    let mut app = library();
    command(&mut app, "demo begin");
    command(&mut app, "library-demo keys [\"member.png\"]");
    app.on_event(
        "skwd.wall.file_renamed",
        &json!({"old_name": "member.png", "new_name": "renamed.png"}),
    );
    assert!(app.library_session.filtered.is_empty());
    command(&mut app, "library-demo add static:renamed.png");
    assert_eq!(filtered_names(&app), ["renamed.png"]);
}
