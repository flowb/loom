use std::cmp::Ordering;
use std::ops::{Add, Sub, AddAssign, SubAssign};

/// Represents a precise position in time, independent of sample rate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimePosition {
    /// Internal representation as ticks (at reference sample rate)
    pub position_ticks: u64,
}

impl TimePosition {
    /// Create a new TimePosition at the specified tick position
    pub fn new(position_ticks: u64) -> Self {
        Self { position_ticks }
    }
    
    /// Create a TimePosition representing time zero
    pub fn zero() -> Self {
        Self { position_ticks: 0 }
    }
    
    /// Convert from seconds to TimePosition using the reference sample rate
    pub fn from_seconds(seconds: f64, reference_sample_rate: u32) -> Self {
        let ticks = (seconds * reference_sample_rate as f64).round() as u64;
        Self { position_ticks: ticks }
    }
    
    /// Convert this position to seconds using the reference sample rate
    pub fn to_seconds(&self, reference_sample_rate: u32) -> f64 {
        self.position_ticks as f64 / reference_sample_rate as f64
    }
}

// Make TimePosition comparable for use in BTreeMap
impl Ord for TimePosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position_ticks.cmp(&other.position_ticks)
    }
}

impl PartialOrd for TimePosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Allow addition of TimePositions
impl Add for TimePosition {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            position_ticks: self.position_ticks + other.position_ticks,
        }
    }
}

// Allow subtraction of TimePositions
impl Sub for TimePosition {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            position_ticks: self.position_ticks.saturating_sub(other.position_ticks),
        }
    }
}

// Allow in-place addition
impl AddAssign for TimePosition {
    fn add_assign(&mut self, other: Self) {
        self.position_ticks += other.position_ticks;
    }
}

// Allow in-place subtraction
impl SubAssign for TimePosition {
    fn sub_assign(&mut self, other: Self) {
        self.position_ticks = self.position_ticks.saturating_sub(other.position_ticks);
    }
}