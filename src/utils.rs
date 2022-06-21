use bevy::math::Vec3;

pub fn lerp(a: f32, b: f32, f: f32) -> f32 {
    return a + f * (b - a);
}

pub fn move_to_point(p1: Vec3, p2: Vec3, f: f32) -> Vec3 {
    return Vec3::new(
        lerp(p1.x, p2.x, f),
        lerp(p1.y, p2.y, f),
        0.
    );
}
