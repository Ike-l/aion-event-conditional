use std::{collections::HashSet, sync::Arc};

use aion_program::prelude::{Resource, ProgramRegistryResolveWithInsert, AccessBuilder, ResourceId, ProgramRegistry, ProgramRegistryReplaceResourceError, ResolveResourceError, AccessSubmissionError};
use aion_processor::prelude::{Shared};

use crate::prelude::{WhileEvent};

/// # While Registry
/// 
/// Holds all `While Event`s; inactive, and active
pub type WhileRegister = HashSet<WhileEvent>;

pub const WHILE_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventConditional WhileRegistry");

pub const WHILE_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(WHILE_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_while_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Shared<'a, WhileRegister>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Shared<WhileRegister>>(
        vec![WHILE_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(WhileRegister::default()))), 
            resource_id: Some(WHILE_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}