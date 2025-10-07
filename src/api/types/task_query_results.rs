pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskQueryResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    /// Incomplete results can be detected by a non-empty nextPageToken field in the query results. In order to retrieve
    /// the next page, perform the exact same request as previously and append a pageToken field with the value of
    /// nextPageToken from the previous page. A new nextPageToken is provided on the following pages until all the
    /// results are retrieved.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
