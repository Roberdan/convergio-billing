//! HTTP API routes for convergio-billing.

use axum::Router;

/// Returns the router for this crate's API endpoints.
pub fn routes() -> Router {
    Router::new()
    // .route("/api/billing/health", get(health))
}
