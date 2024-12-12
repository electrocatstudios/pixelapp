use serde_json::json;

use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{SqlitePool, Pool, Sqlite};

use crate::db::{animation_models::AnimationDesc, animation_queries};

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {

    // POST - /animation - create a new animation
    let new_animation = warp::path!("api" / "new_animation")
        .and(warp::post())
        .map(|| {
            warp::reply::json(&json!({"status": "fail", "message": "Not implementned"}))
        });

    // POST /api/animation_new - create new pixel
    let create_new_animation = warp::post()
        .and(warp::path!("api" / "animation_new"))
        .and(json_body_new_animation())
        .and(db_conn.clone())
        .and_then(create_new_animation_impl);

    new_animation
        .or(create_new_animation)
        .boxed()
}

// -- JSON Parsers
fn json_body_new_animation() -> impl Filter<Extract = (AnimationDesc,), Error = warp::Rejection> + Clone {
    // Perform coercion of incoming data into a AnimationDesc
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// -- Route implementations

// Create the new pixel from the components
async fn create_new_animation_impl(anim: AnimationDesc, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pix_id = match animation_queries::create_new_animation(anim, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    };
    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "pixelid": pix_id}))
        )
    )
}