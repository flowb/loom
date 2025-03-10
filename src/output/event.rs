// src/output/event.rs
use std::sync::Arc;

/// Types of output events that can be sent to endpoints
#[derive(Debug, Clone)]
pub enum OutputEventType {
    // MIDI events
    MidiNoteOn { channel: u8, note: u8, velocity: u8 },
    MidiNoteOff { channel: u8, note: u8 },
    MidiControlChange { channel: u8, controller: u8, value: u8 },
    MidiProgramChange { channel: u8, program: u8 },
    MidiPitchBend { channel: u8, value: i16 },  // -8192 to 8191
    MidiAftertouch { channel: u8, pressure: u8 },
    MidiPolyAftertouch { channel: u8, note: u8, pressure: u8 },

    // Audio events
    AudioBuffer { data: Arc<Vec<f32>>, channels: u8, frames: usize },

    // VST events
    VstParameter { parameter_id: u32, value: f32 },

    // Clock-related events
    SyncPulse,

    // System events
    EndOfTrack,
}

/// An event to be sent to an output endpoint
#[derive(Debug, Clone)]
pub struct OutputEvent {
    /// The type of event
    pub event_type: OutputEventType,

    /// Target endpoint ID (if specific) or None for broadcast
    pub target: Option<crate::model::EndpointId>,
}

impl OutputEvent {
    pub fn new(event_type: OutputEventType, target: Option<crate::model::EndpointId>) -> Self {
        Self { event_type, target }
    }

    pub fn midi_note_on(channel: u8, note: u8, velocity: u8, target: Option<crate::model::EndpointId>) -> Self {
        Self::new(OutputEventType::MidiNoteOn { channel, note, velocity }, target)
    }

    pub fn midi_note_off(channel: u8, note: u8, target: Option<crate::model::EndpointId>) -> Self {
        Self::new(OutputEventType::MidiNoteOff { channel, note }, target)
    }

    pub fn midi_cc(channel: u8, controller: u8, value: u8, target: Option<crate::model::EndpointId>) -> Self {
        Self::new(OutputEventType::MidiControlChange { channel, controller, value }, target)
    }

    pub fn is_midi(&self) -> bool {
        matches!(self.event_type,
            OutputEventType::MidiNoteOn { .. } |
            OutputEventType::MidiNoteOff { .. } |
            OutputEventType::MidiControlChange { .. } |
            OutputEventType::MidiProgramChange { .. } |
            OutputEventType::MidiPitchBend { .. } |
            OutputEventType::MidiAftertouch { .. } |
            OutputEventType::MidiPolyAftertouch { .. }
        )
    }

    pub fn is_audio(&self) -> bool {
        matches!(self.event_type, OutputEventType::AudioBuffer { .. })
    }

    pub fn is_vst(&self) -> bool {
        matches!(self.event_type, OutputEventType::VstParameter { .. })
    }
}