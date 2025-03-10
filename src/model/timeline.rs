use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;
use crate::model::track::{Track, TrackId};
use crate::model::container::{MediaContainer, ContainerId};
use crate::tapestry::TimePosition;

/// Unique identifier for a timeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimelineId(Uuid);

impl TimelineId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Represents a timeline with tracks and containers
#[derive(Debug, Clone)]
pub struct Timeline {
    /// Unique identifier
    pub id: TimelineId,

    /// User-visible name
    pub name: String,

    /// Tracks in this timeline
    pub tracks: Vec<Track>,

    /// All containers in this timeline
    /// Using HashMap as container IDs have no natural ordering
    pub containers: HashMap<ContainerId, MediaContainer>,

    /// Maps track IDs to the containers on that track
    /// Containers within a track are ordered by position for efficient lookup
    pub track_containers: HashMap<TrackId, BTreeMap<TimePosition, ContainerId>>,
}

impl Timeline {
    pub fn new(name: String) -> Self {
        Self {
            id: TimelineId::new(),
            name,
            tracks: Vec::new(),
            containers: HashMap::new(),
            track_containers: HashMap::new(),
        }
    }

    /// Add a track to the timeline
    pub fn add_track(&mut self, track: Track) -> TrackId {
        let id = track.id;
        self.tracks.push(track);
        self.track_containers.insert(id, BTreeMap::new());
        id
    }

    /// Get a track by ID
    pub fn track(&self, id: TrackId) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    /// Get a mutable reference to a track by ID
    pub fn track_mut(&mut self, id: TrackId) -> Option<&mut Track> {
        self.tracks.iter_mut().find(|t| t.id == id)
    }

    /// Add a container to a track
    pub fn add_container(&mut self, track_id: TrackId, container: MediaContainer) -> ContainerId {
        let id = container.id;
        let position = container.position;

        // Store the container
        self.containers.insert(id, container);

        // Add to the track's container map
        if let Some(track_map) = self.track_containers.get_mut(&track_id) {
            track_map.insert(position, id);
        }

        id
    }

    /// Get a container by ID
    pub fn container(&self, id: ContainerId) -> Option<&MediaContainer> {
        self.containers.get(&id)
    }

    /// Get a mutable reference to a container by ID
    pub fn container_mut(&mut self, id: ContainerId) -> Option<&mut MediaContainer> {
        self.containers.get_mut(&id)
    }

    /// Move a container to a new position
    pub fn move_container(&mut self, id: ContainerId, new_position: TimePosition) -> bool {
        // Find which track contains this container
        let mut track_id = None;
        let mut old_position = None;

        for (tid, containers) in &self.track_containers {
            for (pos, cid) in containers {
                if *cid == id {
                    track_id = Some(*tid);
                    old_position = Some(*pos);
                    break;
                }
            }
            if track_id.is_some() {
                break;
            }
        }

        // If we found the container, update its position
        if let (Some(tid), Some(old_pos)) = (track_id, old_position) {
            // Update the container itself
            if let Some(container) = self.containers.get_mut(&id) {
                container.position = new_position;
            } else {
                return false;
            }

            // Update the track's container map
            if let Some(track_map) = self.track_containers.get_mut(&tid) {
                track_map.remove(&old_pos);
                track_map.insert(new_position, id);
                return true;
            }
        }

        false
    }

    /// Get all containers in a time range for a specific track
    pub fn track_containers_in_range(
        &self,
        track_id: TrackId,
        start: &TimePosition,
        end: &TimePosition
    ) -> Vec<&MediaContainer> {
        let mut result = Vec::new();

        if let Some(track_map) = self.track_containers.get(&track_id) {
            // Use BTreeMap range query to find containers that start in the range
            for (_, container_id) in track_map.range(start..end) {
                if let Some(container) = self.containers.get(container_id) {
                    result.push(container);
                }
            }

            // Also find containers that start before but extend into the range
            for (_, container_id) in track_map.range(..start) {
                if let Some(container) = self.containers.get(container_id) {
                    // Calculate end position of the container
                    let container_end = container.position + container.length;
                    if &container_end > start {
                        result.push(container);
                    }
                }
            }
        }

        result
    }

    /// Get all containers in a time range across all tracks
    pub fn containers_in_range(
        &self,
        start: &TimePosition,
        end: &TimePosition
    ) -> Vec<&MediaContainer> {
        let mut result = Vec::new();

        for track_id in self.track_containers.keys() {
            let track_containers = self.track_containers_in_range(*track_id, start, end);
            result.extend(track_containers);
        }

        result
    }
}