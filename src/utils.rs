pub fn lerp(A: f32, B: f32, t: f32) -> f32 {
    return A + (B - A) * t;
}
