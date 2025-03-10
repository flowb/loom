// src/engine/playback.rs
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::engine::clock::{ClockSource, InternalClock};
use crate::controller::event::{Event, EventSender};
use crate::model::Project;
use crate::tapestry::TimePosition;


pub struct PlaybackEngine {
    project: Arc<RwLock<Project>>,
    clock_source: Box<dyn ClockSource>,
    playing: AtomicBool,
    playback_thread: Option<JoinHandle<()>>,
    event_sender: EventSender,
}

impl PlaybackEngine {
    pub fn new(
        project: Arc<RwLock<Project>>,
        event_sender: EventSender,
    ) -> Self {
        // Start with internal clock by default
        let clock_source = Box::new(InternalClock {
            start_time: None,
            sample_rate: 44100,
        });

        Self {
            project,
            clock_source,
            playing: AtomicBool::new(false),
            playback_thread: None,
            event_sender,
        }
    }

    pub fn set_clock_source(&mut self, clock_source: Box<dyn ClockSource>) {
        // Stop playback if running
        if self.playing.load(Ordering::SeqCst) {
            self.stop();
        }

        self.clock_source = clock_source;
    }

    pub fn play(&mut self) {
        if self.playing.load(Ordering::SeqCst) {
            return; // Already playing
        }

        self.playing.store(true, Ordering::SeqCst);

        // Clone necessary references for the playback thread
        let project = Arc::clone(&self.project);
        let playing = Arc::clone(&self.playing);
        let event_sender = self.event_sender.clone();

        // Start playback thread
        self.playback_thread = Some(thread::spawn(move || {
            // Playback logic will go here
            // This is where we'll evaluate the timeline and send events
        }));
    }

    pub fn stop(&mut self) {
        if !self.playing.load(Ordering::SeqCst) {
            return; // Not playing
        }

        self.playing.store(false, Ordering::SeqCst);

        // Wait for playback thread to finish
        if let Some(thread) = self.playback_thread.take() {
            let _ = thread.join();
        }
    }

    pub fn seek(&mut self, position: TimePosition) {
        // Seeking logic
    }

    pub fn current_position(&self) -> TimePosition {
        self.clock_source.current_time()
    }

    pub fn is_playing(&self) -> bool {
        self.playing.load(Ordering::SeqCst)
    }
}