use std::{collections::HashSet, sync::Arc};

use aion_program::prelude::{Resource, ProgramRegistryResolveWithInsert, AccessBuilder, ResourceId, ProgramRegistry, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError};
use aion_processor::prelude::{Unique};

use crate::prelude::{WhileEvent};

/// # Active While Event Registry
/// 
/// Holds all `Active While Event`s
pub type ActiveWhileEventRegistry = HashSet<WhileEvent>;

pub const ACTIVE_WHILE_EVENT_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventConditional ActiveWhileEventRegistry");

pub const ACTIVE_WHILE_EVENT_REGISTRY_ACCESS_BUILDER: AccessBuilder<'static> = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(ACTIVE_WHILE_EVENT_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_active_while_event_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Unique<'a, ActiveWhileEventRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<ActiveWhileEventRegistry>>(
        vec![ACTIVE_WHILE_EVENT_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ActiveWhileEventRegistry::default()))), 
            resource_id: Some(ACTIVE_WHILE_EVENT_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}