use aion_event::prelude::{Event, EventBuffer};

/// # While Event
/// 
/// On `when` If `condition` do `iter` then `end`
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct WhileEvent {
    pub when: Option<Event>,
    pub condition: Option<Event>,
    pub iter: Option<Event>,
    pub end: Option<Event>
}

impl WhileEvent {
    pub fn triggered(&self, events: &EventBuffer) -> bool {
        match &self.when {
            Some(when) => events.contains(when),
            None => true,
        }
    }

    pub fn continues(&self, events: &EventBuffer) -> bool {
        match &self.condition {
            Some(condition) => events.contains(condition),
            None => true,
        }
    }
}