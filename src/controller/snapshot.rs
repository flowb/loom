use std::collections::HashMap;

use crate::model::{
    TrackId, Track, ContainerId, MediaContainer,
    Timeline, TimelineId, EndpointId, EndpointConfig
};
use crate::tapestry::{TimePosition, Duration};

/// Snapshot of a track for UI rendering
#[derive(Debug, Clone)]
pub struct TrackSnapshot {
    pub id: TrackId,
    pub name: String,
    pub color: crate::model::Color,
    pub is_muted: bool,
    pub is_solo: bool,
    pub output_id: Option<EndpointId>,
    pub height: u32,
}

impl From<&Track> for TrackSnapshot {
    fn from(track: &Track) -> Self {
        Self {
            id: track.id,
            name: track.name.clone(),
            color: track.color,
            is_muted: track.is_muted,
            is_solo: track.is_solo,
            output_id: track.output_id,
            height: track.height,
        }
    }
}

/// Snapshot of a container for UI rendering
#[derive(Debug, Clone)]
pub struct ContainerSnapshot {
    pub id: ContainerId,
    pub position: TimePosition,
    pub length: Duration,
    pub content_type: ContainerContentType,
    pub is_looping: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerContentType {
    Pattern,
    MidiClip,
    AudioFile,
}

impl From<&MediaContainer> for ContainerSnapshot {
    fn from(container: &MediaContainer) -> Self {
        let content_type = match container.content {
            crate::model::MediaContent::Pattern(_) => ContainerContentType::Pattern,
            crate::model::MediaContent::MidiClip(_) => ContainerContentType::MidiClip,
            crate::model::MediaContent::AudioFile(_) => ContainerContentType::AudioFile,
        };

        Self {
            id: container.id,
            position: container.position,
            length: container.length,
            content_type,
            is_looping: matches!(container.playback_mode, crate::model::PlaybackMode::Loop),
        }
    }
}

/// Snapshot of a timeline for UI rendering
#[derive(Debug, Clone)]
pub struct TimelineSnapshot {
    pub id: TimelineId,
    pub name: String,
    pub tracks: Vec<TrackSnapshot>,
    pub containers: HashMap<TrackId, Vec<ContainerSnapshot>>,
    pub playback_position: Option<TimePosition>,
}

impl TimelineSnapshot {
    /// Create a snapshot from a timeline
    pub fn from_timeline(
        timeline: &Timeline,
        playback_position: Option<TimePosition>
    ) -> Self {
        let mut containers: HashMap<TrackId, Vec<ContainerSnapshot>> = HashMap::new();

        // Group containers by track
        for track in &timeline.tracks {
            let track_id = track.id;
            let mut track_containers = Vec::new();

            if let Some(track_map) = timeline.track_containers.get(&track_id) {
                for (_, container_id) in track_map {
                    if let Some(container) = timeline.containers.get(container_id) {
                        track_containers.push(ContainerSnapshot::from(container));
                    }
                }
            }

            containers.insert(track_id, track_containers);
        }

        Self {
            id: timeline.id,
            name: timeline.name.clone(),
            tracks: timeline.tracks.iter().map(TrackSnapshot::from).collect(),
            containers,
            playback_position,
        }
    }
}

/// Snapshot of an endpoint for UI rendering
#[derive(Debug, Clone)]
pub struct EndpointSnapshot {
    pub id: EndpointId,
    pub name: String,
    pub device_id: String,
    pub endpoint_type: crate::model::EndpointType,
    pub enabled: bool,
}

impl From<&EndpointConfig> for EndpointSnapshot {
    fn from(config: &EndpointConfig) -> Self {
        Self {
            id: config.id,
            name: config.name.clone(),
            device_id: config.device_id.clone(),
            endpoint_type: config.endpoint_type,
            enabled: config.enabled,
        }
    }
}

/// Complete project snapshot for UI rendering
#[derive(Debug, Clone)]
pub struct ProjectSnapshot {
    pub name: String,
    pub active_timeline: Option<TimelineSnapshot>,
    pub endpoints: Vec<EndpointSnapshot>,
}