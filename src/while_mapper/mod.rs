use std::sync::Arc;

use aion_event::prelude::{EventSystem, EventBuffer, EventHistory};
use aion_program::prelude::{ProgramRegistry};

use crate::prelude::{get_while_registry, get_mut_active_while_event_registry};

pub mod while_registry;
pub mod active_while_event_registry;
pub mod while_event;

/// # While Mapper
/// 
/// 1. Collects all `triggered` `While Event`s
/// 2. Put them into `Active While Event Registry`
/// 3. Iterate over `Active While Event Registry`
/// 4. If `Active While Event` `continues`: spawn `iter`
/// 5. Else: spawn `end` & remove `Active While Event`
/// 
/// ## While Event
/// 
/// On `when` If `condition` do `iter` then `end`
/// 
/// ## While Registry
/// 
/// Holds all `While Event`s; inactive, and active
/// 
/// ## Active While Event Registry
/// 
/// Holds all `Active While Event`s
pub struct WhileMapper;

impl EventSystem for WhileMapper {
    fn execute(
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        _event_history: &EventHistory
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let triggered_events = match get_while_registry(program_registry) {
            Ok(Ok(Ok(while_registry))) => {
                Some(while_registry.as_ref().iter().filter(|while_event| while_event.triggered(current_events)).cloned().collect::<Vec<_>>())
            },
            _ => None
        };

        match get_mut_active_while_event_registry(program_registry) {
            Ok(Ok(Ok(mut active_while_event_registry))) => {
                if let Some(triggered_events) = triggered_events {
                    let active_while_event_registry = active_while_event_registry.as_mut();
                    active_while_event_registry.extend(triggered_events);
                    active_while_event_registry.retain(|active_loop| {
                        if active_loop.continues(current_events) {
                            if let Some(new_event) = &active_loop.iter {
                                event_buffer.insert(new_event.clone());
                            }
            
                            true
                        } else {
                            if let Some(new_event) = &active_loop.end {
                                event_buffer.insert(new_event.clone());
                            }
            
                            false
                        }
                    });
                }                
            },
            _ => ()
        }

        event_buffer
    }
}
