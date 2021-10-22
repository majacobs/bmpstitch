use crate::color::Rgb;
use std::collections::vec_deque::VecDeque;
use std::ops::{Add, AddAssign, Mul, Sub};

const MINIMIZED_AVERAGE_ERROR_WEIGHTS: [[f32; 5]; 3] = [
    [0.0, 0.0, 0.0, 7.0 / 48.0, 5.0 / 48.0],
    [3.0 / 48.0, 5.0 / 48.0, 7.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0],
    [1.0 / 48.0, 3.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0, 1.0 / 48.0],
];

pub struct Ditherer {
    width: usize,
    error_rows: VecDeque<Vec<QuantizationError>>,
    index_in_row: usize,
}

impl Ditherer {
    pub fn new(width: usize) -> Self {
        let mut error_rows = VecDeque::new();
        for _ in MINIMIZED_AVERAGE_ERROR_WEIGHTS {
            error_rows.push_front(vec![QuantizationError::zero(); width]);
        }

        Self {
            width,
            error_rows,
            index_in_row: 0,
        }
    }

    pub fn next(&mut self) {
        self.index_in_row += 1;
        if self.index_in_row == self.width {
            self.index_in_row = 0;

            self.error_rows.pop_front();
            self.error_rows
                .push_back(vec![QuantizationError::zero(); self.width]);
        }
    }

    pub fn apply_error(&self, color: &Rgb) -> Rgb {
        *color + self.error_rows[0][self.index_in_row]
    }

    pub fn record_error(&mut self, original: &Rgb, closest: &Rgb) {
        let error = *original - *closest;

        let weight_index_offset = (MINIMIZED_AVERAGE_ERROR_WEIGHTS[0].len() - 1) / 2;
        for (r_index, row) in MINIMIZED_AVERAGE_ERROR_WEIGHTS.iter().enumerate() {
            for (c_index, col) in row.iter().enumerate() {
                let offset = match c_index.checked_sub(weight_index_offset) {
                    Some(i) => i,
                    None => continue,
                };
                if let Some(elem) = self.error_rows[r_index].get_mut(self.index_in_row + offset) {
                    *elem += error * *col;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct QuantizationError {
    r: i16,
    g: i16,
    b: i16,
}

impl QuantizationError {
    fn zero() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl Add<QuantizationError> for Rgb {
    type Output = Self;

    fn add(self, rhs: QuantizationError) -> Self {
        fn clamp(value: i16) -> u8 {
            value.min(u8::MAX as i16).max(u8::MIN as i16) as u8
        }

        Self {
            r: clamp(self.r as i16 + rhs.r),
            g: clamp(self.g as i16 + rhs.g),
            b: clamp(self.b as i16 + rhs.b),
        }
    }
}

impl Sub for Rgb {
    type Output = QuantizationError;

    fn sub(self, rhs: Self) -> QuantizationError {
        QuantizationError {
            r: self.r as i16 - rhs.r as i16,
            g: self.g as i16 - rhs.g as i16,
            b: self.b as i16 - rhs.b as i16,
        }
    }
}

impl AddAssign for QuantizationError {
    fn add_assign(&mut self, rhs: QuantizationError) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f32> for QuantizationError {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: (self.r as f32 * rhs) as i16,
            g: (self.g as f32 * rhs) as i16,
            b: (self.b as f32 * rhs) as i16,
        }
    }
}
