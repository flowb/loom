/// Represents standard musical note durations
#[derive(Debug, Clone, Copy)]
pub struct NoteValue(pub f64);

impl NoteValue {
    // Standard note values as a fraction of a whole note
    pub const WHOLE: NoteValue = NoteValue(4.0);
    pub const HALF: NoteValue = NoteValue(2.0);
    pub const QUARTER: NoteValue = NoteValue(1.0);
    pub const EIGHTH: NoteValue = NoteValue(0.5);
    pub const SIXTEENTH: NoteValue = NoteValue(0.25);
    pub const THIRTY_SECOND: NoteValue = NoteValue(0.125);
    pub const SIXTY_FOURTH: NoteValue = NoteValue(0.0625);

    // Dotted versions (1.5x the duration)
    pub const DOTTED_HALF: NoteValue = NoteValue(3.0);
    pub const DOTTED_QUARTER: NoteValue = NoteValue(1.5);
    pub const DOTTED_EIGHTH: NoteValue = NoteValue(0.75);
    pub const DOTTED_SIXTEENTH: NoteValue = NoteValue(0.375);

    // Triplet versions (2/3 the duration)
    pub const TRIPLET_HALF: NoteValue = NoteValue(4.0/3.0);
    pub const TRIPLET_QUARTER: NoteValue = NoteValue(2.0/3.0);
    pub const TRIPLET_EIGHTH: NoteValue = NoteValue(1.0/3.0);
    pub const TRIPLET_SIXTEENTH: NoteValue = NoteValue(0.5/3.0);

    /// Convert to beats (quarter notes)
    pub fn to_beats(&self) -> f64 {
        self.0
    }

    /// Create a note value from a custom beat duration
    pub fn from_beats(beats: f64) -> Self {
        Self(beats)
    }
}