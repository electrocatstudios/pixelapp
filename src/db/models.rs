use serde::{Deserialize, Serialize};
use std::vec;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct PixelImage {
    pub id: i32, // By default, using barrel's types::primary() results in i32
    pub name: String,
    pub description: String,
    pub owner_id: i32,
    pub width: i32,
    pub height: i32,
    pub pixelwidth: i32,
    pub guid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelImageDesc {
    pub name: String,
    pub description: String,
    pub width: i32,
    pub height: i32,
    pub pixelwidth: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DuplicateImageData {
    pub newimagename: String
}



#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct PixelPixel {
    pub id: i32, // By default, using barrel's types::primary() results in i32
    pub image_id: i32,
    pub x: i32,
    pub y: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub alpha: f64,
    pub layer: i32,
    pub frame: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct PixelShading {
    pub id: i32, // By default, using barrel's types::primary() results in i32
    pub image_id: i32,
    pub x: i32,
    pub y: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub alpha: f64,
    pub frame: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SavePixel {
    pub guid: String,
    pub pixels: vec::Vec::<IncomingPixel>,
    pub shaders: vec::Vec::<IncomingPixel>,
}

#[derive(Serialize, Deserialize)]
pub struct PixelSaveFile {
    pub name: String,
    pub description: String,
    pub width: i32,
    pub height: i32,
    pub pixelwidth: i32,
    pub pixels: vec::Vec::<PixelPixel>,
    pub shaders: vec::Vec::<PixelShading>
}

#[derive(Serialize, Deserialize)]
pub struct IncomingPixel {
    pub x: i32,
    pub y: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub alpha: f64,
    pub frame: i32,
}

impl IncomingPixel {
    pub fn from_pixel_pixel(pixel: &PixelPixel) -> Self {
        IncomingPixel {
            x: pixel.x,
            y: pixel.y,
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
            alpha: pixel.alpha,
            frame: pixel.frame
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IncomingShader {
    pub x: i32,
    pub y: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub alpha: f64,
    pub frame: i32,
}

impl IncomingShader {
    pub fn from_pixel_shader(shad: &PixelShading) -> Self {
        IncomingShader {
            x: shad.x,
            y: shad.y,
            r: shad.r,
            g: shad.g,
            b: shad.b,
            alpha: shad.alpha,
            frame: shad.frame
        }
    }
}