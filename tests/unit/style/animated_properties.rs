/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use style::values::animated::{Animate, Procedure, ToAnimatedValue};

fn interpolate_rgba(from: RGBA, to: RGBA, progress: f64) -> RGBA {
    let from = from.to_animated_value();
    let to = to.to_animated_value();
    RGBA::from_animated_value(
        from.animate(&to, Procedure::Interpolate { progress })
            .unwrap(),
    )
}

// Color
#[test]
fn test_rgba_color_interepolation_preserves_transparent() {
    assert_eq!(
        interpolate_rgba(RGBA::transparent(), RGBA::transparent(), 0.5),
        RGBA::transparent()
    );
}

#[test]
fn test_rgba_color_interepolation_alpha() {
    assert_eq!(
        interpolate_rgba(RGBA::new(200, 0, 0, 0.4), RGBA::new(0, 200, 0, 0.8), 0.5),
        RGBA::new(67, 133, 0, 0.6)
    );
}

#[test]
fn test_rgba_color_interepolation_out_of_range_1() {
    // Some cubic-bezier functions produce values that are out of range [0, 1].
    // Unclamped cases.
    assert_eq!(
        interpolate_rgba(
            RGBA::from_floats(0.3, 0.0, 0.0, 0.4),
            RGBA::from_floats(0.0, 1.0, 0.0, 0.6),
            -0.5
        ),
        RGBA::new(154, 0, 0, 0.3)
    );
}

#[test]
fn test_rgba_color_interepolation_out_of_range_2() {
    assert_eq!(
        interpolate_rgba(
            RGBA::from_floats(1.0, 0.0, 0.0, 0.6),
            RGBA::from_floats(0.0, 0.3, 0.0, 0.4),
            1.5
        ),
        RGBA::new(0, 154, 0, 0.3)
    );
}

#[test]
fn test_rgba_color_interepolation_out_of_range_clamped_1() {
    assert_eq!(
        interpolate_rgba(
            RGBA::from_floats(1.0, 0.0, 0.0, 0.8),
            RGBA::from_floats(0.0, 1.0, 0.0, 0.2),
            -0.5
        ),
        RGBA::from_floats(1.0, 0.0, 0.0, 1.0)
    );
}

#[test]
fn test_rgba_color_interepolation_out_of_range_clamped_2() {
    assert_eq!(
        interpolate_rgba(
            RGBA::from_floats(1.0, 0.0, 0.0, 0.8),
            RGBA::from_floats(0.0, 1.0, 0.0, 0.2),
            1.5
        ),
        RGBA::from_floats(0.0, 0.0, 0.0, 0.0)
    );
}
