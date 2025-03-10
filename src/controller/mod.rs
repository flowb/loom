pub mod event;
pub mod command;
pub mod snapshot;
pub mod dispatcher;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration as StdDuration;

use crate::controller::command::{Command, CommandReceiver};
use crate::controller::event::{Event, EventHub};
use crate::controller::snapshot::{ProjectSnapshot, TimelineSnapshot};
use crate::engine::playback::PlaybackEngine;
use crate::model::{Project, TrackId, TrackType, ContainerId, MediaContent};
use crate::output::system::OutputSystem;
use crate::tapestry::{TimePosition, Duration};

/// Central controller for the application
pub struct Controller {
    command_receiver: CommandReceiver,
    event_hub: EventHub,
    project: Arc<RwLock<Project>>,
    playback_engine: Arc<RwLock<PlaybackEngine>>,
    output_system: Arc<RwLock<OutputSystem>>,
    running: bool,
}

impl Controller {
    /// Create a new controller
    pub fn new(
        command_receiver: CommandReceiver,
        event_hub: EventHub,
        project: Arc<RwLock<Project>>,
        playback_engine: Arc<RwLock<PlaybackEngine>>,
        output_system: Arc<RwLock<OutputSystem>>,
    ) -> Self {
        Self {
            command_receiver,
            event_hub,
            project,
            playback_engine,
            output_system,
            running: false,
        }
    }

    /// Run the controller in a separate thread
    pub fn run_in_thread(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }

    /// Run the controller in the current thread
    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            // Process commands
            match self.command_receiver.try_recv() {
                Ok(command) => self.handle_command(command),
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    // No commands to process
                },
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    // Command sender was dropped, exit
                    self.running = false;
                    break;
                }
            }

            // Sleep a bit to avoid busy waiting
            thread::sleep(StdDuration::from_millis(1));
        }
    }

    /// Stop the controller
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Handle a command
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::CreateProject { name } => self.handle_create_project(name),
            Command::AddTrack { name, track_type } => self.handle_add_track(name, track_type),
            Command::MoveContainer { container_id, new_position } =>
                self.handle_move_container(container_id, new_position),
            Command::ResizeContainer { container_id, new_length } =>
                self.handle_resize_container(container_id, new_length),
            Command::Play => self.handle_play(),
            Command::Stop => self.handle_stop(),
            Command::Seek { position } => self.handle_seek(position),
            Command::Shutdown => self.handle_shutdown(),
            // Handle other commands...
            _ => {
                // Unhandled command
                let message = format!("Unhandled command: {:?}", command);
                self.event_hub.dispatch(Event::Error { message });
            }
        }
    }

    /// Create a project snapshot for the UI
    pub fn create_project_snapshot(&self) -> ProjectSnapshot {
        let project = self.project.read().unwrap();
        let engine = self.playback_engine.read().unwrap();

        let active_timeline = if let Some(timeline) = project.active_timeline() {
            let playback_position = if engine.is_playing() {
                Some(engine.current_position())
            } else {
                None
            };

            Some(TimelineSnapshot::from_timeline(timeline, playback_position))
        } else {
            None
        };

        let endpoints = project.endpoints.values()
            .map(|config| config.into())
            .collect();

        ProjectSnapshot {
            name: project.name.clone(),
            active_timeline,
            endpoints,
        }
    }

    // Command handlers

    fn handle_create_project(&mut self, name: String) {
        let new_project = Project::new(name.clone());
        let project_id = new_project.id;

        {
            let mut project = self.project.write().unwrap();
            *project = new_project;
        }

        self.event_hub.dispatch(Event::ProjectCreated { project_id });
    }

    fn handle_add_track(&mut self, name: String, track_type: TrackType) {
        let track_id = {
            let mut project = self.project.write().unwrap();

            if let Some(timeline) = project.active_timeline_mut() {
                let track = crate::model::Track::new(name, track_type);
                let id = track.id;
                timeline.add_track(track);
                Some(id)
            } else {
                None
            }
        };

        if let Some(id) = track_id {
            self.event_hub.dispatch(Event::TrackAdded {
                track_id: id,
                track_type
            });
        }
    }

    fn handle_move_container(&mut self, container_id: ContainerId, new_position: TimePosition) {
        let success = {
            let mut project = self.project.write().unwrap();

            if let Some(timeline) = project.active_timeline_mut() {
                timeline.move_container(container_id, new_position)
            } else {
                false
            }
        };

        if success {
            self.event_hub.dispatch(Event::ContainerMoved {
                container_id,
                position: new_position
            });
        }
    }

    fn handle_resize_container(&mut self, container_id: ContainerId, new_length: Duration) {
        let success = {
            let mut project = self.project.write().unwrap();

            if let Some(timeline) = project.active_timeline_mut() {
                if let Some(container) = timeline.container_mut(container_id) {
                    container.length = new_length;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };

        if success {
            self.event_hub.dispatch(Event::ContainerResized {
                container_id,
                length: new_length
            });
        }
    }

    fn handle_play(&mut self) {
        let mut engine = self.playback_engine.write().unwrap();
        engine.play();

        self.event_hub.dispatch(Event::PlaybackStarted);
    }

    fn handle_stop(&mut self) {
        let mut engine = self.playback_engine.write().unwrap();
        engine.stop();

        self.event_hub.dispatch(Event::PlaybackStopped);
    }

    fn handle_seek(&mut self, position: TimePosition) {
        let mut engine = self.playback_engine.write().unwrap();
        engine.seek(position);

        self.event_hub.dispatch(Event::PlaybackPositionChanged { position });
    }

    fn handle_shutdown(&mut self) {
        self.running = false;
    }
}