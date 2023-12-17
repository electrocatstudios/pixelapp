use std::vec;
use sqlx::Sqlite;
use sqlx::pool::Pool;
use crate::db::models::PixelImage;
use super::DBError;

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