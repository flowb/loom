use std::path::PathBuf;
use std::sync::mpsc;

use crate::model::{TrackId, TrackType, ContainerId, MediaContent, EndpointId};
use crate::tapestry::{TimePosition, Duration, Tempo, TimeSignature};
use crate::engine::clock::ClockSourceType;

/// Commands that can be sent to the controller
#[derive(Debug, Clone)]
pub enum Command {
    // Project commands
    CreateProject { name: String },
    OpenProject { path: PathBuf },
    SaveProject { path: PathBuf },

    // Track commands
    AddTrack { name: String, track_type: TrackType },
    RemoveTrack { track_id: TrackId },
    RenameTrack { track_id: TrackId, name: String },
    SetTrackOutput { track_id: TrackId, output_id: Option<EndpointId> },
    MuteTrack { track_id: TrackId, muted: bool },
    SoloTrack { track_id: TrackId, solo: bool },

    // Container commands
    AddContainer { track_id: TrackId, position: TimePosition, content: MediaContent },
    RemoveContainer { container_id: ContainerId },
    MoveContainer { container_id: ContainerId, new_position: TimePosition },
    ResizeContainer { container_id: ContainerId, new_length: Duration },
    SetContainerLoop { container_id: ContainerId, loop_count: Option<u32> },
    SetContainerTimeScale { container_id: ContainerId, time_scale: f64 },

    // Timeline commands
    SetTempo { position: TimePosition, tempo: Tempo },
    SetTimeSignature { position: TimePosition, time_signature: TimeSignature },

    // Transport commands
    Play,
    Stop,
    Pause,
    Seek { position: TimePosition },
    Record { enabled: bool },

    // Output commands
    ScanOutputs,
    ConnectOutput { output_id: EndpointId },
    DisconnectOutput { output_id: EndpointId },

    // Clock commands
    SetClockSource { source_type: ClockSourceType },

    // System commands
    Shutdown,
}

/// Sender for commands to the controller
pub struct CommandSender {
    sender: mpsc::Sender<Command>,
}

impl CommandSender {
    pub fn new(sender: mpsc::Sender<Command>) -> Self {
        Self { sender }
    }

    pub fn send(&self, command: Command) -> Result<(), mpsc::SendError<Command>> {
        self.sender.send(command)
    }

    // Convenience methods for common commands

    pub fn play(&self) -> Result<(), mpsc::SendError<Command>> {
        self.send(Command::Play)
    }

    pub fn stop(&self) -> Result<(), mpsc::SendError<Command>> {
        self.send(Command::Stop)
    }

    pub fn pause(&self) -> Result<(), mpsc::SendError<Command>> {
        self.send(Command::Pause)
    }

    pub fn seek(&self, position: TimePosition) -> Result<(), mpsc::SendError<Command>> {
        self.send(Command::Seek { position })
    }

    pub fn add_track(&self, name: String, track_type: TrackType) -> Result<(), mpsc::SendError<Command>> {
        self.send(Command::AddTrack { name, track_type })
    }

    pub fn add_container(
        &self,
        track_id: TrackId,
        position: TimePosition,
        content: MediaContent
    ) -> Result<(), mpsc::SendError<Command>> {
        self.send(Command::AddContainer {
            track_id,
            position,
            content
        })
    }

    pub fn shutdown(&self) -> Result<(), mpsc::SendError<Command>> {
        self.send(Command::Shutdown)
    }
}

/// Receiver for commands in the controller
pub struct CommandReceiver {
    receiver: mpsc::Receiver<Command>,
}

impl CommandReceiver {
    pub fn new(receiver: mpsc::Receiver<Command>) -> Self {
        Self { receiver }
    }

    pub fn recv(&self) -> Result<Command, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<Command, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn iter(&self) -> mpsc::Iter<'_, Command> {
        self.receiver.iter()
    }
}

/// Create a command channel
pub fn create_command_channel() -> (CommandSender, CommandReceiver) {
    let (sender, receiver) = mpsc::channel();
    (CommandSender::new(sender), CommandReceiver::new(receiver))
}