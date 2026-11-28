use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

/// An error response for the registry endpoints.
/// See [RFC 9457](https://datatracker.ietf.org/doc/html/rfc9457) for more details.
/// See [Simple Repository API](https://packaging.python.org/en/latest/specifications/simple-repository-api/#error-responses)
#[derive(Debug, Serialize, ToSchema)]
pub struct ProblemDetails {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub problem_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ProblemDetails {
    /// Create a new `ProblemDetails` instance.
    pub fn new(status: StatusCode, title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            problem_type: None,
            title: Some(title.into()),
            status: Some(status.as_u16()),
            detail: Some(detail.into()),
            instance: None,
        }
    }

    pub fn with_type(mut self, problem_type: impl Into<String>) -> Self {
        self.problem_type = Some(problem_type.into());
        self
    }

    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }
}

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        let status = self
            .status
            .and_then(|code| StatusCode::from_u16(code).ok())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (
            status,
            [(header::CONTENT_TYPE, "application/problem+json")],
            Json(self),
        )
            .into_response()
    }
}

pub enum RegistryError {
    ProjectNotFound(String),
    Internal(anyhow::Error),
}

impl From<laks_db::error::ProjectDatabaseError> for RegistryError {
    fn from(error: laks_db::error::ProjectDatabaseError) -> Self {
        match error {
            laks_db::error::ProjectDatabaseError::ProjectNotFound(project) => {
                Self::ProjectNotFound(project)
            }
            _ => Self::Internal(anyhow::Error::new(error)),
        }
    }
}

impl IntoResponse for RegistryError {
    fn into_response(self) -> Response {
        let problem = match self {
            RegistryError::ProjectNotFound(project) => ProblemDetails::new(
                StatusCode::NOT_FOUND,
                "Project not found",
                format!("No project named '{}' exist on this index.", project),
            ),
            RegistryError::Internal(err) => {
                // Log the internal error details server-side without leaking them to clients
                tracing::error!(error = ?err, "Internal index error");
                ProblemDetails::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "An unexpected error occurred while processing the index request.",
                )
            }
        };
        problem.into_response()
    }
}
