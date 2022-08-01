pub use bevy::prelude::*;
pub use bevy_rapier2d::prelude::*;

use rand::Rng;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

pub fn lerp_inverse(a: f32, b: f32, t: f32) -> f32 {
    return (t - a) / (b - a);
}

pub fn z_from_y(y: f32) -> f32 {
    return lerp(900.0, 100.0, lerp_inverse(-2500.0, 2500.0, y));
}

#[test]
fn test_lerp() {
    assert_eq!(lerp_inverse(0.0, 1.0, 0.5), 0.5);
    assert_eq!(lerp_inverse(-1.0, 1.0, 0.0), 0.5);

    for i in 1..10 {
        assert_eq!(lerp_inverse(0.0, i as f32, 0.5), 0.5 / i as f32);
    }
}

pub fn rand_pos_vec2() -> Vec2 {
    return Vec2::new(
        rand::thread_rng().gen_range(0.0..1.0),
        rand::thread_rng().gen_range(0.0..1.0),
    );
}

pub fn rand_norm_vec2() -> Vec2 {
    return Vec2::new(
        rand::thread_rng().gen_range(-1.0..1.0),
        rand::thread_rng().gen_range(-1.0..1.0),
    )
    .normalize();
}
