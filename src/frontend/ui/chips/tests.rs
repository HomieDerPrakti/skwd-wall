#![cfg(test)]

#[test]
fn chip_width_grows() {
    let short = super::chip_width("Hi", 12.0, 26.0);
    let long = super::chip_width("Much longer label", 12.0, 26.0);
    assert!(long > short);
    assert!(short >= 16.0);
}

#[test]
fn chip_skew_cap() {
    assert_eq!(super::chip_skew(34.0), 10.0);
    assert_eq!(super::chip_skew(46.0), 10.0);
    assert_eq!(super::chip_skew(20.0), 8.0);
    assert!(super::chip_skew(10.0) < 10.0);
}

#[test]
fn chip_hit_edges() {
    let hit = |px, py| super::chip_contains(0.0, 0.0, 100.0, 30.0, 10.0, px, py);
    assert!(!hit(9.0, 0.0));
    assert!(hit(11.0, 0.0));
    assert!(hit(99.0, 0.0));
    assert!(hit(1.0, 30.0));
    assert!(!hit(91.0, 30.0));
    assert!(hit(89.0, 30.0));
    assert!(!hit(50.0, 30.5));
    assert!(!hit(50.0, -0.5));
}

#[test]
fn chips_layout_scroll() {
    let pal = crate::frontend::theme::Palette::default();
    let entries: Vec<crate::domain::library::search::TagEntry> = (0..50)
        .map(|i| crate::domain::library::search::TagEntry {
            tag: format!("tag-number-{i}"),
            count: i + 1,
            selected: false,
            excluded: false,
        })
        .collect();
    let chips = super::TagChips {
        entries: std::rc::Rc::new(entries),
        pal: &pal,
        width: 300.0,
        max_h: 100.0,
        scale: 1.0,
        entrance: 1.0,
        scroll: f32::MAX,
        scroll_target: f32::MAX,
    };
    let (rects, total) = chips.layout();
    assert_eq!(rects.len(), 50);
    assert!(total > chips.max_h);
    assert_eq!(chips.eff_scroll(total), total - chips.max_h);
    let short = super::TagChips { scroll: 40.0, ..chips };
    assert_eq!(short.eff_scroll(50.0), 0.0);
}

#[test]
fn tag_cloud_body_cap() {
    let entries: Vec<crate::domain::library::search::TagEntry> = (0..20)
        .map(|i| crate::domain::library::search::TagEntry {
            tag: format!("tag-{i}"),
            count: i + 1,
            selected: false,
            excluded: false,
        })
        .collect();
    assert!(super::tag_cloud_row_count(&entries, 260.0, 1.0) > 3);
    assert_eq!(super::tag_cloud_body_height(1, 1.0), 27.0);
    assert_eq!(super::tag_cloud_body_height(2, 1.0), 60.0);
    assert_eq!(super::tag_cloud_body_height(3, 1.0), 93.0);
    assert_eq!(super::tag_cloud_body_height(20, 1.0), 93.0);
}

#[test]
fn tag_chip_click_emits_consumer_intent_and_captures() {
    let pal = crate::frontend::theme::Palette::default();
    let chips = super::TagChips {
        entries: std::rc::Rc::new(vec![crate::domain::library::search::TagEntry {
            tag: "nature".into(),
            count: 1,
            selected: false,
            excluded: false,
        }]),
        pal: &pal,
        width: 300.0,
        max_h: 100.0,
        scale: 1.0,
        entrance: 1.0,
        scroll: 0.0,
        scroll_target: 0.0,
    };
    let event = iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left));
    let cursor = iced::mouse::Cursor::Available(iced::Point::new(10.0, 10.0));
    let action = iced::widget::canvas::Program::update(
        &chips,
        &mut (),
        &event,
        iced::Rectangle::new(iced::Point::ORIGIN, iced::Size::new(300.0, 100.0)),
        cursor,
    )
    .expect("tag click");
    let (message, _, status) = action.into_inner();
    assert!(matches!(
        message,
        Some(crate::frontend::tagcloud::TagIntent::Update(
            crate::frontend::tagcloud::TagMsg::CloudClick(tag, false)
        )) if tag == "nature"
    ));
    assert_eq!(status, iced::event::Status::Captured);
}

#[test]
fn tag_cloud_chips_flow_from_the_right_under_rtl() {
    let entries: Vec<crate::domain::library::search::TagEntry> = (0..12)
        .map(|i| crate::domain::library::search::TagEntry {
            tag: format!("tag-{i}"),
            count: i + 1,
            selected: false,
            excluded: false,
        })
        .collect();
    let width = 260.0;
    let (ltr, ltr_total) = super::tag_cloud_chip_layout(&entries, width, 1.0, false);
    let (rtl, rtl_total) = super::tag_cloud_chip_layout(&entries, width, 1.0, true);
    assert_eq!(ltr_total, rtl_total);
    assert_eq!(ltr.len(), rtl.len());
    assert_eq!(ltr[0].0, 0.0);
    assert!((rtl[0].0 + rtl[0].2 - width).abs() < 1e-4);
    for (a, b) in ltr.iter().zip(&rtl) {
        assert_eq!(a.1, b.1);
        assert_eq!(a.2, b.2);
        assert!((b.0 - (width - a.0 - a.2)).abs() < 1e-4);
    }
    let same_row = ltr.iter().zip(&rtl).filter(|(a, _)| a.1 == ltr[0].1).count();
    assert!(same_row > 1);
    assert!(rtl[1].0 + rtl[1].2 < rtl[0].0);
    assert!(ltr.iter().any(|r| r.1 > 0.0));
}

#[test]
fn mirrored_tag_chip_hit_resolves_same_entry() {
    let entries: Vec<crate::domain::library::search::TagEntry> = (0..6)
        .map(|i| crate::domain::library::search::TagEntry {
            tag: format!("tag-{i}"),
            count: 1,
            selected: false,
            excluded: false,
        })
        .collect();
    let width = 240.0;
    let h = super::TAG_CHIP_H;
    let (ltr, _) = super::tag_cloud_chip_layout(&entries, width, 1.0, false);
    let (rtl, _) = super::tag_cloud_chip_layout(&entries, width, 1.0, true);
    let hit = |rects: &[(f32, f32, f32)], px: f32, py: f32| {
        rects.iter().position(|&(x, y, w)| super::chip_contains(x, y, w, h, 0.0, px, py))
    };
    for (index, &(x, y, w)) in ltr.iter().enumerate() {
        let (cx, cy) = (x + w / 2.0, y + h / 2.0);
        assert_eq!(hit(&ltr, cx, cy), Some(index));
        assert_eq!(hit(&rtl, width - cx, cy), Some(index));
    }
}
