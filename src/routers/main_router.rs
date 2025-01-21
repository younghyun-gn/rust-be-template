use std::sync::Arc;

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
};
use tower_http::compression::CompressionLayer;

use crate::{
    handlers::{fallback::fallback_handler, root::root_handler, user::signup::signup_handler},
    init::state::ServerState,
};

use super::middleware::logging::log_middleware;

pub fn build_router(state: Arc<ServerState>) -> axum::Router {
    let app = axum::Router::new()
        .route("/", get(root_handler))
        .route("/auth/signup", post(signup_handler))
        .fallback(get(fallback_handler))
        .layer(from_fn_with_state(state.clone(), log_middleware))
        .layer(CompressionLayer::new())
        .with_state(state);

    app
}
