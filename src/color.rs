use std::cmp::PartialEq;
use std::convert::From;

pub trait Color: Copy + Clone {
    fn dist(&self, other: &Self) -> f32;
    fn name(&self) -> String;
}

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct RgbLinear {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
}

impl Color for Rgb {
    fn dist(&self, other: &Self) -> f32 {
        let dr = (self.r as f32) - (other.r as f32);
        let dg = (self.g as f32) - (other.g as f32);
        let db = (self.b as f32) - (other.b as f32);

        dr.powi(2) + dg.powi(2) + db.powi(2)
    }

    fn name(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
impl From<RgbLinear> for Rgb {
    fn from(item: RgbLinear) -> Rgb {
        Rgb {
            r: (gamma(item.r) * 255.0).round() as u8,
            g: (gamma(item.g) * 255.0).round() as u8,
            b: (gamma(item.b) * 255.0).round() as u8,
        }
    }
}

impl PartialEq for RgbLinear {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Eq for RgbLinear {}

impl Color for RgbLinear {
    fn dist(&self, other: &Self) -> f32 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        dr.powi(2) + dg.powi(2) + db.powi(2)
    }

    fn name(&self) -> String {
        format!("rgb_linear({}, {}, {})", self.r, self.g, self.b)
    }
}

impl From<Rgb> for RgbLinear {
    fn from(item: Rgb) -> Self {
        RgbLinear {
            r: inv_gamma(item.r as f32 / 255.0),
            g: inv_gamma(item.g as f32 / 255.0),
            b: inv_gamma(item.b as f32 / 255.0),
        }
    }
}

fn gamma(u: f32) -> f32 {
    if u <= 0.0031308 {
        12.92 * u
    } else {
        1.055 * u.powf(1.0 / 2.4) - 0.055
    }
}

fn inv_gamma(u: f32) -> f32 {
    if u <= 0.04045 {
        u / 12.92
    } else {
        ((u + 0.055) / 1.055).powf(2.4)
    }
}
