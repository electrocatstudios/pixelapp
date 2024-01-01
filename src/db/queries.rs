use std::vec;
use sqlx::Sqlite;
use sqlx::pool::Pool;
use serde_json::json;
use uuid::Uuid; 
use std::fs;

use crate::db::models::PixelImage;
use super::{DBError,models::{PixelImageDesc, PixelPixel, IncomingPixel,PixelShading}};

pub async fn get_pixel_list(pool: &mut Pool<Sqlite>) -> Result<vec::Vec::<PixelImage>, DBError> {
    // Do the actual request to get the list
    let pixels = match sqlx::query_as::<_,PixelImage>(
                        "SELECT * FROM pixelimage"
                    ).fetch_all(&*pool).await {
                        Ok(pix) => pix,
                        Err(err) => return Err(DBError::UnknownError(err.to_string()))
                    };
    Ok(pixels)
}

pub async fn create_new_pixel(data: PixelImageDesc, pool: &mut Pool<Sqlite>) -> Result<String, DBError> {
    // Do the actual request to get the list
    let guid: String = format!("{:?}", Uuid::new_v4());
    // log::info!("{}", data.name);
    match sqlx::query(
            "INSERT INTO pixelimage(name, description, \
            width, height, pixelwidth, guid) VALUES \
            ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&data.name)
            .bind(&data.description)
            .bind(&data.width)
            .bind(&data.height)
            .bind(&data.pixelwidth)
            .bind(&guid.clone())
            .execute(&*pool).await {
                Ok(_) => Ok(guid),
                Err(err) => Err(DBError::DatabaseError(err.to_string()))
            }
}

pub async fn get_pixel_details_as_json(guid: String, pool: &mut Pool<Sqlite>) -> Result<serde_json::Value, DBError> {
    let pixel = match get_pixel_details(guid, pool).await {
        Ok(pix) => pix,
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    };
        
    let toolbar: String = fs::read_to_string("templates/snippets/toolbar.html").unwrap().parse().unwrap();
    let menubar: String = fs::read_to_string("templates/snippets/menubar.html").unwrap().parse().unwrap();
    let ret = &json!({
        "name": &pixel.name,
        "width": &pixel.width,
        "height": &pixel.height,
        "pixelwidth": &pixel.pixelwidth,
        "guid": &pixel.guid,
        "toolbar": &toolbar,
        "menubar": &menubar
    });

    Ok(ret.clone())
}

pub async fn get_pixel_details(guid: String, pool: &mut Pool<Sqlite>) -> Result<PixelImage, DBError> {
    match sqlx::query_as::<_,PixelImage>(
        "SELECT * FROM pixelimage WHERE guid=$1"
    )
    .bind(guid)
    .fetch_one(&*pool).await {
        Ok(pix) => Ok(pix),
        Err(err) => Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_pixels_for_image(image_id: i32, frame: i32, layer: i32, pool: &mut Pool<Sqlite>) -> Result<Vec::<PixelPixel>, DBError> {
    let pixels = match sqlx::query_as::<_,PixelPixel>(
        "SELECT * FROM pixel WHERE image_id=$1 AND layer=$2 AND frame=$3"
        )
        .bind(image_id)
        .bind(layer)
        .bind(frame)
        .fetch_all(&*pool).await {
            Ok(pix) => pix,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
    
    Ok(pixels)
}

pub async fn get_all_pixels_for_image(image_id: i32, pool: &mut Pool<Sqlite>) -> Result<Vec::<PixelPixel>, DBError> {
    let pixels = match sqlx::query_as::<_,PixelPixel>(
        "SELECT * FROM pixel WHERE image_id=$1"
        )
        .bind(image_id)
        .fetch_all(&*pool).await {
            Ok(pix) => pix,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
    // TODO: Put in logging to check how many results there are.....
    log::info!("Found {} results", pixels.len());
    Ok(pixels)
}

pub async fn get_all_shaders_for_image(image_id: i32, pool: &mut Pool<Sqlite>) -> Result<Vec::<PixelShading>, DBError> {
    let pixels = match sqlx::query_as::<_,PixelShading>(
        "SELECT * FROM shading WHERE image_id=$1"
        )
        .bind(image_id)
        .fetch_all(&*pool).await {
            Ok(pix) => pix,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };

    Ok(pixels)
}

async fn create_pixel_for_image(image_id: i32, incoming: &IncomingPixel, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    log::info!("Saving image pixels");
    match sqlx::query(
        "INSERT INTO pixel(image_id, x, y, \
        r, g, b, alpha, layer, frame) VALUES \
        ($1, $2, $3, $4, $5, $6, $7, 1, $8)"
        )
        .bind(&image_id)
        .bind(&incoming.x)
        .bind(&incoming.y)
        .bind(&incoming.r)
        .bind(&incoming.g)
        .bind(&incoming.b)
        .bind(&incoming.alpha)
        .bind(&incoming.frame)
        .execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(DBError::DatabaseError(err.to_string()))
        }
}

async fn update_pixel_for_image(pixel_id: i32, incoming: &IncomingPixel, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "UPDATE pixel SET r=$1, g=$2, b=$3, \
        alpha=$4 WHERE id=$5"
        )
        .bind(&incoming.r)
        .bind(&incoming.g)
        .bind(&incoming.b)
        .bind(&incoming.alpha)
        .bind(&pixel_id)
        .execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(DBError::DatabaseError(err.to_string()))
        }
}

pub async fn save_pixel_for_image(image_id: i32, incoming: &IncomingPixel, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    log::info!("Saving image {}:{} - {},{},{}", incoming.x, incoming.y, incoming.r, incoming.g, incoming.b);

    let pixel = match sqlx::query_as::<_, PixelPixel>(
            "SELECT * FROM pixel WHERE image_id=$1 AND frame=$2 AND x=$3 AND y=$4"
        )
        .bind(image_id)
        .bind(incoming.frame)
        .bind(incoming.x)
        .bind(incoming.y)
        .fetch_one(&*pool).await {
            Ok(pix) => pix,
            Err(_) => {
                log::debug!("Pixel being created");
                // Pixel doesn't exist so create it
                return create_pixel_for_image(image_id, incoming, &mut pool.clone()).await;
            }
        };
    log::debug!("Found pixel so updating");
    update_pixel_for_image(pixel.id, incoming, &mut pool.clone()).await
} 

pub async fn delete_image_and_pixels(image_id: i32, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "DELETE FROM pixel WHERE image_id=$1"
        )
        .bind(image_id)
        .execute(&*pool).await {
            Ok(_) => {},
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
        }

    match sqlx::query(
        "DELETE FROM pixel WHERE image_id=$1"
        )
        .bind(image_id)
        .execute(&*pool).await {
            Ok(_) => {},
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
        }
           
    match sqlx::query(
        "DELETE FROM pixelimage WHERE id=$1"
    )
    .bind(image_id)
    .execute(&*pool).await {
        Ok(_) => {},
        Err(err) => return Err(DBError::DatabaseError(err.to_string()))
    }

    Ok(())
}