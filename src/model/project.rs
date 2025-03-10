use std::collections::HashMap;
use uuid::Uuid;
use crate::model::timeline::{Timeline, TimelineId};
use crate::model::endpoint::{EndpointConfig, EndpointId};
use crate::tapestry::{TempoMap, Tempo, TimePosition, TimeSignature};

/// Unique identifier for a project
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Project settings
#[derive(Debug, Clone)]
pub struct ProjectSettings {
    /// Reference sample rate used for internal time calculations
    pub reference_sample_rate: u32,

    /// Playback sample rate
    pub playback_sample_rate: u32,

    /// Default MIDI output device (if any)
    pub default_midi_output: Option<EndpointId>,

    /// Default MIDI input device (if any)
    pub default_midi_input: Option<String>,

    /// Default MIDI channel for new events
    pub default_midi_channel: u8,

    /// Default velocity for new MIDI notes
    pub default_velocity: u8,

    /// Default note duration in beats
    pub default_note_duration: f64,

    /// Snap to grid enabled by default
    pub snap_to_grid: bool,

    /// Default grid size in beats
    pub grid_size: f64,

    /// Auto-quantize MIDI input
    pub auto_quantize: bool,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            reference_sample_rate: 44100,
            playback_sample_rate: 44100,
            default_midi_output: None,
            default_midi_input: None,
            default_midi_channel: 0,
            default_velocity: 100,
            default_note_duration: 0.25,  // 16th note by default
            snap_to_grid: true,
            grid_size: 0.25,              // 16th note grid by default
            auto_quantize: true,
        }
    }
}

/// Represents the main project container
#[derive(Debug, Clone)]
pub struct Project {
    /// Unique identifier
    pub id: ProjectId,

    /// User-visible name
    pub name: String,

    /// Project settings
    pub settings: ProjectSettings,

    /// Project version/modification counter
    pub version: u32,

    /// Tempo map for timing calculations
    pub tempo_map: TempoMap,

    /// Timelines in this project
    pub timelines: HashMap<TimelineId, Timeline>,

    /// Configured output endpoints
    pub endpoints: HashMap<EndpointId, EndpointConfig>,

    /// The currently active timeline
    pub active_timeline_id: Option<TimelineId>,
}

impl Project {
    pub fn new(name: String) -> Self {
        // Create a default tempo map
        let mut tempo_map = TempoMap::new(44100, 44100);
        tempo_map.add_tempo_change(TimePosition::zero(), Tempo::new(120.0));
        tempo_map.add_time_signature_change(TimePosition::zero(), TimeSignature::new(4, 4));

        // Create a default timeline
        let timeline = Timeline::new("Main".to_string());
        let timeline_id = timeline.id;

        let mut timelines = HashMap::new();
        timelines.insert(timeline_id, timeline);

        Self {
            id: ProjectId::new(),
            name,
            settings: ProjectSettings::default(),
            version: 1,
            tempo_map,
            timelines,
            endpoints: HashMap::new(),
            active_timeline_id: Some(timeline_id),
        }
    }

    /// Get the active timeline
    pub fn active_timeline(&self) -> Option<&Timeline> {
        self.active_timeline_id.and_then(|id| self.timelines.get(&id))
    }

    /// Get a mutable reference to the active timeline
    pub fn active_timeline_mut(&mut self) -> Option<&mut Timeline> {
        let id = self.active_timeline_id?;
        self.timelines.get_mut(&id)
    }

    /// Add a new timeline to the project
    pub fn add_timeline(&mut self, name: String) -> TimelineId {
        let timeline = Timeline::new(name);
        let id = timeline.id;
        self.timelines.insert(id, timeline);
        self.version += 1;
        id
    }

    /// Set the active timeline
    pub fn set_active_timeline(&mut self, id: TimelineId) -> Result<(), &'static str> {
        if self.timelines.contains_key(&id) {
            self.active_timeline_id = Some(id);
            self.version += 1;
            Ok(())
        } else {
            Err("Timeline not found")
        }
    }

    /// Add an output endpoint configuration
    pub fn add_endpoint(&mut self, config: EndpointConfig) -> EndpointId {
        let id = config.id;
        self.endpoints.insert(id, config);
        self.version += 1;
        id
    }

    /// Get a reference to an endpoint by ID
    pub fn endpoint(&self, id: EndpointId) -> Option<&EndpointConfig> {
        self.endpoints.get(&id)
    }

    /// Get a mutable reference to an endpoint by ID
    pub fn endpoint_mut(&mut self, id: EndpointId) -> Option<&mut EndpointConfig> {
        self.endpoints.get_mut(&id)
    }
}