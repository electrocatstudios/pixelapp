use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Sqlite,SqlitePool,Pool};
use serde_json::json;

use crate::db::queries;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
        .allow_any_origin().allow_methods(&[warp::http::Method::DELETE]);
    let heartbeat_delete = warp::path!("heartbeat")
        .and(warp::delete())
        .map(|| warp::reply::json(&json!({"status": "ok"})))
        .with(cors);

    // DELETE - /api/<guid> 
    let delete_pixel = warp::delete()
        .and(warp::path!("api" / String))
        .and(db_conn.clone())
        .and_then(delete_pixel_impl);

    // Fallback fail - unknown route
    let default = warp::any()
        .and(warp::delete())
        .map(|| {
            warp::reply::json(&json!({"status": "fail", "message": "Unknown route"}))
        });
    
    heartbeat_delete
        .or(delete_pixel)
        .or(default)
        .boxed()

}

async fn delete_pixel_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    log::info!("Deleting pixel {}", guid);

    let pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(p) => p,
        Err(_) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": "Pixel doesn't exist"}))
                )
            )
        }
    };

    match queries::delete_image_and_pixels(pixel.id, &mut db_pool.clone()).await {
        Ok(_) => {
            Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "ok", "message": ""}))
                )
            )
        },
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    }
    
}