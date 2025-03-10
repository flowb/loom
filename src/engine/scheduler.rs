// src/engine/scheduler.rs
use std::collections::BTreeMap;
use crate::model::container::MediaContainer;
use crate::tapestry::TimePosition;
use crate::output::event::OutputEvent;

pub struct EventScheduler {
    scheduled_events: BTreeMap<TimePosition, Vec<OutputEvent>>,
}

impl EventScheduler {
    pub fn new() -> Self {
        Self {
            scheduled_events: BTreeMap::new(),
        }
    }

    /// Schedule an output event at a specific time
    pub fn schedule_event(&mut self, position: TimePosition, event: OutputEvent) {
        self.scheduled_events.entry(position).or_insert_with(Vec::new).push(event);
    }

    /// Get all events between two time positions
    pub fn get_events(&self, start: &TimePosition, end: &TimePosition) -> Vec<(TimePosition, OutputEvent)> {
        let mut result = Vec::new();

        for (pos, events) in self.scheduled_events.range(start..end) {
            for event in events {
                result.push((*pos, event.clone()));
            }
        }

        result
    }

    /// Clear all scheduled events
    pub fn clear(&mut self) {
        self.scheduled_events.clear();
    }
}