
use warp::{filters::BoxedFilter, Filter, Reply};
use serde_json::json;
use sqlx::SqlitePool;

pub(super) async fn make_routes(_db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
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

    // let create_new_pixel = create_new_pixel(db_conn.clone());

    heartbeat_post
        .or(default)
        // .or(create_new_pixel)
        .boxed()
}

// fn create_new_pixel(db_conn: &mut SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("heartbeat")
//         .and(warp::any())
//         .map( move ||  )
// }