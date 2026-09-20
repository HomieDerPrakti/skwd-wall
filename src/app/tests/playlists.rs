use super::*;

#[test]
fn playlist_name_persists() {
    let mut app = test_app();
    app.open_card_picker(String::from("wallpaper-key"), String::from("Wallpaper"));
    let _ = drain_calls(&app);

    let _ =
        update(&mut app, Message::CardPicker(crate::frontend::playlists::CardPickerMsg::NewSubmit));

    let calls = drain_calls(&app);
    assert!(
        calls.iter().any(|(method, params)| {
            method == "playlist.create" && params["name"] == "Playlist 1"
        })
    );
    assert_eq!(app.panels.card_picker.as_ref().unwrap().new_buf, "");
}

fn picker_lists(app: &mut App) {
    use crate::frontend::playlists::PlMsg;
    let _ = update(app, Message::Pl(PlMsg::OpenPicker));
    let id = app
        .daemon
        .pending
        .iter()
        .find_map(|(&id, pending)| matches!(pending, Pending::PlList).then_some(id))
        .unwrap();
    respond(
        app,
        id,
        json!({"playlists": [
        {"id": 17, "name": "Same name", "kind": "curated"},
        {"id": 42, "name": "Same name", "kind": "smart", "source": "ratio:portrait"}
    ], "assign": []}),
    );
}

fn member_request(app: &App) -> u64 {
    app.daemon
        .pending
        .iter()
        .find_map(|(&id, pending)| matches!(pending, Pending::PlMembers).then_some(id))
        .unwrap()
}

#[test]
fn playlist_picker_filters_members_without_starting_playback() {
    use crate::frontend::playlists::PlMsg;
    let mut app = test_app();
    seed(&mut app, &[wall("one.png", "static", 1, 0), wall("two.png", "static", 2, 0)]);
    let key = app.library_session.library.catalog().items[0].key.clone();
    picker_lists(&mut app);
    assert!(app.library_session.playlist_filter.is_none());
    assert!(!app.daemon.pending.values().any(|pending| matches!(pending, Pending::PlMembers)));
    let _ = update(&mut app, Message::Pl(PlMsg::Browse(Some(42))));
    let id = member_request(&app);
    respond(&mut app, id, json!({"id": 42, "members": [{"key": key}]}));
    assert!(app.panels.playlists.is_none());
    assert_eq!(app.library_session.filtered.len(), 1);
    assert_eq!(app.library_session.playlist_filter.as_ref().unwrap().0, 42);
    assert!(!drain_calls(&app).iter().any(|(method, _)| matches!(
        method.as_str(),
        "playlist.assign" | "wall.playlist.next" | "wall.apply"
    )));
    let _ = update(&mut app, Message::Pl(PlMsg::Browse(None)));
    assert!(app.library_session.playlist_filter.is_none());
    assert_eq!(app.library_session.filtered.len(), 2);
}

#[test]
fn playlist_picker_cancel_preserves_filter_and_ignores_late_response() {
    use crate::frontend::playlists::PlMsg;
    let mut app = test_app();
    app.library_session.playlist_filter =
        Some((17, "Old".into(), std::collections::HashSet::default()));
    picker_lists(&mut app);
    let _ = update(&mut app, Message::Pl(PlMsg::Browse(Some(42))));
    let id = member_request(&app);
    let _ = update(&mut app, Message::ClosePlaylists);
    respond(&mut app, id, json!({"id": 42, "members": []}));
    assert_eq!(app.library_session.playlist_filter.as_ref().unwrap().0, 17);
    assert!(app.panels.playlists.is_none());
}

#[test]
fn playlist_picker_rejects_an_older_selection_response() {
    use crate::frontend::playlists::PlMsg;
    let mut app = test_app();
    picker_lists(&mut app);
    let _ = update(&mut app, Message::Pl(PlMsg::Browse(Some(17))));
    let old = member_request(&app);
    let _ = update(&mut app, Message::Pl(PlMsg::Browse(Some(42))));
    respond(&mut app, old, json!({"id": 17, "members": []}));
    assert!(app.library_session.playlist_filter.is_none());
    assert!(app.panels.playlists.is_some());
    let current = member_request(&app);
    respond(&mut app, current, json!({"id": 42, "members": []}));
    assert_eq!(app.library_session.playlist_filter.as_ref().unwrap().0, 42);
    assert!(app.panels.playlists.is_none());
}

#[test]
fn playlist_filter_help_escape_keeps_editor_and_source() {
    use crate::frontend::playlists::PlMsg;
    let mut app = test_app();
    picker_lists(&mut app);
    let _ = update(&mut app, Message::Pl(PlMsg::EditPlaylists));
    let _ = update(&mut app, Message::Pl(PlMsg::Select(42)));
    let _ = update(&mut app, Message::Pl(PlMsg::FilterHelp(true)));
    assert!(app.panels.playlists.as_ref().unwrap().filter_help);
    app.close_overlay(crate::app::overlay::Overlay::Playlists);
    let editor = app.panels.playlists.as_ref().unwrap();
    assert!(!editor.filter_help);
    assert!(!editor.picker);
    assert_eq!(editor.source_buf, "ratio:portrait");
}
