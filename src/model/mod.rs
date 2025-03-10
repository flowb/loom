pub mod project;
pub mod timeline;
pub mod track;
pub mod container;
pub mod endpoint;

// Re-export common types
pub use project::{Project, ProjectId, ProjectSettings};
pub use timeline::{Timeline, TimelineId};
pub use track::{Track, TrackId, TrackType, Color};
pub use container::{MediaContainer, ContainerId, MediaContent, PlaybackMode};
pub use container::{PatternId, MidiClipId, AudioFileId};
pub use endpoint::{EndpointConfig, EndpointId, EndpointType, EndpointParameters};