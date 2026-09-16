#![cfg(test)]

#[test]
fn slider_value_map() {
    let map = |x| super::slider_value(0.0, 100.0, 1.0, x, 120.0);
    assert_eq!(map(0.0), 0.0);
    assert_eq!(map(120.0), 100.0);
    assert_eq!(map(60.0), 50.0);
    assert_eq!(map(-50.0), 0.0);
    assert_eq!(map(500.0), 100.0);
    assert_eq!(super::slider_value(0.0, 10.0, 2.0, 60.0, 120.0), 6.0);
}

#[test]
fn marker_snap_raster() {
    for step in 0..400 {
        let raw = 7.0 + step as f32 * 0.173;
        let snapped = super::marker_snap(raw);
        assert_eq!(snapped * 2.0, (snapped * 2.0).round());
        assert!((snapped - raw).abs() <= 0.25 + 1e-4, "snap drift");
    }
}

#[test]
fn slider_fraction_clamps() {
    assert_eq!(super::slider_fraction(0.0, 100.0, 50.0), 0.5);
    assert_eq!(super::slider_fraction(0.0, 100.0, -1.0), 0.0);
    assert_eq!(super::slider_fraction(0.0, 100.0, 101.0), 1.0);
    assert_eq!(super::slider_fraction(4.0, 4.0, 4.0), 0.0);
    assert_eq!(super::slider_fraction(5.0, 4.0, 4.5), 0.0);
}

#[test]
fn track_x_round_trips_both_directions() {
    for rtl in [false, true] {
        for x in [0.0f32, 12.5, 60.0, 120.0] {
            assert_eq!(super::track_x(rtl, super::track_x(rtl, x, 120.0), 120.0), x);
        }
    }
    assert_eq!(super::track_x(false, 30.0, 120.0), 30.0);
    assert_eq!(super::track_x(true, 30.0, 120.0), 90.0);
}

#[test]
fn slider_value_mirrors_under_rtl() {
    let pal = crate::frontend::theme::Palette::default();
    let slider = super::FolioSlider {
        min: 0.0,
        max: 100.0,
        value: 25.0,
        step: 1.0,
        pal,
        on_change: Box::new(|value: f64| value),
        on_release: 0.0,
    };
    let inset = super::FOLIO_SLIDER_INSET;
    let width = 120.0 + inset * 2.0;
    for value in [0.0f64, 25.0, 50.0, 87.0, 100.0] {
        let fraction = super::slider_fraction(0.0, 100.0, value);
        let ltr_x = inset + 120.0 * fraction;
        let rtl_x = inset + super::track_x(true, 120.0 * fraction, 120.0);
        assert_eq!(slider.value_at(false, ltr_x, width), value);
        assert_eq!(slider.value_at(true, rtl_x, width), value);
        assert!((rtl_x - (width - ltr_x)).abs() < 1e-4);
    }
    assert_eq!(slider.value_at(true, inset, width), 100.0);
    assert_eq!(slider.value_at(true, width - inset, width), 0.0);
    assert_eq!(slider.value_at(false, inset, width), 0.0);
}
