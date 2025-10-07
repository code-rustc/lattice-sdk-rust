use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EntitiesClient {
    pub http_client: HttpClient,
}

impl EntitiesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Publish an entity for ingest into the Entities API. Entities created with this method are "owned" by the originator: other sources,
    /// such as the UI, may not edit or delete these entities. The server validates entities at API call time and
    /// returns an error if the entity is invalid.
    ///
    /// An entity ID must be provided when calling this endpoint. If the entity referenced by the entity ID does not exist
    /// then it will be created. Otherwise the entity will be updated. An entity will only be updated if its
    /// provenance.sourceUpdateTime is greater than the provenance.sourceUpdateTime of the existing entity.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn publish_entity(
        &self,
        request: &Entity,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "api/v1/entities",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_entity(
        &self,
        entity_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/entities/{}", entity_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Only fields marked with overridable can be overridden. Please refer to our documentation to see the comprehensive
    /// list of fields that can be overridden. The entity in the request body should only have a value set on the field
    /// specified in the field path parameter. Field paths are rooted in the base entity object and must be represented
    /// using lower_snake_case. Do not include "entity" in the field path.
    ///
    /// Note that overrides are applied in an eventually consistent manner. If multiple overrides are created
    /// concurrently for the same field path, the last writer wins.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The unique ID of the entity to override
    /// * `field_path` - fieldPath to override
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn override_entity(
        &self,
        entity_id: &String,
        field_path: &String,
        request: &EntityOverride,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v1/entities/{}{}", entity_id, field_path),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// This operation clears the override value from the specified field path on the entity.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The unique ID of the entity to undo an override from.
    /// * `field_path` - The fieldPath to clear overrides from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove_entity_override(
        &self,
        entity_id: &String,
        field_path: &String,
        options: Option<RequestOptions>,
    ) -> Result<Entity, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v1/entities/{}{}", entity_id, field_path),
                None,
                None,
                options,
            )
            .await
    }

    /// This is a long polling API that will first return all pre-existing data and then return all new data as
    /// it becomes available. If you want to start a new polling session then open a request with an empty
    /// 'sessionToken' in the request body. The server will return a new session token in the response.
    /// If you want to retrieve the next batch of results from an existing polling session then send the session
    /// token you received from the server in the request body. If no new data is available then the server will
    /// hold the connection open for up to 5 minutes. After the 5 minute timeout period, the server will close the
    /// connection with no results and you may resume polling with the same session token. If your session falls behind
    /// more than 3x the total number of entities in the environment, the server will terminate your session.
    /// In this case you must start a new session by sending a request with an empty session token.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn long_poll_entity_events(
        &self,
        request: &EntityEventRequest,
        options: Option<RequestOptions>,
    ) -> Result<EntityEventResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/entities/events",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Establishes a persistent connection to stream entity events as they occur.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming response
    pub async fn stream_entities(
        &self,
        request: &EntityStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/entities/stream",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
