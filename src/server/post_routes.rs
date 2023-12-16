
use warp::{filters::BoxedFilter, Filter, Reply};
use serde_json::json;

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {
    // POST routes
    // POST /heartbeat - a POST version of the heartbeat route
    let cors = warp::cors()
        .allow_any_origin().allow_methods(&[warp::http::Method::GET, warp::http::Method::POST]);
    let heartbeat_post = warp::path!("heartbeat")
        .and(warp::post())
        .map(|| warp::reply::json(&json!({"status": "ok"})))
        .with(cors);

    // POST - catchall
    let default = warp::any()
        .and(warp::post())
        .map(|| {
            warp::reply::json(&json!({"status": "fail", "message": "Unknown route"}))
        });

    heartbeat_post
        .or(default)
        .boxed()
}