use serde_json::json;

use warp::{Filter, filters::BoxedFilter, Reply};
use sqlx::SqlitePool;

pub(super) async fn make_routes(_db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {

    // POST - /animation - create a new animation
    let new_animation = warp::path!("api" / "new_animation")
        .and(warp::post())
        .map(|| {
            warp::reply::json(&json!({"status": "fail", "message": "Not implementned"}))
        });

    new_animation.boxed()
}
