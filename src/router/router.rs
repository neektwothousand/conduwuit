use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Router};
use conduit::Error;
use conduit_api::router::{state, state::Guard};
use conduit_service::Services;
use http::{StatusCode, Uri};
use ruma::api::client::error::ErrorKind;

pub(crate) fn build(services: &Arc<Services>) -> (Router, Guard) {
	let router = Router::<state::State>::new();
	let (state, guard) = state::create(services.clone());
	let router = conduit_api::router::build(router, &services.server)
		.route("/", get(it_works))
		.fallback(not_found)
		.with_state(state);

	(router, guard)
}

async fn not_found(_uri: Uri) -> impl IntoResponse {
	Error::Request(ErrorKind::Unrecognized, "Not Found".into(), StatusCode::NOT_FOUND)
}

async fn it_works() -> &'static str { "hewwo from conduwuit woof!" }
