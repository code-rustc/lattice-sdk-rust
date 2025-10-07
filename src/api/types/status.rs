pub use crate::prelude::*;

/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Status {
    /// The status code, which should be an enum value of [google.rpc.Code][google.rpc.Code].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the [google.rpc.Status.details][google.rpc.Status.details] field, or localized by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A list of messages that carry the error details.  There is a common set of message types for APIs to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<GoogleProtobufAny>>,
}
