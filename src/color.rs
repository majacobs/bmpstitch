use std::cmp::PartialEq;
use std::convert::From;
use std::hash::Hash;

pub trait Color: Copy + Clone {
    fn dist(&self, other: &Self) -> f32;
    fn name(&self) -> String;
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r: r, g: g, b: b }
    }

    #[cfg(test)]
    pub fn from_hex(code: u32) -> Self {
        Rgb {
            r: ((code >> 16) & 0xFF) as u8,
            g: ((code >> 8) & 0xFF) as u8,
            b: (code & 0xFF) as u8,
        }
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Eq for Rgb {}

impl From<Hsl> for Rgb {
    fn from(item: Hsl) -> Rgb {
        let h = if item.h == 0.0 { 360.0 } else { item.h };
        let s = item.s;
        let l = item.l;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let (r1, g1, b1) = match h_prime.ceil() as i32 {
            1 => (c, x, 0.0),
            2 => (x, c, 0.0),
            3 => (0.0, c, x),
            4 => (0.0, x, c),
            5 => (x, 0.0, c),
            6 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let m = l - c / 2.0;
        let r = (r1 + m) * 255.0;
        let g = (g1 + m) * 255.0;
        let b = (b1 + m) * 255.0;

        Rgb {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}

impl Color for Rgb {
    fn dist(&self, other: &Self) -> f32 {
        let dr = (self.r as f32) - (other.r as f32);
        let dg = (self.g as f32) - (other.g as f32);
        let db = (self.b as f32) - (other.b as f32);

        (dr.powi(2) + dg.powi(2) + db.powi(2)).sqrt()
    }

    fn name(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Hsl {
    #[cfg(test)]
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        assert!(0.0 <= h && h <= 360.0);
        assert!(0.0 <= s && s <= 1.0);
        assert!(0.0 <= l && l <= 1.0);
        Hsl { h: h, s: s, l: l }
    }
}

impl From<Rgb> for Hsl {
    fn from(item: Rgb) -> Self {
        let r = item.r as f32 / 255.0;
        let g = item.g as f32 / 255.0;
        let b = item.b as f32 / 255.0;

        let (x_max, x_min) = max_and_min(r, g, b);

        let c = x_max - x_min;
        let l = (x_max + x_min) / 2.0;
        let h = if c == 0.0 {
            0.0
        } else if x_max == r {
            60.0 * (0.0 + (g - b) / c)
        } else if x_max == g {
            60.0 * (2.0 + (b - r) / c)
        } else if x_max == b {
            60.0 * (4.0 + (r - g) / c)
        } else {
            panic!("hue conversion");
        };
        let h = (h + 360.0) % 360.0;

        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            2.0 * (x_max - l) / (1.0 - (2.0 * l - 1.0).abs())
        };

        Hsl { h: h, s: s, l: l }
    }
}

impl PartialEq for Hsl {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h && self.s == other.s && self.l == other.l
    }
}

impl Eq for Hsl {}

impl Color for Hsl {
    fn dist(&self, other: &Self) -> f32 {
        // Normalize hue to [0, 1] and have the angle wrap around.
        let phi = ((self.h - other.h).abs() % 360.0) / 360.0;
        let dh = if phi > 0.5 { 1.0 - phi } else { phi };
        let ds = self.s - other.s;
        let dl = self.l - other.l;

        const HUE_WEIGHT: f32 = 1.5;
        (HUE_WEIGHT * dh.powi(2) + ds.powi(2) + dl.powi(2)).sqrt()
    }

    fn name(&self) -> String {
        format!("hsl({}, {}, {})", self.h, self.s, self.l)
    }
}

fn max_and_min<T>(r: T, g: T, b: T) -> (T, T)
where
    T: PartialOrd,
{
    if r > g {
        if g > b {
            (r, b)
        } else if r > b {
            (r, g)
        } else {
            (b, g)
        }
    } else {
        if r > b {
            (g, b)
        } else if g > b {
            (g, r)
        } else {
            (b, r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_and_min_123() {
        assert_eq!(max_and_min(1, 2, 3), (3, 1));
    }

    #[test]
    fn max_and_min_132() {
        assert_eq!(max_and_min(1, 3, 2), (3, 1));
    }

    #[test]
    fn max_and_min_213() {
        assert_eq!(max_and_min(2, 1, 3), (3, 1));
    }

    #[test]
    fn max_and_min_231() {
        assert_eq!(max_and_min(2, 3, 1), (3, 1));
    }

    #[test]
    fn max_and_min_312() {
        assert_eq!(max_and_min(3, 1, 2), (3, 1));
    }

    #[test]
    fn max_and_min_321() {
        assert_eq!(max_and_min(3, 2, 1), (3, 1));
    }

    #[test]
    fn rgb_to_hsl_000000() {
        let hsl: Hsl = Rgb::from_hex(0x000000).into();
        assert_eq!(hsl, Hsl::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn rgb_to_hsl_ffffff() {
        let hsl: Hsl = Rgb::from_hex(0xFFFFFF).into();
        assert_eq!(hsl, Hsl::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn rgb_to_hsl_ff0000() {
        let hsl: Hsl = Rgb::from_hex(0xFF0000).into();
        assert_eq!(hsl, Hsl::new(0.0, 1.0, 0.5));
    }

    #[test]
    fn rgb_to_hsl_00ff00() {
        let hsl: Hsl = Rgb::from_hex(0x00FF00).into();
        assert_eq!(hsl, Hsl::new(120.0, 1.0, 0.5));
    }

    #[test]
    fn rgb_to_hsl_0000ff() {
        let hsl: Hsl = Rgb::from_hex(0x0000FF).into();
        assert_eq!(hsl, Hsl::new(240.0, 1.0, 0.5));
    }

    #[test]
    fn rgb_to_hsl_ea38b9() {
        let hsl: Hsl = Rgb::from_hex(0xEA38B9).into();
        assert!((hsl.h - 316.5).abs() < 0.1);
        assert!((hsl.s - 0.809).abs() < 0.001);
        assert!((hsl.l - 0.569).abs() < 0.001);
    }

    #[test]
    fn hsl_to_rgb_000000() {
        let rgb: Rgb = Hsl::new(0.0, 0.0, 0.0).into();
        assert_eq!(rgb, Rgb::from_hex(0x000000));
    }

    #[test]
    fn hsl_to_rgb_ffffff() {
        let rgb: Rgb = Hsl::new(0.0, 0.0, 1.0).into();
        assert_eq!(rgb, Rgb::from_hex(0xFFFFFF));
    }

    #[test]
    fn hsl_to_rgb_ff0000() {
        let rgb: Rgb = Hsl::new(0.0, 1.0, 0.5).into();
        assert_eq!(rgb, Rgb::from_hex(0xFF0000));
    }

    #[test]
    fn hsl_to_rgb_00ff00() {
        let rgb: Rgb = Hsl::new(120.0, 1.0, 0.5).into();
        assert_eq!(rgb, Rgb::from_hex(0x00FF00));
    }

    #[test]
    fn hsl_to_rgb_0000ff() {
        let rgb: Rgb = Hsl::new(240.0, 1.0, 0.5).into();
        assert_eq!(rgb, Rgb::from_hex(0x0000FF));
    }

    #[test]
    fn hsl_to_rgb_ea38b9() {
        let rgb: Rgb = Hsl::new(316.5, 0.809, 0.569).into();
        assert_eq!(rgb, Rgb::from_hex(0xEA38B9));
    }
}
