use std::sync::Arc;
use uuid::Uuid;
use crate::model::endpoint::EndpointId;

/// Unique identifier for a track
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrackId(Uuid);

impl TrackId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Represents a track color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Defines the type of a track
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackType {
    Midi,
    Audio,
    Instrument,
    Automation,
}

/// Represents a track in the timeline
#[derive(Debug, Clone)]
pub struct Track {
    /// Unique identifier
    pub id: TrackId,

    /// User-visible name
    pub name: String,

    /// Type of track
    pub track_type: TrackType,

    /// Connected output endpoint (if any)
    pub output_id: Option<EndpointId>,

    /// Display color
    pub color: Color,

    /// Mute state
    pub is_muted: bool,

    /// Solo state
    pub is_solo: bool,

    /// Track height in the UI (in pixels)
    pub height: u32,
}

impl Track {
    pub fn new(name: String, track_type: TrackType) -> Self {
        Self {
            id: TrackId::new(),
            name,
            track_type,
            output_id: None,
            color: Color::new(100, 100, 200),  // Default light blue
            is_muted: false,
            is_solo: false,
            height: 100,  // Default height
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_output(mut self, output_id: EndpointId) -> Self {
        self.output_id = Some(output_id);
        self
    }
}