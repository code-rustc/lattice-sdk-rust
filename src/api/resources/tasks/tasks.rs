use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Submit a request to create a task and schedule it for delivery. Tasks, once delivered, will
    /// be asynchronously updated by their destined agent.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_task(
        &self,
        request: &TaskCreation,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/tasks",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_task(
        &self,
        task_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/tasks/{}", task_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the status of a task.
    ///
    /// # Arguments
    ///
    /// * `task_id` - ID of task to update status of
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_task_status(
        &self,
        task_id: &String,
        request: &TaskStatusUpdate,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v1/tasks/{}", task_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Query for tasks by a specified search criteria.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn query_tasks(
        &self,
        request: &TaskQuery,
        options: Option<RequestOptions>,
    ) -> Result<TaskQueryResults, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/tasks/query",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// This is a long polling API that will block until a new task is ready for delivery. If no new task is
    /// available then the server will hold on to your request for up to 5 minutes, after that 5 minute timeout
    /// period you will be expected to reinitiate a new request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn listen_as_agent(
        &self,
        request: &AgentListener,
        options: Option<RequestOptions>,
    ) -> Result<AgentRequest, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/agent/listen",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
