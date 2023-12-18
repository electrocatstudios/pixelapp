use std::vec;
use sqlx::Sqlite;
use sqlx::pool::Pool;
use serde_json::json;
use crate::db::models::PixelImage;
use super::{DBError,models::PixelImageDesc};
use uuid::Uuid; 

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

pub async fn get_pixel_details(guid: String, pool: &mut Pool<Sqlite>) -> Result<serde_json::Value, DBError> {
    let pixel = match sqlx::query_as::<_,PixelImage>(
            "SELECT * FROM pixelimage WHERE guid=$1"
        )
        .bind(guid)
        .fetch_one(&*pool).await {
            Ok(pix) => pix,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
        
    let ret = &json!({
        "name": &pixel.name,
        "width": &pixel.width,
        "height": &pixel.height,
        "pixelwidth": &pixel.pixelwidth,
        "guid": &pixel.guid
    });

    Ok(ret.clone())
}