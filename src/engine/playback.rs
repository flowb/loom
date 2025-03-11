// src/engine/playback.rs
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Instant, Duration};
use eframe::glow::TEXTURE_BORDER_COLOR;
use crate::engine::clock::{ClockSource, InternalClock};
use crate::controller::event::{Event, EventSender};
use crate::model::{MediaContent, Project};
use crate::tapestry::{TimePosition};
use crate::output::{OutputEvent, OutputSystem};


pub struct PlaybackEngine {
    project: Arc<RwLock<Project>>,
    clock_source: Box<dyn ClockSource>,
    playing: Arc<AtomicBool>,
    playback_thread: Option<JoinHandle<()>>,
    event_sender: EventSender,
    output_system: Arc<RwLock<OutputSystem>>,
}

impl PlaybackEngine {
    pub fn new(
        project: Arc<RwLock<Project>>,
        event_sender: EventSender,
        output_system: Arc<RwLock<OutputSystem>>,
    ) -> Self {
        // Start with internal clock by default
        let clock_source = Box::new(InternalClock {
            start_time: None,
            sample_rate: 44100,
        });

        Self {
            project,
            clock_source,
            playing: Arc::new(AtomicBool::new(false)),
            playback_thread: None,
            event_sender,
            output_system,
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
        let output_system = Arc::clone(&self.output_system);

        // Start playback thread
        self.playback_thread = Some(thread::spawn(move || {
            let start_time = Instant::now();
            let mut last_position = TimePosition::zero();

            while playing.load(Ordering::SeqCst) {
                //get current time from clock
                let elapsed_secs = start_time.elapsed().as_secs_f64();

                let project_guard = project.read().unwrap();

                let current_position = {

                    TimePosition::from_seconds(
                        elapsed_secs,
                        project_guard.settings.reference_sample_rate,
                    )
                };

                let _ = event_sender.send(Event::PlaybackPositionChanged {
                    position: current_position
                });

                //process events between last_ and current_ positions
                let events_to_process = {
                    if let Some(timeline) = project_guard.active_timeline() {
                        // get containers that are active in this range
                        let containers = timeline.containers_in_range(
                            &last_position,
                            &current_position,
                        );

                        // convert container events to output events
                        let mut output_events = Vec::new();
                        for container in containers {
                            match &container.content {
                                MediaContent::Pattern(pattern_id) => {
                                    todo!();
                                },
                                MediaContent::MidiClip(midi_clip_id) => {
                                    todo!();
                                },
                                MediaContent::AudioFile(audio_file_id) => {
                                    todo!();
                                },
                            }
                        }

                        // test tone generator
                        let beat = project_guard.tempo_map.position_to_beats(&current_position);
                        if beat.floor() > project_guard.tempo_map.position_to_beats(&last_position).floor() {
                            // beat tone
                            let note = 60 + ((beat as u8) % 12); // procedural C major
                            let event = OutputEvent::midi_note_on(0, note, 100, None);
                            output_events.push(event.clone());

                            let note_off = OutputEvent::midi_note_off(0,note, None);
                            output_events.push(note_off);
                        }
                        output_events
                    } else {
                        Vec::new()
                    }
                };

                drop(project_guard);

                {
                    let mut output_guard = output_system.write().unwrap();
                    for event in events_to_process {
                        let _ = output_guard.send_event(&event);
                    }
                }

                last_position = current_position;
                thread::sleep(Duration::from_millis(1));
            }
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