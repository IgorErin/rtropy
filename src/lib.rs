pub fn fround(f: f32, num: u32) -> f32 {
    let m = 10i32.pow(num) as f32;
    (f * m).round() / m
}
