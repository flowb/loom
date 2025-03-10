use uuid::Uuid;
use crate::tapestry::{TimePosition, Duration};

/// Unique identifier for a media container
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContainerId(Uuid);

impl ContainerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatternId(Uuid);

impl PatternId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for a MIDI clip
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiClipId(Uuid);

impl MidiClipId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Unique identifier for an audio file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioFileId(Uuid);

impl AudioFileId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Defines how a container's content is played back
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackMode {
    /// Play once at normal length
    Normal,

    /// Repeat until container length is reached
    Loop,

    /// Play once regardless of container length
    OneShot,

    /// Alternate forward/backward playback
    PingPong,
}

/// Represents the content of a media container
#[derive(Debug, Clone)]
pub enum MediaContent {
    /// Tracker-style pattern
    Pattern(PatternId),

    /// MIDI clip (direct note data)
    MidiClip(MidiClipId),

    /// Audio file
    AudioFile(AudioFileId),

    // Future content types can be added here
}

/// A container that holds media content on the timeline
#[derive(Debug, Clone)]
pub struct MediaContainer {
    /// Unique identifier
    pub id: ContainerId,

    /// Position on the timeline
    pub position: TimePosition,

    /// Length on the timeline (can differ from content's intrinsic length)
    pub length: Duration,

    /// How the content is played back
    pub playback_mode: PlaybackMode,

    /// Number of times to loop (None = infinite within container length)
    pub loop_count: Option<u32>,

    /// Offset from the start of the content
    pub start_offset: Duration,

    /// Offset from the end of the content
    pub end_offset: Duration,

    /// Playback speed (1.0 = normal)
    pub time_scale: f64,

    /// The actual content in the container
    pub content: MediaContent,
}

impl MediaContainer {
    pub fn new(position: TimePosition, content: MediaContent) -> Self {
        // Default container with reasonable settings
        Self {
            id: ContainerId::new(),
            position,
            length: Duration::from_beats(4.0),  // Default 4 beats
            playback_mode: PlaybackMode::Normal,
            loop_count: None,
            start_offset: Duration::zero(),
            end_offset: Duration::zero(),
            time_scale: 1.0,
            content,
        }
    }

    pub fn with_length(mut self, length: Duration) -> Self {
        self.length = length;
        self
    }

    pub fn with_loop(mut self, count: Option<u32>) -> Self {
        self.playback_mode = PlaybackMode::Loop;
        self.loop_count = count;
        self
    }

    pub fn with_crop(mut self, start: Duration, end: Duration) -> Self {
        self.start_offset = start;
        self.end_offset = end;
        self
    }

    pub fn with_time_scale(mut self, scale: f64) -> Self {
        self.time_scale = scale;
        self
    }
}