pub fn lerp(A: f32, B: f32, t: f32) -> f32 {
    return A + (B - A) * t;
}
pub struct IntersectionResult {
    x: f32,
    y: f32,
    offset: f32,
}

pub fn get_intersection(A: Vec2, B: Vec2, C: Vec2, D: Vec2) -> Option<IntersectionResult> {
    let t_top = (D.x - C.x) * (A.y - C.y) - (D.y - C.y) * (A.x - C.x);
    let u_top = (C.y - A.y) * (A.x - B.x) - (C.x - A.x) * (A.y - B.y);
    let bottom = (D.y - C.y) * (B.x - A.x) - (D.x - C.x) * (B.y - A.y);

    if bottom != 0.0 {
        let t = t_top / bottom;
        let u = u_top / bottom;
        if t >= 0. && t <= 1. && u >= 0. && u <= 1. {
            return Some(IntersectionResult {
                x: lerp(A.x, B.x, t),
                y: lerp(A.y, B.y, t),
                offset: t,
            });
        };
    }

    return None;
}
