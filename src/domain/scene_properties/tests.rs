#![cfg(test)]

use super::*;

#[test]
fn value_coercion() {
    assert!(ScenePropertyValue::Flag(true).flag());
    assert!(ScenePropertyValue::Number(1.0).flag());
    assert!(!ScenePropertyValue::Number(0.0).flag());
    assert!(!ScenePropertyValue::Text("false".into()).flag());
    assert!(!ScenePropertyValue::Text("0".into()).flag());
    assert!(!ScenePropertyValue::Text("  ".into()).flag());
    assert!(ScenePropertyValue::Text("yes".into()).flag());
    assert!(!ScenePropertyValue::Absent.flag());

    assert!((ScenePropertyValue::Number(2.5).number() - 2.5).abs() < f64::EPSILON);
    assert!((ScenePropertyValue::Text("2.5".into()).number() - 2.5).abs() < f64::EPSILON);
    assert!((ScenePropertyValue::Flag(true).number() - 1.0).abs() < f64::EPSILON);
    assert!((ScenePropertyValue::Vector(vec![0.25, 0.5]).number() - 0.25).abs() < 1e-6);

    assert_eq!(ScenePropertyValue::Vector(vec![1.0, 0.0, 0.5]).colour(), Some([1.0, 0.0, 0.5]));
    assert_eq!(ScenePropertyValue::Text("1 0 0.5".into()).colour(), Some([1.0, 0.0, 0.5]));
    assert_eq!(ScenePropertyValue::Vector(vec![1.0, 0.0]).colour(), None);
    assert_eq!(ScenePropertyValue::Number(1.0).colour(), None);
}

#[test]
fn range_degenerate_bounds() {
    let bounded = SceneProperty {
        min: Some(0.5),
        max: Some(3.0),
        step: Some(0.01),
        ..SceneProperty::default()
    };
    assert_eq!(bounded.range(), (0.5, 3.0, 0.01));

    let unbounded = SceneProperty::default();
    let (min, max, step) = unbounded.range();
    assert!((min - 0.0).abs() < f64::EPSILON);
    assert!((max - 1.0).abs() < f64::EPSILON);
    assert!(step > 0.0);

    let inverted = SceneProperty {
        min: Some(2.0),
        max: Some(1.0),
        step: Some(0.0),
        ..SceneProperty::default()
    };
    let (min, max, step) = inverted.range();
    assert!(max > min);
    assert!(step > 0.0);
}

#[test]
fn editable_kinds() {
    let editable = |kind| SceneProperty { kind, ..SceneProperty::default() }.editable();
    assert!(editable(ScenePropertyKind::Flag));
    assert!(editable(ScenePropertyKind::Colour));
    assert!(editable(ScenePropertyKind::Range));
    assert!(editable(ScenePropertyKind::Choice));
    assert!(!editable(ScenePropertyKind::Group));
    assert!(!editable(ScenePropertyKind::Unsupported));
    assert!(
        SceneProperty { kind: ScenePropertyKind::Group, ..SceneProperty::default() }.is_group()
    );
}

#[test]
fn display_formatting() {
    assert_eq!(ScenePropertyValue::Vector(vec![1.0, 0.0, 0.5]).display(), "1.000 0.000 0.500");
    assert_eq!(ScenePropertyValue::Number(2.0).display(), "2");
    assert_eq!(ScenePropertyValue::Number(2.5).display(), "2.500");
    assert_eq!(ScenePropertyValue::Absent.display(), "");
    assert_eq!(parse_vector("  1  0   0.5 "), Some(vec![1.0, 0.0, 0.5]));
    assert_eq!(parse_vector("not numbers"), None);
    assert_eq!(parse_vector(""), None);
}

#[test]
fn conditions_follow_the_engine_expression_shapes() {
    let rows = vec![
        SceneProperty {
            name: "language".into(),
            value: ScenePropertyValue::Number(2.0),
            ..SceneProperty::default()
        },
        SceneProperty {
            name: "clock".into(),
            value: ScenePropertyValue::Flag(true),
            ..SceneProperty::default()
        },
        SceneProperty {
            name: "date".into(),
            value: ScenePropertyValue::Flag(false),
            ..SceneProperty::default()
        },
    ];
    let shown = |condition: &str| {
        SceneProperty { condition: Some(condition.into()), ..SceneProperty::default() }.shown(&rows)
    };
    assert!(shown("language.value == 2"));
    assert!(!shown("language.value == 1"));
    assert!(shown("Language.value === 2 && clock.value"));
    assert!(shown("clock.value == true && date.value == false"));
    assert!(!shown("clock.value && date.value"));
    assert!(shown("(language.value == 1 || language.value == 2) && !date.value"));
    assert!(shown("language.value != 1 && language.value >= 2"));
    assert!(shown("clock.value===true"));
    assert!(shown("unknown.value == 5"));
    assert!(shown("language.value == 2 &&"));
    assert!(shown(""));
    assert!(SceneProperty::default().shown(&rows));
}
