// src/tapestry/duration.rs
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

/// Represents a duration of time, independent of sample rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Duration {
    /// Internal representation as ticks (at reference sample rate)
    pub(crate) ticks: u64,
}

impl Duration {
    /// Create a new Duration with the specified tick count
    pub fn new(ticks: u64) -> Self {
        Self { ticks }
    }

    /// Create a zero duration
    pub fn zero() -> Self {
        Self { ticks: 0 }
    }

    /// Convert from seconds to Duration using the reference sample rate
    pub fn from_seconds(seconds: f64, reference_sample_rate: u32) -> Self {
        let ticks = (seconds * reference_sample_rate as f64).round() as u64;
        Self { ticks }
    }

    /// Convert from beats to Duration using a tempo
    pub fn from_beats(beats: f64) -> Self {
        // For now, we'll use a simple conversion where 1 beat = 22050 ticks (0.5 sec at 44.1kHz)
        // In a real implementation, this would use the tempo map
        let ticks = (beats * 22050.0).round() as u64;
        Self { ticks }
    }

    /// Convert this duration to seconds using the reference sample rate
    pub fn to_seconds(&self, reference_sample_rate: u32) -> f64 {
        self.ticks as f64 / reference_sample_rate as f64
    }

    /// Get the raw tick count
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}

// Implement addition of Durations
impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ticks: self.ticks + other.ticks,
        }
    }
}

// Implement subtraction of Durations
impl Sub for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ticks: self.ticks.saturating_sub(other.ticks),
        }
    }
}

// Allow in-place addition
impl AddAssign for Duration {
    fn add_assign(&mut self, other: Self) {
        self.ticks += other.ticks;
    }
}

// Allow in-place subtraction
impl SubAssign for Duration {
    fn sub_assign(&mut self, other: Self) {
        self.ticks = self.ticks.saturating_sub(other.ticks);
    }
}

// Multiplication by scalar
impl Mul<f64> for Duration {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            ticks: (self.ticks as f64 * scalar).round() as u64,
        }
    }
}

// Division by scalar
impl Div<f64> for Duration {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            ticks: (self.ticks as f64 / scalar).round() as u64,
        }
    }
}