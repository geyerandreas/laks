use axum::extract::FromRef;
use nyxos_auth::jwt::JwtService;
use nyxos_storage::file::FileStorage;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub jwt: JwtService,
    pub storage: FileStorage,
}

impl FromRef<AppState> for sqlx::SqlitePool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for JwtService {
    fn from_ref(input: &AppState) -> Self {
        input.jwt.clone()
    }
}

impl FromRef<AppState> for FileStorage {
    fn from_ref(input: &AppState) -> Self {
        input.storage.clone()
    }
}
