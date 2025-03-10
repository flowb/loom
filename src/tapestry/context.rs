use std::sync::Arc;
use crate::tapestry::position::TimePosition;
use crate::tapestry::tempo_map::TempoMap;

/// Thread-safe wrapper for sharing the tempo map
pub struct TimeContext {
    tempo_map: Arc<TempoMap>,
    current_position: TimePosition,
}

impl TimeContext {
    pub fn new(tempo_map: TempoMap) -> Self {
        Self {
            tempo_map: Arc::new(tempo_map),
            current_position: TimePosition::zero(),
        }
    }
    
    pub fn set_position(&mut self, position: TimePosition) {
        self.current_position = position;
    }
    
    pub fn position(&self) -> TimePosition {
        self.current_position
    }
    
    pub fn tempo_map(&self) -> &TempoMap {
        &self.tempo_map
    }
    
    /// Create a clone with a new Arc pointer to the same TempoMap
    pub fn clone_with_new_position(&self, position: TimePosition) -> Self {
        Self {
            tempo_map: Arc::clone(&self.tempo_map),
            current_position: position,
        }
    }
}