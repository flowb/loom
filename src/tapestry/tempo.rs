/// Represents musical tempo
#[derive(Debug, Clone, Copy)]
pub struct Tempo {
    /// Beats per minute
    pub bpm: f64,
}

impl Tempo {
    pub fn new(bpm: f64) -> Self {
        Self { bpm }
    }
    
    /// Get duration of a beat in seconds
    pub fn beat_duration_secs(&self) -> f64 {
        60.0 / self.bpm
    }
}

/// Represents musical time signature
#[derive(Debug, Clone, Copy)]
pub struct TimeSignature {
    /// Top number (how many beats per bar)
    pub numerator: u8,
    /// Bottom number (what note value gets one beat)
    pub denominator: u8,
}

impl TimeSignature {
    pub fn new(numerator: u8, denominator: u8) -> Self {
        Self { numerator, denominator }
    }
    
    /// Get number of beats in a bar
    pub fn beats_per_bar(&self) -> f64 {
        self.numerator as f64
    }
}