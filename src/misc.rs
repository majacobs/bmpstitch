pub fn distance_u8(a: &(u8, u8, u8), b: &(u8, u8, u8)) -> f32 {
    let diff0 = a.0 as f32 - b.0 as f32;
    let diff1 = a.1 as f32 - b.1 as f32;
    let diff2 = a.2 as f32 - b.2 as f32;
    let sum = diff0.powi(2) + diff1.powi(2) + diff2.powi(2);
    sum.sqrt()
}
