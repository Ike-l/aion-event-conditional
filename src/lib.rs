pub mod while_mapper;

pub mod prelude {
    pub use super::{
        while_mapper::{
            WhileMapper,
            while_registry::{
                WHILE_REGISTRY_ACCESS_BUILDER,
                WHILE_REGISTRY_RESOURCE_ID,
                WhileRegister,
                get_while_registry,
            },
            while_event::{
                WhileEvent
            },
            active_while_event_registry::{
                ACTIVE_WHILE_EVENT_REGISTRY_ACCESS_BUILDER,
                ACTIVE_WHILE_EVENT_REGISTRY_RESOURCE_ID,
                ActiveWhileEventRegistry,
                get_mut_active_while_event_registry,
            }
        }
    };
}