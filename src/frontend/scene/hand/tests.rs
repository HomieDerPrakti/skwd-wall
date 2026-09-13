#![cfg(test)]

use super::*;
use glam::{Mat4, Vec3};

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

#[test]
fn bezier_endpoints_and_monotone_linear() {
    assert_eq!(EASE_DEAL_OUT.at(0.0), 0.0);
    assert_eq!(EASE_DEAL_OUT.at(1.0), 1.0);
    for step in 0..=10 {
        let x = step as f32 / 10.0;
        assert!(close(EASE_LINEAR.at(x), x), "{x}");
    }
    assert!(EASE_DEAL_IN.at(0.5) > 0.9);
    assert!(EASE_TWIST.at(0.5) > 1.0);
}

#[test]
fn css_rotation_handedness() {
    let p = rot(Y, 90.0).transform_point3(Vec3::X);
    assert!(close(p.x, 0.0) && close(p.z, -1.0));
    let p = rot(X, 90.0).transform_point3(Vec3::Y);
    assert!(close(p.y, 0.0) && close(p.z, 1.0));
    let p = rot(Z, 90.0).transform_point3(Vec3::X);
    assert!(close(p.x, 0.0) && close(p.y, 1.0));
}

#[test]
fn matrix_product_applies_right_first() {
    let m = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0)) * Mat4::from_scale(Vec3::splat(2.0));
    assert_eq!(m.transform_point3(Vec3::new(1.0, 1.0, 0.0)), Vec3::new(12.0, 2.0, 0.0));
}

#[test]
fn pose_mix_matching_orders_keeps_multi_turn() {
    let from = fan_pose(1.0, &DESIGN_FAN, 1.0, 1.0, 0.0, 0.0, 1.0);
    let to = move_pose(Move::Corkscrew, 1.0, 1.0, 1.0);
    let half = Pose::mix(&from, &to, 0.5);
    assert_eq!(half.n, 2);
    assert!(close(half.rots[0].deg, (-12.0 + 1030.0) * 0.5));
    assert!(close(half.s, 0.54));
    assert_eq!(Pose::mix(&from, &to, 0.0), from);
}

#[test]
fn pose_mix_mismatched_orders_slerps_shortest_path() {
    let from = fan_pose(0.0, &DESIGN_FAN, 1.0, 1.0, 0.0, 0.0, 1.0);
    let to = move_pose(Move::Shuffle, 0.0, 1.0, 1.0);
    let half = Pose::mix(&from, &to, 0.5);
    assert_eq!(half.n, 1);
    assert!(half.rots[0].deg.abs() < 180.0);
    let start = Pose::mix(&from, &to, 0.0);
    let p = start.matrix().transform_point3(Vec3::new(10.0, 0.0, 0.0));
    assert!(close(p.x, 10.0) && close(p.y, 0.0));
}

#[test]
fn pose_tween_respects_delay_and_end() {
    let tween = PoseTween::new(Pose::REST, sel_pose(1.0), 100.0, 400.0, EASE_LINEAR);
    assert!(tween.at(0.05).matrix().abs_diff_eq(Mat4::IDENTITY, 1e-5));
    assert!(close(tween.at(0.3).t[2], 85.0));
    assert!(close(tween.end(), 0.5));
    assert!(close(tween.at(1.0).t[1], -40.0));
}

#[test]
fn ribbon_cuts_partition_the_card() {
    let cuts = ribbon_cuts(6, 432.0, Cut::Straight, Variance::None, 0.31, 0, 2.0);
    assert_eq!(cuts.len(), 6);
    assert!(cuts[0].a0 < -218.0 && cuts[0].a1 == cuts[0].a0);
    assert!(cuts[5].b1 > 218.0 && cuts[5].b0 == cuts[5].b1);
    for pair in cuts.windows(2) {
        assert!(close(pair[0].b0 - pair[1].a0, 2.0 * RIBBON_OVERLAP));
        assert!(close(pair[0].b1 - pair[1].a1, 2.0 * RIBBON_OVERLAP));
    }
    let slanted = ribbon_cuts(6, 432.0, Cut::Steep, Variance::None, 0.31, 0, 2.0);
    assert!(close(slanted[1].a1 - slanted[1].a0, 2.4 * 72.0));
    assert!(close(slanted[1].mid, -108.0));
    let wild = ribbon_cuts(6, 432.0, Cut::Slant, Variance::Wild, 0.31, 2, 2.0);
    let widths: Vec<f32> = wild.iter().map(|c| c.mid).collect();
    assert!(widths.windows(2).all(|w| w[1] > w[0]));
    for seed in 0..40 {
        for nr in [2usize, 6, 14] {
            let cuts =
                ribbon_cuts(nr, 432.0, Cut::Steep, Variance::Wild, seed as f32 * 0.037, seed, 2.0);
            for cut in &cuts {
                assert!(
                    cut.b0 > cut.a0 && cut.b1 > cut.a1,
                    "bow-tie ribbon at seed {seed} nr {nr}: {cut:?}"
                );
            }
        }
    }
}

#[test]
fn ribbon_locals_follow_corner_order() {
    let cut = RibbonCut { a0: 1.0, a1: 2.0, b0: 3.0, b1: 4.0, mid: 2.5 };
    assert_eq!(ribbon_locals(&cut, Axis::Rows), [1.0, 2.0, 4.0, 3.0]);
    assert_eq!(ribbon_locals(&cut, Axis::Columns), [1.0, 3.0, 4.0, 2.0]);
    let rows = ribbon_corners(&cut, Axis::Rows, 84.0, 216.0, 0.0);
    assert_eq!(rows[1], [84.0, 2.0, 0.0]);
    let cols = ribbon_corners(&cut, Axis::Columns, 84.0, 216.0, 0.0);
    assert_eq!(cols[3], [2.0, 216.0, 0.0]);
}

#[test]
fn projection_identity_on_the_anchor_plane() {
    let cam = Camera { d: 1700.0, origin: [800.0, 400.0], shift: [0.0, 16.0] };
    let p = cam.project([50.0, -30.0, 0.0]);
    assert!(close(p[0], 850.0) && close(p[1], 370.0) && close(p[2], 1.0));
    let near = cam.project([50.0, 16.0, 850.0]);
    assert!(close(near[0], 900.0) && close(near[2], 0.5));
}

#[test]
fn quad_facing_flips_with_rotation() {
    let cam = Camera { d: 1700.0, origin: [0.0, 0.0], shift: [0.0, 0.0] };
    let front = project_quad(&cam, &Mat4::IDENTITY, &card_corners(84.0, 216.0, 0.0, 0.0));
    assert!(front.facing());
    let back = project_quad(&cam, &rot(Y, 180.0), &card_corners(84.0, 216.0, 0.0, 0.0));
    assert!(!back.facing());
    let edge_on = project_quad(&cam, &rot(X, 90.0), &card_corners(84.0, 216.0, 0.0, 0.0));
    let [_, _, _, hh] = edge_on.bounds();
    assert!(hh < 1.0);
}

#[test]
fn flip_ribbons_land_flat_and_return_home() {
    let nr = 6;
    for r in 0..nr {
        let landed = flip_ribbon_pose(flip_open_ms(nr) + 1.0, false, r, nr, 1.0, 1.0, Axis::Rows);
        assert_eq!(landed.rots[0].axis, X);
        assert!(close(landed.rots[0].deg, 180.0), "ribbon {r}: {}", landed.rots[0].deg);
        assert!(close(landed.t[2], 0.0));
        let home = flip_ribbon_pose(flip_close_ms(nr) + 1.0, true, r, nr, 1.0, 1.0, Axis::Rows);
        assert!(close(home.rots[0].deg, 360.0));
        let lifted = flip_ribbon_pose(300.0, false, r, nr, 1.0, 1.0, Axis::Rows);
        assert!(lifted.t[2] >= 0.0);
    }
    let start = flip_ribbon_pose(0.0, false, 0, nr, 1.0, 1.0, Axis::Rows);
    assert!(start.matrix().abs_diff_eq(Mat4::IDENTITY, 1e-5));
}

#[test]
fn twist_settles_after_landing() {
    let open = flip_twist_pose(FLIP_LIFT_MS + TWIST_MS * 1.2, false, 1.0);
    assert!(close(open.rots[0].deg, -13.0));
    let closed = flip_twist_pose(FLIP_CLOSE_LIFT_MS + TWIST_MS * 1.2, true, 1.0);
    assert!(close(closed.rots[0].deg, 0.0) && close(closed.s, 1.0));
}

#[test]
fn bob_is_periodic_and_bounded() {
    for step in 0..40 {
        let pose = bob(step as f32 * 0.3, 2, 1.0);
        assert!(pose.t[1] >= -6.001 && pose.t[1] <= 7.001);
    }
    let a = bob(1.0, 0, 1.0);
    let b = bob(1.0 + BOB_PERIOD_S, 0, 1.0);
    assert!(close(a.t[1], b.t[1]));
}

#[test]
fn hand_window_centres_and_clamps() {
    assert_eq!(hand_window(0, 20, 5), 0);
    assert_eq!(hand_window(4, 20, 5), 0);
    assert_eq!(hand_window(5, 20, 5), 5);
    assert_eq!(hand_window(7, 20, 5), 5);
    assert_eq!(hand_window(19, 20, 5), 15);
    assert_eq!(hand_window(11, 12, 5), 7);
    assert_eq!(hand_window(2, 3, 5), 0);
    assert_eq!(hand_len(15, 20, 5), 5);
    assert_eq!(hand_len(0, 3, 5), 3);
}

#[test]
fn move_cycle_wraps() {
    assert_eq!(Move::Spiral.index(), 4);
    assert_eq!(Move::ALL[Move::Cascade.index()], Move::Cascade);
    assert_eq!(Move::from_key("cascade"), Some(Move::Cascade));
    assert_eq!(Move::from_key("cycle"), None);
    assert_eq!(Cut::from_key("steep").factor(), 2.4);
    assert_eq!(Variance::from_key("wild").factor(), 1.05);
    assert_eq!(Axis::from_key("columns"), Axis::Columns);
}

#[test]
fn params_settle_only_when_equal() {
    let base = HandParams::default();
    let mut moving = HandParams { spread: 90.0, ..base };
    assert!(!moving.settled_to(&base));
    for _ in 0..200 {
        moving.morph_toward(&base, 0.2);
    }
    assert!(moving.settled_to(&base));
    let topo = HandParams { ribbons: 9, ..base };
    assert!(!topo.settled_to(&base));
}

#[test]
fn relax_window_is_a_smooth_step() {
    assert_eq!(relax(0.0), 0.0);
    assert_eq!(relax(RELAX_WINDOW.0), 0.0);
    assert_eq!(relax(RELAX_WINDOW.1), 1.0);
    assert_eq!(relax(1.0), 1.0);
    let mid = (RELAX_WINDOW.0 + RELAX_WINDOW.1) * 0.5;
    assert!(relax(mid) > 0.45 && relax(mid) < 0.55);
}

#[test]
fn stage_scale_prefers_the_tighter_axis() {
    assert!(close(stage_scale(1440.0, 900.0), 1.0));
    assert!(close(stage_scale(2560.0, 1440.0), 1.6));
    assert!(close(stage_scale(1024.0, 768.0), 1024.0 / 1440.0));
}

#[test]
fn selected_card_flies_to_the_middle() {
    let sel = sel_pose(1.0);
    assert_eq!(sel.t, [0.0, -40.0, 170.0]);
    assert_eq!(sel.rots[0].deg, 0.0);
    assert_eq!(sel.rots[1].deg, 0.0);
    let half = Pose::mix(&fan_pose(2.0, &DESIGN_FAN, 1.0, 1.24, 0.0, -150.0, 1.0), &sel, 0.5);
    assert!((half.t[0] - 2.0 * 126.0 * 1.24 * 0.5).abs() < 1e-3);
}

#[test]
fn fan_knobs_shape_the_hand() {
    let wide = Fan { spread: 200.0, angle: 30.0, roll: 0.0, arch: 0.0 };
    let pose = fan_pose(2.0, &wide, 1.0, 1.0, 0.0, 0.0, 1.0);
    assert_eq!(pose.t[0], 400.0);
    assert_eq!(pose.t[1], 0.0);
    assert_eq!(pose.rots[0].deg, -60.0);
    assert_eq!(pose.rots[1].deg, 0.0);
    let params = HandParams { skew: 30.0, ..HandParams::default() };
    assert_eq!(params.fan(2.0).spread, 252.0);
    assert_eq!(params.fan(2.0).arch, 40.0);
    assert_eq!(hand_window(15, 16, 16), 0);
    assert_eq!(hand_len(0, 3, 2), 2);
}

#[test]
fn fan_slots_close_the_gap_around_a_lifted_card() {
    assert_eq!(fan_slot(2, 5, None), 0.0);
    assert_eq!(fan_slot(0, 5, None), -2.0);
    assert_eq!(fan_slot(2, 5, Some(3)), 0.5);
    assert_eq!(fan_slot(4, 5, Some(3)), 1.5);
    assert_eq!(fan_slot(0, 5, Some(3)), -1.5);
    assert_eq!(fan_slot(3, 5, Some(3)), 1.0);
    assert_eq!(fan_slot(0, 1, Some(0)), 0.0);
}

#[test]
fn deal_moves_come_from_the_enabled_pool() {
    let only_two = [false, true, true, false, false];
    assert_eq!(pick_move(DealMode::Cycle, only_two, Move::Cascade, 0.0), Move::Shuffle);
    assert_eq!(pick_move(DealMode::Cycle, only_two, Move::Shuffle, 0.0), Move::Cascade);
    assert_eq!(pick_move(DealMode::Cycle, only_two, Move::Spiral, 0.0), Move::Cascade);
    assert_eq!(pick_move(DealMode::Cycle, [false; 5], Move::Spiral, 0.0), Move::Corkscrew);
    for step in 0..20 {
        let seed = step as f32 / 20.0;
        assert_eq!(pick_move(DealMode::Random, only_two, Move::Cascade, seed), Move::Shuffle);
    }
    let picks: Vec<Move> = (0..20)
        .map(|step| pick_move(DealMode::Random, [true; 5], Move::Spiral, step as f32 / 20.0))
        .collect();
    assert!(picks.iter().all(|mv| *mv != Move::Spiral));
    assert!(picks.contains(&Move::Corkscrew) && picks.contains(&Move::Ribbon));
    let solo = [false, false, false, true, false];
    assert_eq!(pick_move(DealMode::Random, solo, Move::Ribbon, 0.7), Move::Ribbon);
    assert_eq!(DealMode::from_key("random"), DealMode::Random);
    assert_eq!(DealMode::from_key("cascade"), DealMode::Cycle);
}

#[test]
fn mirrored_cut_lands_the_back_in_the_ribbon_slot() {
    let cut = RibbonCut { a0: -20.0, a1: 60.0, b0: 52.0, b1: 132.0, mid: 56.0 };
    let back = mirrored_cut(&cut);
    let flip = |y: f32| 2.0 * cut.mid - y;
    assert!(close(flip(back.a0), cut.b0));
    assert!(close(flip(back.a1), cut.b1));
    assert!(close(flip(back.b0), cut.a0));
    assert!(close(flip(back.b1), cut.a1));
    let outer = RibbonCut { a0: -650.0, a1: -650.0, b0: -230.0, b1: -57.0, mid: -182.0 };
    let back = mirrored_cut(&outer);
    let flip = |y: f32| 2.0 * outer.mid - y;
    assert!(close(flip(back.a0), outer.b0) && close(flip(back.a1), outer.b1));
    assert!(close(flip(back.b0), outer.a0) && close(flip(back.b1), outer.a1));
}
