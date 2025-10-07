use crate::{ApiError, ClientConfig};

pub mod entities;
pub mod entity;
pub mod object;
pub mod objects;
pub mod task;
pub mod tasks;
pub struct LatticeClient {
    pub config: ClientConfig,
    pub entities: EntitiesClient,
    pub tasks: TasksClient,
    pub objects: ObjectsClient,
}

impl LatticeClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            entities: EntitiesClient::new(config.clone())?,
            tasks: TasksClient::new(config.clone())?,
            objects: ObjectsClient::new(config.clone())?,
        })
    }
}

pub use entities::EntitiesClient;
pub use entity::EntityClient;
pub use object::ObjectClient;
pub use objects::ObjectsClient;
pub use task::TaskClient;
pub use tasks::TasksClient;
