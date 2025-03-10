// src/output/midi.rs
use std::error::Error;
use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};

use crate::model::{EndpointId, EndpointType};
use crate::output::event::{OutputEvent, OutputEventType};
use crate::output::endpoint::OutputEndpoint;

pub struct MidiOutputEndpoint {
    id: EndpointId,
    name: String,
    port_name: String,
    port_index: usize,
    connection: Option<MidiOutputConnection>,
}

impl MidiOutputEndpoint {
    pub fn new(id: EndpointId, name: String, port_index: usize, port_name: String) -> Self {
        Self {
            id,
            name,
            port_name,
            port_index,
            connection: None,
        }
    }

    fn send_midi_message(&mut self, message: &[u8]) -> Result<(), Box<dyn Error>> {
        if let Some(conn) = &mut self.connection {
            conn.send(message)?;
            Ok(())
        } else {
            Err("MIDI device not connected".into())
        }
    }
}

impl OutputEndpoint for MidiOutputEndpoint {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_connected() {
            return Ok(());
        }

        let midi_out = MidiOutput::new("Loom")?;
        let ports = midi_out.ports();

        if self.port_index >= ports.len() {
            return Err(format!("MIDI port index {} out of range", self.port_index).into());
        }

        let port = &ports[self.port_index];
        let conn = midi_out.connect(port, "loom-output")?;

        self.connection = Some(conn);
        Ok(())
    }

    fn disconnect(&mut self) {
        self.connection = None;
    }

    fn send_event(&mut self, event: &OutputEvent) -> Result<(), Box<dyn Error>> {
        match &event.event_type {
            OutputEventType::MidiNoteOn { channel, note, velocity } => {
                let message = [0x90 | (channel & 0x0F), *note, *velocity];
                self.send_midi_message(&message)
            }

            OutputEventType::MidiNoteOff { channel, note } => {
                let message = [0x80 | (channel & 0x0F), *note, 0];
                self.send_midi_message(&message)
            }

            OutputEventType::MidiControlChange { channel, controller, value } => {
                let message = [0xB0 | (channel & 0x0F), *controller, *value];
                self.send_midi_message(&message)
            }

            OutputEventType::MidiProgramChange { channel, program } => {
                let message = [0xC0 | (channel & 0x0F), *program];
                self.send_midi_message(&message)
            }

            OutputEventType::MidiPitchBend { channel, value } => {
                // Convert i16 (-8192 to 8191) to 14-bit value (0 to 16383)
                let bend_value = (*value + 8192) as u16;
                let message = [
                    0xE0 | (channel & 0x0F),
                    (bend_value & 0x7F) as u8,         // LSB
                    ((bend_value >> 7) & 0x7F) as u8   // MSB
                ];
                self.send_midi_message(&message)
            }

            OutputEventType::MidiAftertouch { channel, pressure } => {
                let message = [0xD0 | (channel & 0x0F), *pressure];
                self.send_midi_message(&message)
            }

            OutputEventType::MidiPolyAftertouch { channel, note, pressure } => {
                let message = [0xA0 | (channel & 0x0F), *note, *pressure];
                self.send_midi_message(&message)
            }

            _ => Err("Unsupported event type for MIDI endpoint".into())
        }
    }

    fn endpoint_type(&self) -> EndpointType {
        EndpointType::Midi
    }
}