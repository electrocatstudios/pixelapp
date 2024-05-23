use std::vec;
use sqlx::{Sqlite, Row};
use sqlx::pool::Pool;
use serde_json::json;
use uuid::Uuid; 
use std::fs;

use crate::db::models::PixelImage;
use super::models::IncomingShader;
use super::{DBError,models::{PixelImageDesc, PixelPixel, IncomingPixel,PixelShading,Collection}};

pub async fn get_pixel_list(pool: &mut Pool<Sqlite>) -> Result<vec::Vec::<PixelImage>, DBError> {
    // Do the actual request to get the list
    match sqlx::query_as::<_,PixelImage>(
        "SELECT * FROM pixelimage"
    ).fetch_all(&*pool).await {
        Ok(pix) => Ok(pix),
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_collections(pool: &mut Pool<Sqlite>) -> Result<vec::Vec::<Collection>, DBError> {
    match sqlx::query_as::<_,Collection>(
        "SELECT * FROM collection" 
    ).fetch_all(&*pool).await {
        Ok(colls) => Ok(colls),
        Err(err) => Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn create_new_pixel(data: PixelImageDesc, pool: &mut Pool<Sqlite>) -> Result<String, DBError> {
    // Do the actual request to get the list
    let guid: String = format!("{:?}", Uuid::new_v4());
    
    let collection_id = match data.collection {
        Some(id) => { 
            match get_collection_by_id(id, &mut pool.clone()).await {
                Ok(collection) => Some(collection.id),
                Err(err) => {
                    return Err(DBError::DatabaseError(err.to_string()));
                }
            }
        },
        None => None  
    };
    
    // log::info!("{}", data.name);
    match sqlx::query(
            "INSERT INTO pixelimage(name, description, \
            width, height, pixelwidth, guid, collection_id) VALUES \
            ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(&data.name)
            .bind(&data.description)
            .bind(&data.width)
            .bind(&data.height)
            .bind(&data.pixelwidth)
            .bind(&guid.clone())
            .bind(&collection_id)
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
    let coll_id = match pixel.collection_id {
        Some(id) => format!("{}", id),
        None => "null".to_string()
    };    
    let ret = &json!({
        "name": &pixel.name,
        "width": &pixel.width,
        "height": &pixel.height,
        "pixelwidth": &pixel.pixelwidth,
        "guid": &pixel.guid,
        "toolbar": &toolbar,
        "menubar": &menubar,
        "collection_id": &coll_id
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

pub async fn update_pixel_details(pixel: PixelImage, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "UPDATE pixelimage SET name=$1, description=$2, \
        width=$3, height=$4, pixelwidth=$5, collection_id=$6 WHERE id=$7"
    )
    .bind(pixel.name)
    .bind(pixel.description)
    .bind(pixel.width)
    .bind(pixel.height)
    .bind(pixel.pixelwidth)
    .bind(pixel.collection_id)
    .bind(pixel.id)
    .execute(&*pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(DBError::UnknownError(err.to_string()))
    }
}


pub async fn create_collection(collection_name: String, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match get_collection_by_name(collection_name.clone(), &mut pool.clone()).await {
        Ok(_) => {
            let err_str = format!("Collection {} already exists", collection_name);
            return Err(DBError::AlreadyExists(err_str))
        },
        Err(_) => {}
    }
    match sqlx::query(
        "INSERT INTO collection (name) VALUES ($1)"
    )
    .bind(collection_name)
    .execute(&*pool).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
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
    // log::info!("Found {} results", pixels.len());
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

pub async fn get_shader_for_image_at_point(image_id: i32, frame: i32, x: i32, y: i32, pool: &mut Pool<Sqlite>) -> Result<PixelShading, DBError> {
    let pixel = match sqlx::query_as::<_,PixelShading>(
        "SELECT * FROM shading WHERE image_id=$1 \
             AND frame=$2 AND x=$3 AND y=$4 "
        )
        .bind(image_id)
        .bind(frame)
        .bind(x)
        .bind(y)
        .fetch_one(&*pool).await {
            Ok(pix) => pix,
            Err(_) => return Err(DBError::NoneFound)
        };

    Ok(pixel)
}

async fn create_pixel_for_image(image_id: i32, incoming: &IncomingPixel, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    // log::info!("Saving image pixels");
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

async fn create_shading_for_image(image_id: i32, incoming: &IncomingShader, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    // log::info!("Saving image shading");
    match sqlx::query(
        "INSERT INTO shading(image_id, x, y, \
        r, g, b, alpha, frame) VALUES \
        ($1, $2, $3, $4, $5, $6, $7, $8)"
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

async fn update_shading_for_image(shad_id: i32, incoming: &IncomingShader, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "UPDATE shading SET r=$1, g=$2, b=$3, \
        alpha=$4 WHERE id=$5"
        )
        .bind(&incoming.r)
        .bind(&incoming.g)
        .bind(&incoming.b)
        .bind(&incoming.alpha)
        .bind(&shad_id)
        .execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(DBError::DatabaseError(err.to_string()))
        }
}

pub async fn save_pixel_for_image(image_id: i32, incoming: &IncomingPixel, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    // log::info!("Saving image {}:{} - {},{},{}", incoming.x, incoming.y, incoming.r, incoming.g, incoming.b);

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

pub async fn save_shader_for_image(image_id: i32, incoming: &IncomingShader, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    // log::info!("Saving image {}:{} - {},{},{}", incoming.x, incoming.y, incoming.r, incoming.g, incoming.b);

    let pixel = match sqlx::query_as::<_, PixelShading>(
            "SELECT * FROM shading WHERE image_id=$1 AND frame=$2 AND x=$3 AND y=$4"
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
                return create_shading_for_image(image_id, incoming, &mut pool.clone()).await;
            }
        };
    log::debug!("Found pixel so updating");
    update_shading_for_image(pixel.id, incoming, &mut pool.clone()).await
} 

pub async fn delete_image_and_pixels(image_id: i32, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "DELETE FROM pixelimage WHERE id=$1"
        )
        .bind(image_id)
        .execute(&*pool).await {
            Ok(_) => {},
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
        }
    
    match delete_pixels_for_image(image_id, -1, &mut pool.clone()).await{
        Ok(_) => {},
        Err(err) => return Err(err)
    };

    Ok(())
}

pub async fn delete_pixels_for_image(image_id: i32, frame: i32, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    if frame == -1 {
        match sqlx::query(
            "DELETE FROM pixel WHERE image_id=$1"
        )
        .bind(image_id)
        .execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
        }
    } else {
        match sqlx::query(
            "DELETE FROM pixel WHERE image_id=$1 AND frame=$2"
        )
        .bind(image_id)
        .bind(frame)
        .execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
        }
    }
}

pub async fn get_frame_count(image_id: i32, pool: &mut Pool<Sqlite>) -> Result<i32, DBError> {
    match sqlx::query(
        "SELECT MAX(frame) as frame FROM pixel \
            WHERE image_id=$1"
    )
    .bind(image_id)
    .fetch_one(&*pool).await {
        Ok(row) => {
            let val: i32 = match row.try_get(0){
                Ok(val) => val,
                Err(_) => {
                    return Err(DBError::DatabaseError("Failed to find column while search for frame count".to_string()))
                } 
            };
            Ok(val+1)
        },
        Err(err) => {
            log::error!("{}", err.to_string());
            Err(DBError::DatabaseError(err.to_string()))
        }
    }
}

pub async fn update_pixelwidth_for_pixel(image_id: i32, pixelwidth: i32, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "UPDATE pixelimage SET \
            pixelwidth=$1 WHERE id=$2"
        )
        .bind(&pixelwidth)
        .bind(&image_id)
        .execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(DBError::DatabaseError(err.to_string()))
        }
}

pub async fn get_collection_by_name(collection_name: String, pool: &mut Pool<Sqlite>) -> Result<Collection, DBError> {
    let collection = match sqlx::query_as::<_, Collection>(
        "SELECT * FROM collection WHERE name=$1" 
        )
        .bind(&collection_name)
        .fetch_one(&*pool).await {
            Ok(c) => c,
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
        };
    Ok(collection)
}

pub async fn get_collection_by_id(collection_id: i32, pool: &mut Pool<Sqlite>) -> Result<Collection, DBError> {
    let collection =  match sqlx::query_as::<_, Collection>(
        "SELECT * FROM collection WHERE id=$1" 
        )
        .bind(&collection_id)
        .fetch_one(&*pool).await {
            Ok(c) => c,
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
        };
    Ok(collection)
}

pub async fn make_new_collection(name: String, pool: &mut Pool<Sqlite>) -> Result<i32, DBError> {
    // Check if already exists
    match get_collection_by_name(name.clone(), &mut pool.clone()).await {
        Ok(c) => {
            log::error!("Tried to make collection when it already exists");
            return Ok(c.id);
        },
        Err(_) => {}
    }

    // If not create new
    match sqlx::query(
        "INSERT INTO collection(name) VALUES($1)"
        )
        .bind(&name.clone())
        .execute(&*pool).await {
            Ok(_) => {},
            Err(err) => return Err(DBError::DatabaseError(err.to_string()))
    }

    // Select new and return 
    match get_collection_by_name(name.clone(), &mut pool.clone()).await {
        Ok(c) => {
            Ok(c.id)
        },
        Err(err) => {
            Err(err)
        }
    }
}