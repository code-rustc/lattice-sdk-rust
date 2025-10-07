use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("BadRequestError: Bad request - {{message}}")]
    BadRequestError {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("UnauthorizedError: Authentication failed - {{message}}")]
    UnauthorizedError {
        message: String,
        auth_type: Option<String>,
    },
    #[error("NotFoundError: Resource not found - {{message}}")]
    NotFoundError {
        message: String,
        resource_id: Option<String>,
        resource_type: Option<String>,
    },
    #[error("RequestTimeoutError: {{message}}")]
    RequestTimeoutError { message: String },
    #[error("TooManyRequestsError: Rate limit exceeded - {{message}}")]
    TooManyRequestsError {
        message: String,
        retry_after_seconds: Option<u64>,
        limit_type: Option<String>,
    },
    #[error("InternalServerError: Internal server error - {{message}}")]
    InternalServerError {
        message: String,
        error_id: Option<String>,
    },
    #[error("ContentTooLargeError: {{message}}")]
    ContentTooLargeError { message: String },
    #[error("InsufficientStorageError: {{message}}")]
    InsufficientStorageError { message: String },
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Network error: {0}")]
    Network(reqwest::Error),
    #[error("Serialization error: {0}")]
    Serialization(serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
}

impl ApiError {
    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        match status_code {
            400 => {
                // Parse error body for BadRequestError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadRequestError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            field: parsed
                                .get("field")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            details: parsed
                                .get("details")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::BadRequestError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    field: None,
                    details: None,
                };
            }
            401 => {
                // Parse error body for UnauthorizedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnauthorizedError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            auth_type: parsed
                                .get("auth_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnauthorizedError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    auth_type: None,
                };
            }
            404 => {
                // Parse error body for NotFoundError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NotFoundError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            resource_id: parsed
                                .get("resource_id")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            resource_type: parsed
                                .get("resource_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::NotFoundError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    resource_id: None,
                    resource_type: None,
                };
            }
            408 => {
                // Parse error body for RequestTimeoutError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::RequestTimeoutError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::RequestTimeoutError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            429 => {
                // Parse error body for TooManyRequestsError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::TooManyRequestsError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            retry_after_seconds: parsed
                                .get("retry_after_seconds")
                                .and_then(|v| v.as_u64()),
                            limit_type: parsed
                                .get("limit_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::TooManyRequestsError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    retry_after_seconds: None,
                    limit_type: None,
                };
            }
            500 => {
                // Parse error body for InternalServerError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::InternalServerError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            error_id: parsed
                                .get("error_id")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::InternalServerError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    error_id: None,
                };
            }
            413 => {
                // Parse error body for ContentTooLargeError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ContentTooLargeError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::ContentTooLargeError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            507 => {
                // Parse error body for InsufficientStorageError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::InsufficientStorageError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::InsufficientStorageError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            _ => Self::Http {
                status: status_code,
                message: body.unwrap_or("Unknown error").to_string(),
            },
        }
    }
}
