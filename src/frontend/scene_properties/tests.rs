#![cfg(test)]

use super::state::SceneProperties;
use crate::domain::scene_properties::{SceneProperty, ScenePropertyKind, ScenePropertyValue};

fn property(name: &str, kind: ScenePropertyKind, value: ScenePropertyValue) -> SceneProperty {
    SceneProperty {
        name: name.to_string(),
        label: name.to_string(),
        kind,
        default: value.clone(),
        value,
        ..SceneProperty::default()
    }
}

fn loaded() -> SceneProperties {
    let mut panel = SceneProperties::opening("scene-a", "Scene A");
    panel.accept(
        "scene-a",
        0,
        &[],
        vec![
            property("header", ScenePropertyKind::Group, ScenePropertyValue::Absent),
            property("glow", ScenePropertyKind::Flag, ScenePropertyValue::Flag(true)),
            property(
                "tint",
                ScenePropertyKind::Colour,
                ScenePropertyValue::Vector(vec![1.0, 1.0, 1.0]),
            ),
            property("zoom", ScenePropertyKind::Range, ScenePropertyValue::Number(1.0)),
            property("shortcut", ScenePropertyKind::Unsupported, ScenePropertyValue::Absent),
        ],
    );
    panel
}

#[test]
fn foreign_reply_ignored() {
    let mut panel = SceneProperties::opening("scene-a", "Scene A");
    panel.accept(
        "scene-b",
        0,
        &[],
        vec![property("glow", ScenePropertyKind::Flag, ScenePropertyValue::Flag(true))],
    );
    assert!(panel.rows.is_empty());
    assert!(panel.loading);

    panel.accept(
        "scene-a",
        0,
        &[],
        vec![property("glow", ScenePropertyKind::Flag, ScenePropertyValue::Flag(true))],
    );
    assert_eq!(panel.rows.len(), 1);
    assert!(!panel.loading);
}

#[test]
fn editable_and_overridden_counts() {
    let mut panel = loaded();
    assert_eq!(panel.editable_count(), 3);
    assert_eq!(panel.changed_count(), 0);

    panel.set_local("glow", ScenePropertyValue::Flag(false));
    assert_eq!(panel.changed_count(), 1);
    assert!(panel.row("glow").unwrap().overridden);

    panel.set_local("glow", ScenePropertyValue::Flag(true));
    assert_eq!(panel.changed_count(), 0);
}

#[test]
fn colour_draft_seeding() {
    let mut panel = loaded();
    assert_eq!(panel.colour_draft("tint"), Some("1.000 1.000 1.000"));
    assert_eq!(panel.colour_draft("glow"), None);

    panel.set_colour_draft("tint", "0.5 0");
    assert_eq!(panel.colour_draft("tint"), Some("0.5 0"));

    panel.accept(
        "scene-a",
        0,
        &[],
        vec![property(
            "tint",
            ScenePropertyKind::Colour,
            ScenePropertyValue::Vector(vec![0.25, 0.5, 0.75]),
        )],
    );
    assert_eq!(panel.colour_draft("tint"), Some("0.5 0"));
}

#[test]
fn failure_keeps_message() {
    let mut panel = SceneProperties::opening("scene-a", "Scene A");
    panel.fail("daemon said no");
    assert!(!panel.loading);
    assert_eq!(panel.error.as_deref(), Some("daemon said no"));
}

#[test]
fn a_late_reply_cannot_revert_an_acknowledged_value() {
    let mut panel = loaded();
    panel.set_local("zoom", ScenePropertyValue::Number(2.0));
    let first_revision = panel.revision;
    let first_rows = panel.rows.clone();
    panel.set_local("zoom", ScenePropertyValue::Number(3.0));
    panel.accept("scene-a", panel.revision, &["zoom".into()], panel.rows.clone());
    panel.accept("scene-a", first_revision, &["zoom".into()], first_rows);
    assert_eq!(panel.row("zoom").unwrap().value.number(), 3.0);
}

#[test]
fn reset_is_immediate_and_preserves_later_edits_when_it_completes() {
    let mut panel = loaded();
    panel.set_local("zoom", ScenePropertyValue::Number(3.0));
    panel.set_colour_draft("tint", "0.2 0");
    panel.reset_local();
    assert_eq!(panel.changed_count(), 0);
    assert_eq!(panel.colour_draft("tint"), Some("1.000 1.000 1.000"));
    let reset_revision = panel.revision;
    let reset_rows = panel.rows.clone();
    panel.set_local("glow", ScenePropertyValue::Flag(false));
    panel.accept(
        "scene-a",
        reset_revision,
        &["glow".into(), "zoom".into(), "tint".into()],
        reset_rows,
    );
    assert!(!panel.row("glow").unwrap().value.flag());
    assert_eq!(panel.changed_count(), 1);
}

#[test]
fn language_controls_change_visibility_and_the_adjustable_count_together() {
    let mut panel = SceneProperties::opening("3299228616", "Lonely Cat");
    let mut english =
        property("clockopacity", ScenePropertyKind::Range, ScenePropertyValue::Number(1.0));
    english.condition = Some("language.value == 1".into());
    let mut vietnamese =
        property("clockopacity1", ScenePropertyKind::Range, ScenePropertyValue::Number(1.0));
    vietnamese.condition = Some("language.value == 2".into());
    panel.accept(
        "3299228616",
        0,
        &[],
        vec![
            property("language", ScenePropertyKind::Choice, ScenePropertyValue::Text("1".into())),
            english,
            vietnamese,
        ],
    );
    assert_eq!(panel.editable_count(), 2);
    assert_eq!(panel.shown_rows()[1].name, "clockopacity");
    panel.set_local("language", ScenePropertyValue::Number(2.0));
    assert_eq!(panel.editable_count(), 2);
    assert_eq!(panel.shown_rows()[1].name, "clockopacity1");
}

#[test]
fn another_property_save_does_not_commit_an_unreleased_slider() {
    let mut panel = loaded();
    panel.set_local("zoom", ScenePropertyValue::Number(2.5));
    panel.set_local("glow", ScenePropertyValue::Flag(false));
    let mut saved = loaded().rows;
    saved.iter_mut().find(|row| row.name == "glow").unwrap().value =
        ScenePropertyValue::Flag(false);
    panel.accept("scene-a", panel.revision, &["glow".into()], saved);
    assert_eq!(panel.row("zoom").unwrap().value.number(), 2.5);
}

#[test]
fn numeric_choices_match_string_defaults_from_wallpaper_engine() {
    let mut panel = SceneProperties::opening("3299228616", "Lonely Cat");
    panel.accept(
        "3299228616",
        0,
        &[],
        vec![property("language", ScenePropertyKind::Choice, ScenePropertyValue::Text("1".into()))],
    );
    panel.set_local("language", ScenePropertyValue::Number(1.0));
    assert_eq!(panel.changed_count(), 0);
    panel.set_local("language", ScenePropertyValue::Number(2.0));
    assert_eq!(panel.changed_count(), 1);
}

#[test]
fn saved_defaults_are_resettable_without_counting_as_changes() {
    let mut panel = loaded();
    let mut saved = panel.rows.clone();
    for row in &mut saved {
        row.overridden = row.editable();
    }
    panel.accept("scene-a", 0, &[], saved);
    assert_eq!(panel.changed_count(), 0);
    assert!(panel.rows.iter().any(|row| row.overridden));
}

#[test]
fn fps_follows_global_until_overridden_and_ignores_stale_replies() {
    let mut panel = SceneProperties::opening("scene-a", "Scene A");
    panel.accept_fps("scene-a", 0, None, Some(30));
    assert_eq!(panel.fps, None);
    assert_eq!(panel.global_fps, Some(30));
    assert_eq!(panel.editable_count(), 1);
    panel.set_fps_local(Some(15));
    panel.accept_fps("scene-a", 0, None, Some(30));
    panel.accept_fps("scene-b", 9, Some(60), Some(60));
    assert_eq!(panel.fps, Some(15));
    panel.accept_fps("scene-a", panel.revision, Some(15), Some(60));
    assert_eq!(panel.global_fps, Some(60));
    assert_eq!(panel.changed_count(), 1);
    panel.reset_local();
    panel.accept_fps("scene-a", 1, Some(15), Some(30));
    assert_eq!(panel.fps, None);
    assert_eq!(panel.global_fps, Some(60));
    assert_eq!(panel.changed_count(), 0);
}
