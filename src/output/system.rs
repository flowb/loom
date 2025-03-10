// src/output/system.rs
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::model::{EndpointId, EndpointConfig, EndpointType, EndpointParameters};
use crate::output::endpoint::OutputEndpoint;
use crate::output::midi::MidiOutputEndpoint;
use crate::output::event::OutputEvent;

pub struct OutputSystem {
    endpoints: HashMap<EndpointId, Box<dyn OutputEndpoint>>,
}

impl OutputSystem {
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
        }
    }

    /// Scan for available MIDI output ports
    pub fn scan_midi_outputs(&self) -> Vec<(usize, String)> {
        let midi_out = match midir::MidiOutput::new("Loom") {
            Ok(out) => out,
            Err(_) => return Vec::new(),
        };

        let ports = midi_out.ports();
        let mut result = Vec::new();

        for (i, port) in ports.iter().enumerate() {
            if let Ok(name) = midi_out.port_name(port) {
                result.push((i, name));
            }
        }

        result
    }

    /// Add an endpoint based on configuration
    pub fn add_endpoint(&mut self, config: &EndpointConfig) -> Result<(), Box<dyn Error>> {
        match config.endpoint_type {
            EndpointType::Midi => {
                let params = match &config.parameters {
                    EndpointParameters::Midi { channel } => (channel.unwrap_or(0)),
                    _ => return Err("Invalid parameters for MIDI endpoint".into()),
                };

                // Extract port index from device_id (format: "index:name")
                let parts: Vec<&str> = config.device_id.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err("Invalid MIDI device ID format".into());
                }

                let port_index = parts[0].parse::<usize>()
                    .map_err(|_| "Invalid MIDI port index")?;
                let port_name = parts[1].to_string();

                let endpoint = MidiOutputEndpoint::new(
                    config.id,
                    config.name.clone(),
                    port_index,
                    port_name,
                );

                self.endpoints.insert(config.id, Box::new(endpoint));
                Ok(())
            },

            // Other endpoint types will be added here
            _ => Err(format!("Endpoint type {:?} not implemented", config.endpoint_type).into()),
        }
    }

    /// Connect to an endpoint
    pub fn connect_endpoint(&mut self, id: EndpointId) -> Result<(), Box<dyn Error>> {
        if let Some(endpoint) = self.endpoints.get_mut(&id) {
            endpoint.connect()
        } else {
            Err(format!("Endpoint {} not found", id).into())
        }
    }

    /// Disconnect from an endpoint
    pub fn disconnect_endpoint(&mut self, id: EndpointId) {
        if let Some(endpoint) = self.endpoints.get_mut(&id) {
            endpoint.disconnect();
        }
    }

    /// Check if endpoint is connected
    pub fn is_endpoint_connected(&self, id: EndpointId) -> bool {
        self.endpoints.get(&id)
            .map(|e| e.is_connected())
            .unwrap_or(false)
    }

    /// Send an event to a specific endpoint
    pub fn send_event_to_endpoint(&mut self, id: EndpointId, event: &OutputEvent) -> Result<(), Box<dyn Error>> {
        if let Some(endpoint) = self.endpoints.get_mut(&id) {
            endpoint.send_event(event)
        } else {
            Err(format!("Endpoint {} not found", id).into())
        }
    }

    /// Send an event to all compatible endpoints
    pub fn send_event(&mut self, event: &OutputEvent) -> Vec<Result<(), Box<dyn Error>>> {
        let mut results = Vec::new();

        if let Some(target) = event.target {
            // Send to specific endpoint
            results.push(self.send_event_to_endpoint(target, event));
        } else {
            // Send to all compatible endpoints
            for (id, endpoint) in &mut self.endpoints {
                match event.event_type {
                    OutputEvent::MidiNoteOn { .. } |
                    OutputEvent::MidiNoteOff { .. } |
                    OutputEvent::MidiControlChange { .. } |
                    OutputEvent::MidiProgramChange { .. } |
                    OutputEvent::MidiPitchBend { .. } |
                    OutputEvent::MidiAftertouch { .. } |
                    OutputEvent::MidiPolyAftertouch { .. }
                    if endpoint.endpoint_type() == EndpointType::Midi => {
                        results.push(endpoint.send_event(event));
                    },

                    OutputEvent::AudioBuffer { .. }
                    if endpoint.endpoint_type() == EndpointType::Audio => {
                        results.push(endpoint.send_event(event));
                    },

                    _ => {}
                }
            }
        }

        results
    }
}