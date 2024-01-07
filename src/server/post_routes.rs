
use warp::{filters::BoxedFilter, Filter, Reply, Rejection};
use serde_json::json;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::{queries,models::{PixelImageDesc,SavePixel, IncomingPixel}};

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

    // POST /api/double/<guid> - double the pixel density in both x and y planes
    let double_pixels = warp::post()
        .and(warp::path!("api" / "double" / String))
        .and(db_conn.clone())
        .and_then(double_pixel_data);

    heartbeat_post
        .or(create_new_pixel)
        .or(save_pixels)
        .or(double_pixels)
        .or(default)
        .boxed()
}

fn json_body_new_pixel() -> impl Filter<Extract = (PixelImageDesc,), Error = warp::Rejection> + Clone {
    // Perform coercion of incoming data into a PixelImageDesc
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn json_body_save_pixel() -> impl Filter<Extract = (SavePixel,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 1024)
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

async fn double_pixel_data(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    // Update the pixelwidth
    if pixel.pixelwidth > 1 {
        match queries::update_pixelwidth_for_pixel(pixel.id, pixel.pixelwidth/2, &mut db_pool.clone()).await {
            Ok(_) => {},
            Err(err) => {
                log::error!("Error while updating pixelwidth on pixel {}", err.to_string());
                return Err(
                    warp::reject::not_found()
                );
            }
        }
    } else {
        log::warn!("Trying to double pixel density but already at smallest possible");
        // We are already at smallest pixel density
        return Err(warp::reject::not_found());
    }

    let frame_count = match queries::get_frame_count(pixel.id, &mut db_pool.clone()).await {
        Ok(count) => count,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    for frame in 0..frame_count {
        let pixels = match queries::get_pixels_for_image(pixel.id, frame, 1, &mut db_pool.clone()).await {
            Ok(pixels) => pixels,
            Err(err) => {
                log::error!("Error finding pixels {}", err);
                return Err(warp::reject::not_found())
            }
        };

        // Delete every pixel for image
        match queries::delete_pixels_for_image(pixel.id, frame, &mut db_pool.clone()).await{
            Ok(_) => {},
            Err(err) => {
                log::error!("Error deleting pixels: {}", err.to_string());
                return Err(warp::reject::not_found())
            }
        };

        // Update every pixel by creating 
        for pix in pixels.iter() {
            let new_x = pix.x * 2;
            let new_y = pix.y * 2;
            for x in 0..2 {
                for y in 0..2 {
                    let inc = IncomingPixel{
                        x: new_x + x,
                        y: new_y + y,
                        r: pix.r,
                        g: pix.g,
                        b: pix.b,
                        alpha: pix.alpha,
                        frame: pix.frame
                    };
                    match queries::save_pixel_for_image(pixel.id, &inc, &mut db_pool.clone()).await {
                        Ok(_) => {},
                        Err(err) => {
                            log::error!("Error saving pixel while doubling density {}", err.to_string());
                        }
                    }
                } 
            }
           
        }
        
    }
    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": ""}))
        )
    )
}