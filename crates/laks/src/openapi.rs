use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(
    title = "laks API",
    version = "0.1.0",
    description = "Self-hosted Python registry"
))]
pub struct ApiDoc;
