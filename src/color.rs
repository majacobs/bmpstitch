use std::cmp::PartialEq;
use std::convert::From;
use std::hash::Hash;
use std::marker::PhantomData;

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

pub trait Illuminant: Copy {
    const SRGB_TO_XYZ_MATRIX: [[f32; 3]; 3];
    const XYZ_TO_SRGB_MATRIX: [[f32; 3]; 3];
    const TRISTIMULUS_X: f32;
    const TRISTIMULUS_Y: f32;
    const TRISTIMULUS_Z: f32;
}

#[derive(Debug, Copy, Clone)]
pub struct IlluminantD65;

impl Illuminant for IlluminantD65 {
    const SRGB_TO_XYZ_MATRIX: [[f32; 3]; 3] = [
        [0.41239080, 0.35758434, 0.18048079],
        [0.21263901, 0.71516868, 0.07219232],
        [0.01933082, 0.11919478, 0.95053215],
    ];

    const XYZ_TO_SRGB_MATRIX: [[f32; 3]; 3] = [
        [3.24096994, -1.53738318, -0.49861076],
        [-0.96924364, 1.87596750, 0.04155506],
        [0.05563008, -0.20397696, 1.05697151],
    ];

    const TRISTIMULUS_X: f32 = 0.95047;

    const TRISTIMULUS_Y: f32 = 1.0;

    const TRISTIMULUS_Z: f32 = 1.08883;
}

#[derive(Debug, Copy, Clone)]
pub struct CieXyz<I: Illuminant> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    phantom: PhantomData<I>,
}

pub type CieXyzD65 = CieXyz<IlluminantD65>;

#[derive(Debug, Copy, Clone)]
pub struct CieLuv<I: Illuminant> {
    pub l: f32,
    pub u: f32,
    pub v: f32,
    phantom: PhantomData<I>,
}

pub type CieLuvD65 = CieXyz<IlluminantD65>;

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

impl<I: Illuminant> From<CieXyz<I>> for Rgb {
    fn from(item: CieXyz<I>) -> Rgb {
        let matrix = I::XYZ_TO_SRGB_MATRIX;
        let r_linear = matrix[0][0] * item.x + matrix[0][1] * item.y + matrix[0][2] * item.z;
        let g_linear = matrix[1][0] * item.x + matrix[1][1] * item.y + matrix[1][2] * item.z;
        let b_linear = matrix[2][0] * item.x + matrix[2][1] * item.y + matrix[2][2] * item.z;

        Rgb {
            r: (gamma(r_linear) * 255.0).round() as u8,
            g: (gamma(g_linear) * 255.0).round() as u8,
            b: (gamma(b_linear) * 255.0).round() as u8,
        }
    }
}
impl<I: Illuminant> From<CieLuv<I>> for Rgb {
    fn from(item: CieLuv<I>) -> Rgb {
        CieXyz::from(item).into()
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

impl<I: Illuminant> CieXyz<I> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        CieXyz {
            x: x,
            y: y,
            z: z,
            phantom: PhantomData,
        }
    }
}

impl<I: Illuminant> PartialEq for CieXyz<I> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<I: Illuminant> Eq for CieXyz<I> {}

impl<I: Illuminant> Color for CieXyz<I> {
    fn dist(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }

    fn name(&self) -> String {
        unimplemented!();
    }
}

impl<I: Illuminant> From<Rgb> for CieXyz<I> {
    fn from(item: Rgb) -> CieXyz<I> {
        let r_linear = inv_gamma(item.r as f32 / 255.0);
        let g_linear = inv_gamma(item.g as f32 / 255.0);
        let b_linear = inv_gamma(item.b as f32 / 255.0);

        let matrix = I::SRGB_TO_XYZ_MATRIX;
        CieXyz {
            x: matrix[0][0] * r_linear + matrix[0][1] * g_linear + matrix[0][2] * b_linear,
            y: matrix[1][0] * r_linear + matrix[1][1] * g_linear + matrix[1][2] * b_linear,
            z: matrix[2][0] * r_linear + matrix[2][1] * g_linear + matrix[2][2] * b_linear,
            phantom: PhantomData,
        }
    }
}
impl<I: Illuminant> CieLuv<I> {
    pub fn new(l: f32, u: f32, v: f32) -> Self {
        CieLuv {
            l: l,
            u: u,
            v: v,
            phantom: PhantomData,
        }
    }
}

impl<I: Illuminant> PartialEq for CieLuv<I> {
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.u == other.u && self.v == other.v
    }
}

impl<I: Illuminant> Eq for CieLuv<I> {}

impl<I: Illuminant> Color for CieLuv<I> {
    fn dist(&self, other: &Self) -> f32 {
        let dl = self.l - other.l;
        let du = self.u - other.u;
        let dv = self.v - other.v;

        (dl.powi(2) + du.powi(2) + dv.powi(2)).sqrt()
    }

    fn name(&self) -> String {
        unimplemented!();
    }
}

impl<I: Illuminant> From<CieXyz<I>> for CieLuv<I> {
    fn from(item: CieXyz<I>) -> CieLuv<I> {
        fn u(x:f32, y:f32, z:f32) -> f32 { 4.0 * x / (x + 15.0 * y + 3.0 * z)}
        fn v(x:f32, y:f32, z:f32) -> f32 { 9.0 * y / (x + 15.0 * y + 3.0 * z)}

        const EPISILON: f32 = 216.0 / 24389.0;
        const KAPPA: f32 = 24389.0 / 27.0;

        let u_prime = u(item.x, item.y, item.z);
        let v_prime = v(item.x, item.y, item.z);
        let u_prime_r = u(I::TRISTIMULUS_X, I::TRISTIMULUS_Y, I::TRISTIMULUS_Z);
        let v_prime_r = v(I::TRISTIMULUS_X, I::TRISTIMULUS_Y, I::TRISTIMULUS_Z);

        let y_r = item.y / I::TRISTIMULUS_Y;

        let l = if y_r > EPISILON {
            116.0 * y_r.cbrt() - 16.0
        } else {
            KAPPA * y_r
        };

        CieLuv {
            l: l,
            u: 13.0 * l * (u_prime - u_prime_r),
            v: 13.0 * l * (v_prime - v_prime_r),
            phantom: PhantomData,
        }
    }
}
impl<I: Illuminant> From<CieLuv<I>> for CieXyz<I> {
    fn from(item: CieLuv<I>) -> CieXyz<I> {
        fn u(x:f32, y:f32, z:f32) -> f32 { 4.0 * x / (x + 15.0 * y + 3.0 * z)}
        fn v(x:f32, y:f32, z:f32) -> f32 { 9.0 * y / (x + 15.0 * y + 3.0 * z)}

        const EPSILON: f32 = 216.0 / 24389.0;
        const KAPPA: f32 = 24389.0 / 27.0;

        let u_0 = u(I::TRISTIMULUS_X, I::TRISTIMULUS_Y, I::TRISTIMULUS_Z);
        let v_0 = v(I::TRISTIMULUS_X, I::TRISTIMULUS_Y, I::TRISTIMULUS_Z);

        let y = if item.l > KAPPA * EPSILON {
            ((item.l + 16.0) / 116.0).powi(3)
        } else {
            item.l / KAPPA
        };

        let a = (52.0 * item.l / (item.u + 13.0 * item.l * u_0) - 1.0) / 3.0;
        let b = -5.0 * y;
        let c = -1.0 / 3.0;
        let d = y * (39.0 * item.l / (item.v + 13.0 * item.l * v_0) - 5.0);

        let x = (d - b) / (a - c);

        CieXyz {
            x: x,
            y: y,
            z: x * a + b,
            phantom: PhantomData,
        }
    }
}

impl<I: Illuminant> From<Rgb> for CieLuv<I> {
    fn from(item: Rgb) -> CieLuv<I> {
        CieXyz::from(item).into()
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if ($x - $y).abs() > $d {
                panic!("failed {} \u{2248} {} within {}", $x, $y, $d);
            }
        };
    }

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

    #[test]
    fn rgb_to_ciexyz_000000() {
        let xyz: CieXyz<IlluminantD65> = Rgb::from_hex(0x000000).into();
        assert_eq!(xyz, CieXyz::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn rgb_to_ciexyz_ffffff() {
        let xyz: CieXyz<IlluminantD65> = Rgb::from_hex(0xFFFFFF).into();
        assert_delta!(xyz.x, 0.95047, 0.001);
        assert_delta!(xyz.y, 1.00000, 0.001);
        assert_delta!(xyz.z, 1.08883, 0.001);
    }

    #[test]
    fn rgb_to_ciexyz_ff0000() {
        let xyz: CieXyz<IlluminantD65> = Rgb::from_hex(0xFF0000).into();
        assert_delta!(xyz.x, 0.41246, 0.001);
        assert_delta!(xyz.y, 0.21267, 0.001);
        assert_delta!(xyz.z, 0.01933, 0.001);
    }

    #[test]
    fn rgb_to_ciexyz_00ff00() {
        let xyz: CieXyz<IlluminantD65> = Rgb::from_hex(0x00FF00).into();
        assert_delta!(xyz.x, 0.35758, 0.001);
        assert_delta!(xyz.y, 0.71515, 0.001);
        assert_delta!(xyz.z, 0.11919, 0.001);
    }

    #[test]
    fn rgb_to_ciexyz_0000ff() {
        let xyz: CieXyz<IlluminantD65> = Rgb::from_hex(0x0000FF).into();
        assert_delta!(xyz.x, 0.18044, 0.001);
        assert_delta!(xyz.y, 0.07217, 0.001);
        assert_delta!(xyz.z, 0.95030, 0.001);
    }

    #[test]
    fn rgb_to_ciexyz_ea38b9() {
        let xyz: CieXyz<IlluminantD65> = Rgb::from_hex(0xEA38B9).into();
        assert_delta!(xyz.x, 0.44104, 0.001);
        assert_delta!(xyz.y, 0.23828, 0.001);
        assert_delta!(xyz.z, 0.48166, 0.001);
    }

    #[test]
    fn ciexyz_to_rgb_000000() {
        let rgb: Rgb = CieXyz::<IlluminantD65>::new(0.0, 0.0, 0.0).into();
        assert_eq!(rgb, Rgb::from_hex(0x000000));
    }

    #[test]
    fn ciexyz_to_rgb_ffffff() {
        let rgb: Rgb = CieXyz::<IlluminantD65>::new(0.95047, 1.00000, 1.08883).into();
        assert_eq!(rgb, Rgb::from_hex(0xFFFFFF));
    }

    #[test]
    fn ciexyz_to_rgb_ff0000() {
        let rgb: Rgb = CieXyz::<IlluminantD65>::new(0.41246, 0.21267, 0.01933).into();
        assert_eq!(rgb, Rgb::from_hex(0xFF0000));
    }

    #[test]
    fn ciexyz_to_rgb_00ff00() {
        let rgb: Rgb = CieXyz::<IlluminantD65>::new(0.35758, 0.71515, 0.11919).into();
        assert_eq!(rgb, Rgb::from_hex(0x00FF00));
    }

    #[test]
    fn ciexyz_to_rgb_0000ff() {
        let rgb: Rgb = CieXyz::<IlluminantD65>::new(0.18044, 0.07217, 0.95030).into();
        assert_eq!(rgb, Rgb::from_hex(0x0000FF));
    }

    #[test]
    fn ciexyz_to_rgb_ea38b9() {
        let rgb: Rgb = CieXyz::<IlluminantD65>::new(0.44104, 0.23828, 0.48166).into();
        assert_eq!(rgb, Rgb::from_hex(0xEA38B9));
    }

    #[test]
    fn rgb_to_ciexyz_to_rgb_ea38b9() {
        let original = Rgb::from_hex(0xEA38B9);
        let round_trip: Rgb = CieXyz::<IlluminantD65>::from(original.clone()).into();
        assert_eq!(original, round_trip);
    }

    #[test]
    fn ciexyz_to_cieluv() {
        let luv: CieLuv<IlluminantD65> = CieXyz::new(0.44104, 0.23828, 0.48166).into();
        assert_delta!(luv.l, 55.915, 0.001);
        assert_delta!(luv.u, 91.047, 0.001);
        assert_delta!(luv.v, -54.941, 0.001);
    }

    #[test]
    fn ciexyz_to_cieluv_to_ciexyz() {
        let original = CieXyz::<IlluminantD65>::new(0.44104, 0.23828, 0.48166);
        let round_trip: CieXyz<_> = CieLuv::from(original.clone()).into();
        assert_delta!(original.x, round_trip.x, 0.001);
        assert_delta!(original.y, round_trip.y, 0.001);
        assert_delta!(original.z, round_trip.z, 0.001);
    }
}
