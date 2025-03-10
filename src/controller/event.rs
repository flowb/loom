use std::path::PathBuf;
use std::sync::mpsc;

use crate::model::{ProjectId, TrackId, TrackType, ContainerId, EndpointId};
use crate::tapestry::{TimePosition, Tempo, TimeSignature};

/// Events that can be dispatched from the controller
#[derive(Debug, Clone)]
pub enum Event {
    // Project events
    ProjectCreated { project_id: ProjectId },
    ProjectOpened { project_id: ProjectId, path: PathBuf },
    ProjectSaved { path: PathBuf },
    ProjectModified,

    // Track events
    TrackAdded { track_id: TrackId, track_type: TrackType },
    TrackRemoved { track_id: TrackId },
    TrackRenamed { track_id: TrackId, name: String },
    TrackOutputChanged { track_id: TrackId, output_id: Option<EndpointId> },
    TrackMuteChanged { track_id: TrackId, muted: bool },
    TrackSoloChanged { track_id: TrackId, solo: bool },

    // Container events
    ContainerAdded { container_id: ContainerId, track_id: TrackId },
    ContainerRemoved { container_id: ContainerId },
    ContainerMoved { container_id: ContainerId, position: TimePosition },
    ContainerResized { container_id: ContainerId, length: crate::tapestry::Duration },
    ContainerLoopChanged { container_id: ContainerId, loop_count: Option<u32> },

    // Timeline events
    TempoChanged { position: TimePosition, tempo: Tempo },
    TimeSignatureChanged { position: TimePosition, time_signature: TimeSignature },

    // Playback events
    PlaybackStarted,
    PlaybackStopped,
    PlaybackPaused,
    PlaybackPositionChanged { position: TimePosition },
    RecordingStarted,
    RecordingEnded,

    // Output events
    OutputsScanned,
    OutputConnected { output_id: EndpointId },
    OutputDisconnected { output_id: EndpointId },
    OutputError { output_id: EndpointId, message: String },

    // UI events
    TimelineViewChanged { scroll_offset: f32, zoom_level: f32 },

    // Error events
    Error { message: String },
}

/// Sender for events from the controller
#[derive(Clone)]
pub struct EventSender {
    sender: mpsc::Sender<Event>,
}

impl EventSender {
    pub fn new(sender: mpsc::Sender<Event>) -> Self {
        Self { sender }
    }

    pub fn send(&self, event: Event) -> Result<(), mpsc::SendError<Event>> {
        self.sender.send(event)
    }

    // Convenience methods for common events

    pub fn playback_position_changed(&self, position: TimePosition) -> Result<(), mpsc::SendError<Event>> {
        self.send(Event::PlaybackPositionChanged { position })
    }

    pub fn error(&self, message: String) -> Result<(), mpsc::SendError<Event>> {
        self.send(Event::Error { message })
    }
}

/// Receiver for events from the controller
pub struct EventReceiver {
    receiver: mpsc::Receiver<Event>,
}

impl EventReceiver {
    pub fn new(receiver: mpsc::Receiver<Event>) -> Self {
        Self { receiver }
    }

    pub fn recv(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<Event, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn iter(&self) -> mpsc::Iter<'_, Event> {
        self.receiver.iter()
    }
}

/// Create an event channel
pub fn create_event_channel() -> (EventSender, EventReceiver) {
    let (sender, receiver) = mpsc::channel();
    (EventSender::new(sender), EventReceiver::new(receiver))
}

/// Hub for distributing events to multiple receivers
pub struct EventHub {
    receivers: Vec<EventSender>,
}

impl EventHub {
    pub fn new() -> Self {
        Self { receivers: Vec::new() }
    }

    pub fn add_receiver(&mut self, receiver: EventSender) {
        self.receivers.push(receiver);
    }

    pub fn dispatch(&self, event: Event) {
        for receiver in &self.receivers {
            let _ = receiver.send(event.clone());
        }
    }
}