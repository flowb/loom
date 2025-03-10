use std::collections::BTreeMap;
use crate::tapestry::position::TimePosition;
use crate::tapestry::tempo::{Tempo, TimeSignature};

/// Maps between different time domains: ticks, beats, bars, etc.
#[derive(Debug, Clone)]
pub struct TempoMap {
    /// Internal reference sample rate used for tick calculations
    reference_sample_rate: u32,
    /// Current playback sample rate
    playback_sample_rate: u32,
    /// Tempo changes keyed by position
    tempo_changes: BTreeMap<TimePosition, Tempo>,
    /// Time signature changes keyed by position
    time_signature_changes: BTreeMap<TimePosition, TimeSignature>,
}

impl TempoMap {
    /// Create a new TempoMap with default tempo and time signature
    pub fn new(reference_sample_rate: u32, playback_sample_rate: u32) -> Self {
        let mut tempo_changes = BTreeMap::new();
        let mut time_signature_changes = BTreeMap::new();
        
        // Default to 120 BPM, 4/4 time at position zero
        tempo_changes.insert(TimePosition::zero(), Tempo::new(120.0));
        time_signature_changes.insert(TimePosition::zero(), TimeSignature::new(4, 4));
        
        Self {
            reference_sample_rate,
            playback_sample_rate,
            tempo_changes,
            time_signature_changes,
        }
    }
    
    /// Set the playback sample rate without changing time positions
    pub fn set_playback_sample_rate(&mut self, new_playback_sample_rate: u32) {
        self.playback_sample_rate = new_playback_sample_rate;
    }
    
    /// Add a tempo change at the specified position
    pub fn add_tempo_change(&mut self, position: TimePosition, tempo: Tempo) {
        self.tempo_changes.insert(position, tempo);
    }
    
    /// Add a time signature change at the specified position
    pub fn add_time_signature_change(&mut self, position: TimePosition, time_signature: TimeSignature) {
        self.time_signature_changes.insert(position, time_signature);
    }
    
    /// Get the tempo at a specific position
    pub fn tempo_at(&self, position: &TimePosition) -> Tempo {
        // Find the last tempo change before or at the given position
        match self.tempo_changes.range(..=position).next_back() {
            Some((_, tempo)) => *tempo,
            None => panic!("No tempo defined"), // Should never happen as we always have a default
        }
    }
    
    /// Get the time signature at a specific position
    pub fn time_signature_at(&self, position: &TimePosition) -> TimeSignature {
        // Find the last time signature change before or at the given position
        match self.time_signature_changes.range(..=position).next_back() {
            Some((_, time_sig)) => *time_sig,
            None => panic!("No time signature defined"), // Should never happen as we always have a default
        }
    }
    
    /// Convert from internal ticks to actual playback samples
    pub fn ticks_to_playback_samples(&self, position: &TimePosition) -> u64 {
        (position.position_ticks as f64 * self.playback_sample_rate as f64 / 
         self.reference_sample_rate as f64).round() as u64
    }
    
    /// Convert from actual playback samples to internal ticks
    pub fn playback_samples_to_ticks(&self, samples: u64) -> TimePosition {
        TimePosition {
            position_ticks: (samples as f64 * self.reference_sample_rate as f64 / 
                            self.playback_sample_rate as f64).round() as u64
        }
    }
    
    /// Convert time position to beats
    pub fn position_to_beats(&self, position: &TimePosition) -> f64 {
        // We need to process each tempo segment separately
        let mut result = 0.0;
        let mut last_position = TimePosition::zero();
        let mut last_tempo = self.tempo_at(&last_position);
        
        // Process each tempo change segment
        for (change_position, tempo) in self.tempo_changes.range(&TimePosition::zero()..=position) {
            if change_position > &last_position {
                // Calculate beats in the segment with consistent tempo
                let segment_duration_secs = (change_position.position_ticks - last_position.position_ticks) as f64 / 
                                           self.reference_sample_rate as f64;
                result += segment_duration_secs / last_tempo.beat_duration_secs();
                
                // Update for next segment
                last_position = *change_position;
                last_tempo = *tempo;
            }
        }
        
        // Process final segment up to the target position
        if position > &last_position {
            let final_duration_secs = (position.position_ticks - last_position.position_ticks) as f64 / 
                                     self.reference_sample_rate as f64;
            result += final_duration_secs / last_tempo.beat_duration_secs();
        }
        
        result
    }
    
    /// Convert beats to time position
    pub fn beats_to_position(&self, beats: f64) -> TimePosition {
        let mut remaining_beats = beats;
        let mut current_position = TimePosition::zero();
        let mut current_tempo = self.tempo_at(&current_position);
        let mut iter = self.tempo_changes.iter();
        
        // Skip the first entry which is at position zero
        let _ = iter.next();
        
        // Process each tempo segment
        for (change_position, next_tempo) in iter {
            // Calculate beats until the next tempo change
            let current_beats = self.position_to_beats(change_position);
            
            if remaining_beats <= current_beats {
                // Target is within this tempo segment
                break;
            }
            
            // Move to the tempo change position and update tempo
            current_position = *change_position;
            current_tempo = *next_tempo;
            remaining_beats -= current_beats;
        }
        
        // Calculate final position within the current tempo segment
        let segment_duration_secs = remaining_beats * current_tempo.beat_duration_secs();
        let additional_ticks = (segment_duration_secs * self.reference_sample_rate as f64).round() as u64;
        
        TimePosition {
            position_ticks: current_position.position_ticks + additional_ticks
        }
    }
    
    /// Convert time position to bars and beats
    pub fn position_to_bars_and_beats(&self, position: &TimePosition) -> (u32, f64) {
        let total_beats = self.position_to_beats(position);
        let mut remaining_beats = total_beats;
        let mut bars = 0;
        
        // Accumulate bars based on time signature changes
        let mut last_sig_position = TimePosition::zero();
        let mut last_sig = self.time_signature_at(&last_sig_position);
        
        for (sig_position, time_sig) in self.time_signature_changes.range(..=position) {
            let beats_at_sig_change = self.position_to_beats(sig_position);
            
            if beats_at_sig_change > 0.0 {
                // Process complete bars in the current time signature
                let beats_in_segment = beats_at_sig_change - self.position_to_beats(&last_sig_position);
                let bars_in_segment = (beats_in_segment / last_sig.beats_per_bar()).floor();
                
                bars += bars_in_segment as u32;
                remaining_beats -= bars_in_segment * last_sig.beats_per_bar();
                
                // Update for next segment
                last_sig_position = *sig_position;
                last_sig = *time_sig;
            }
        }
        
        // Process the final segment
        let final_bars = (remaining_beats / last_sig.beats_per_bar()).floor() as u32;
        let beat_in_bar = remaining_beats - (final_bars as f64 * last_sig.beats_per_bar());
        
        (bars + final_bars, beat_in_bar)
    }
    
    /// Get the reference sample rate
    pub fn reference_sample_rate(&self) -> u32 {
        self.reference_sample_rate
    }
    
    /// Get the playback sample rate
    pub fn playback_sample_rate(&self) -> u32 {
        self.playback_sample_rate
    }
}