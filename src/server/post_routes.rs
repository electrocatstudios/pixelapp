
use warp::{filters::BoxedFilter, Filter, Reply, Rejection};
use serde_json::json;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::{queries,models::{PixelImageDesc,SavePixel,IncomingPixel,IncomingShader,DuplicateImageData,PixelSaveFile,PixelResizeData}};

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

    // POST /api/dupliate/<guid> - create a copy of the image with the given name
    let duplicate_image = warp::post()
        .and(warp::path!("api" / "duplicate" / String))
        .and(json_body_duplicate_image())
        .and(db_conn.clone())
        .and_then(duplicate_image_impl);

    // POST /api/newfromfile - upload a json file and create image from that
    let newfromfile = warp::post()
        .and(warp::path!("api" / "newfromfile"))
        .and(json_body_newfromfile())
        .and(db_conn.clone())
        .and_then(newfromfile_impl);

    // POST /api/size/<guid> - update the width and height for image
    let resize_image = warp::post()
        .and(warp::path!("api" / "size" / String))
        .and(json_body_for_resize())
        .and(db_conn.clone())
        .and_then(resize_image_impl);

    heartbeat_post
        .or(create_new_pixel)
        .or(save_pixels)
        .or(double_pixels)
        .or(duplicate_image)
        .or(newfromfile)
        .or(resize_image)
        .or(default)
        .boxed()
}

// -- JSON PARSERS 
fn json_body_new_pixel() -> impl Filter<Extract = (PixelImageDesc,), Error = warp::Rejection> + Clone {
    // Perform coercion of incoming data into a PixelImageDesc
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn json_body_save_pixel() -> impl Filter<Extract = (SavePixel,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 1024)
        .and(warp::body::json())
}

fn json_body_duplicate_image() -> impl Filter<Extract = (DuplicateImageData,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn json_body_newfromfile() ->  impl Filter<Extract = (PixelSaveFile,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

fn json_body_for_resize() -> impl Filter<Extract = (PixelResizeData,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// -- END JSON PARSERS 

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

    // Remove all pixels for image to prevent confusion
    match queries::delete_pixels_for_image(pixel.id, -1, &mut db_pool.clone()).await {
        Ok(_) => {},
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    }

    // Loop through pixels and save
    for p in save_pixel.pixels.iter() {
        // log::info!("Next pixel {}:{} - {}", p.x, p.y, save_pixel.guid.clone());
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

async fn duplicate_image_impl(guid: String, duplicate_data: DuplicateImageData, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let old_pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": "failed to create duplicate", "guid": ""}))
                )
            )
        }
    };
    
    let new_pixel = PixelImageDesc{
        name: duplicate_data.newimagename.clone(),
        description: old_pixel.description.clone(),
        width: old_pixel.width,
        height: old_pixel.height,
        pixelwidth: old_pixel.pixelwidth
    };

    let new_guid = match queries::create_new_pixel(new_pixel, &mut db_pool.clone()).await {
        Ok(guid) => guid,
        Err(err) => {
            log::error!("Error duplicating image: {}", err.to_string());
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": "failed to create duplicate", "guid": ""}))
                )
            )
        }
    };

    let new_pixel = match queries::get_pixel_details(new_guid.clone(), &mut db_pool.clone()).await {
        Ok(pixel) => pixel,
        Err(err) => {
            log::error!("Error getting new pixel after cloning: {}", err.to_string());
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": "failed to create duplicate", "guid": ""}))
                )
            )
        }
    };

    let pixels = match queries::get_all_pixels_for_image(old_pixel.id, &mut db_pool.clone()).await {
        Ok(pixels) => pixels,
        Err(err) => {
            log::error!("Error getting pixels for old image: {}", err.to_string());
            return Err(
                warp::reject::not_found()
            )
        }
    };

    for pix in pixels.iter() {
        let inc = IncomingPixel{
            x: pix.x,
            y: pix.y,
            r: pix.r,
            g: pix.g,
            b: pix.b,
            alpha: pix.alpha,
            frame: pix.frame,
        };
        match queries::save_pixel_for_image(new_pixel.id, &inc, &mut db_pool.clone()).await {
            Ok(_) => {},
            Err(err) => {
                log::error!("Error saving pixel for new image: {}", err.to_string());
            }
        }

    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "guid": new_guid}))
        )
    )
}

async fn newfromfile_impl(pixel_data: PixelSaveFile, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let new_pix_data = PixelImageDesc{
        name: pixel_data.name,
        description: pixel_data.description,
        width: pixel_data.width,
        height: pixel_data.height,
        pixelwidth: pixel_data.pixelwidth
    };
    let pix_id = match queries::create_new_pixel(new_pix_data, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string(), "guid": ""}))
                )
            )
        }
    };

    let image_id = match queries::get_pixel_details(pix_id.clone(), &mut db_pool.clone()).await {
        Ok(res) => res.id,
        Err(_) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": "failed to find image during create from file", "guid": ""}))
                )
            )
        }
    };

    for pix in pixel_data.pixels.iter() {
        let inc = IncomingPixel::from_pixel_pixel(pix);

        match queries::save_pixel_for_image(image_id, &inc, &mut db_pool.clone()).await {
            Ok(_) => {},
            Err(err) => {
                log::error!("Error saving pixel during create from file: {}", err.to_string());
                return Ok(
                    Box::new(
                        warp::reply::json(&json!({"status": "fail", "message": "failed to find image during create from file", "guid": ""}))
                    )
                )
            }
        }
    }

    for shad in pixel_data.shaders.iter() {
        let inc = IncomingShader::from_pixel_shader(shad);

        match queries::save_shader_for_image(image_id, &inc, &mut db_pool.clone()).await {
            Ok(_) => {},
            Err(err) => {
                log::error!("Error saving pixel during create from file: {}", err.to_string());
                return Ok(
                    Box::new(
                        warp::reply::json(&json!({"status": "fail", "message": "failed to find image during create from file", "guid": ""}))
                    )
                )
            }
        }
    }

    return Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "guid": pix_id}))
        )
    )
}

async fn resize_image_impl(guid: String, newsize: PixelResizeData, db_pool: Pool<Sqlite> ) -> Result<Box<dyn Reply>, Rejection> {
    let mut pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    pixel.width = newsize.width as i32;
    pixel.height = newsize.height as i32;

    match queries::update_pixel_details(pixel, &mut db_pool.clone()).await {
        Ok(_) => {
            Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "ok", "message": ""}))
                )
            )
        },
        Err(err) => {
            log::error!("Error updating image size: {}", err.to_string());
            Err(
                warp::reject::not_found()
            )
        }
    }

}