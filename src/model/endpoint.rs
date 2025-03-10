use uuid::Uuid;
use std::fmt;

/// Unique identifier for an output endpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointId(Uuid);

impl EndpointId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for EndpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type of endpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointType {
    Midi,
    Audio,
    Vst,
}

/// Configuration for an output endpoint
#[derive(Debug, Clone)]
pub struct EndpointConfig {
    /// Unique identifier
    pub id: EndpointId,

    /// User-visible name
    pub name: String,

    /// Type of endpoint
    pub endpoint_type: EndpointType,

    /// System device identifier (like MIDI port name)
    pub device_id: String,

    /// Is this endpoint enabled?
    pub enabled: bool,

    /// Additional configuration parameters
    pub parameters: EndpointParameters,
}

/// Type-specific parameters for endpoints
#[derive(Debug, Clone)]
pub enum EndpointParameters {
    Midi {
        /// MIDI channel (0-15)
        channel: Option<u8>,
    },

    Audio {
        /// Volume (0.0 - 1.0)
        volume: f32,
        /// Pan (-1.0 to 1.0)
        pan: f32,
    },

    Vst {
        /// Path to VST plugin
        plugin_path: String,
        /// Plugin-specific state data
        plugin_state: Option<Vec<u8>>,
    },
}

impl EndpointConfig {
    /// Create a new MIDI endpoint configuration
    pub fn new_midi(name: String, device_id: String) -> Self {
        Self {
            id: EndpointId::new(),
            name,
            endpoint_type: EndpointType::Midi,
            device_id,
            enabled: true,
            parameters: EndpointParameters::Midi {
                channel: None,  // All channels
            },
        }
    }

    /// Create a new audio endpoint configuration
    pub fn new_audio(name: String, device_id: String) -> Self {
        Self {
            id: EndpointId::new(),
            name,
            endpoint_type: EndpointType::Audio,
            device_id,
            enabled: true,
            parameters: EndpointParameters::Audio {
                volume: 1.0,
                pan: 0.0,
            },
        }
    }

    /// Create a new VST endpoint configuration
    pub fn new_vst(name: String, plugin_path: String) -> Self {
        Self {
            id: EndpointId::new(),
            name,
            endpoint_type: EndpointType::Vst,
            device_id: plugin_path.clone(),  // Use plugin path as device ID
            enabled: true,
            parameters: EndpointParameters::Vst {
                plugin_path,
                plugin_state: None,
            },
        }
    }
}