// src/output/endpoint.rs
use std::error::Error;
use crate::output::event::OutputEvent;

/// Trait for output endpoints that can receive events
pub trait OutputEndpoint: Send + Sync {
    /// Get the name of this endpoint
    fn name(&self) -> &str;

    /// Check if this endpoint is connected
    fn is_connected(&self) -> bool;

    /// Connect to the physical device/resource
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;

    /// Disconnect from the physical device/resource
    fn disconnect(&mut self);

    /// Send an event to this endpoint
    fn send_event(&mut self, event: &OutputEvent) -> Result<(), Box<dyn Error>>;

    /// Get the type of this endpoint
    fn endpoint_type(&self) -> crate::model::EndpointType;
}