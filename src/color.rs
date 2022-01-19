use image::Rgba;

pub trait Distance {
    fn distance(&self, other: &Self) -> f32;
}

impl Distance for Rgba<u8> {
    fn distance(&self, other: &Self) -> f32 {
        let dr = (self.0[0] as f32) - (other.0[0] as f32);
        let dg = (self.0[1] as f32) - (other.0[1] as f32);
        let db = (self.0[2] as f32) - (other.0[2] as f32);

        dr.powi(2) + dg.powi(2) + db.powi(2)
    }
}
