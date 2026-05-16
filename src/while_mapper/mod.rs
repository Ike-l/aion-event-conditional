use std::{any::TypeId, collections::HashSet, sync::Arc};

use aion_ecs::prelude::{Query, World};
use aion_event::prelude::{EventSystem, EventBuffer, EventHistory};
use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveWithInsert, Resource, ResourceId, Unique};
use hecs::{Entity, With};

use crate::prelude::WhileEvent;

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

struct ActiveWhileEventFlag;

impl EventSystem for WhileMapper {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        _event_history: &EventHistory
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let mut triggered_while_events = HashSet::new();
        {
            let while_events = program_registry.resolve::<Query<(Entity, &WhileEvent)>>(None, vec![]);
            if let Ok(Ok(while_events)) = while_events {
                for (entity, while_event) in while_events.query().iter() {
                    if while_event.triggered(current_events) {
                        triggered_while_events.insert(entity);
                    }
                }
            }
        }

        {
            let world = program_registry.resolve_with_insert::<Unique<World>>(None, vec![], ProgramRegistryResolveWithInsert {
                resource: Some(Box::new(|| Resource::new(World::default()))),
                resource_id: Some(ResourceId::TypeId(TypeId::of::<World>())),
                ..Default::default()
            }).expect("Resource and ResourceId are Some");

            if let Ok(Ok(Ok(mut world))) = world {
                for entity in triggered_while_events {
                    let _ = world.as_mut().insert(entity, (ActiveWhileEventFlag,));
                }
            }
        }

        let mut dead_active_while_events = HashSet::new();
        {
            let active_while_events = program_registry.resolve::<Query<With<(Entity, &WhileEvent), &ActiveWhileEventFlag>>>(None, vec![]);
            if let Ok(Ok(active_while_events)) = active_while_events {
                for (entity, active_while_event) in active_while_events.query().iter() {
                    if active_while_event.continues(current_events) {
                        if let Some(new_event) = &active_while_event.iter {
                            event_buffer.insert(new_event.clone());
                        }
                    } else {
                        if let Some(final_event) = &active_while_event.end {
                            event_buffer.insert(final_event.clone());
                        }

                        dead_active_while_events.insert(entity);
                    }                
                }
            }
        }

        {
            let world = program_registry.resolve_with_insert::<Unique<World>>(None, vec![], ProgramRegistryResolveWithInsert {
                resource: Some(Box::new(|| Resource::new(World::default()))),
                resource_id: Some(ResourceId::TypeId(TypeId::of::<World>())),
                ..Default::default()
            }).expect("Resource and ResourceId are Some");
    
            {
                if let Ok(Ok(Ok(mut world))) = world {
                    for dead_active_while_event in dead_active_while_events {                    
                        let _ = world.as_mut().remove::<(ActiveWhileEventFlag,)>(dead_active_while_event);
                    }
                }
            }
        }

        event_buffer
    }
}
