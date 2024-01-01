
use warp::{filters::BoxedFilter, Filter, Reply, Rejection};
use serde_json::json;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::{queries,models::{PixelImageDesc,SavePixel}};

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
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

    // POST /api/pixel - get list of pixels
    let create_new_pixel = warp::post()
        .and(warp::path!("api" / "new"))
        .and(json_body_new_pixel())
        .and(db_conn.clone())
        .and_then(create_new_pixel_impl);

    // POST /api/save - save pixel data for image
    let save_pixels = warp::post()
        .and(warp::path!("api" / "save"))
        .and(json_body_save_pixel())
        .and(db_conn.clone())
        .and_then(save_pixel_data);

    heartbeat_post
        .or(create_new_pixel)
        .or(save_pixels)
        .or(default)
        .boxed()
}

fn json_body_new_pixel() -> impl Filter<Extract = (PixelImageDesc,), Error = warp::Rejection> + Clone {
    // Perform coercion of incoming data into a PixelImageDesc
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn json_body_save_pixel() -> impl Filter<Extract = (SavePixel,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// Create the new pixel from the components
async fn create_new_pixel_impl(pixel: PixelImageDesc, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pix_id = match queries::create_new_pixel(pixel, &mut db_pool.clone()).await {
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

async fn save_pixel_data(save_pixel: SavePixel, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    log::info!("Saving pixel data from func save_pixel_data");
    let pixel = match queries::get_pixel_details(save_pixel.guid.clone(), &mut db_pool.clone()).await {
        Ok(p) => p,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    };

    // Loop through pixels and save
    for p in save_pixel.pixels.iter() {
        log::info!("Next pixel {}:{} - {}", p.x, p.y, save_pixel.guid.clone());
        match queries::save_pixel_for_image(pixel.id, p, &mut db_pool.clone()).await {
            Ok(_) => {},
            Err(err) => {
                return Ok(
                    Box::new(
                        warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                    )
                )
            }
        }
    }

    // Loop through shaders and save

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": ""}))
        )
    )
}