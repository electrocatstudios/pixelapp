use sqlx::Sqlite;
use sqlx::pool::Pool;
use std::io::Cursor;
use image::{Rgba,RgbaImage};

use gif::{Frame, Encoder, Repeat};

use std::vec::Vec;

use crate::db::queries;
use crate::server::query_params::{GifRenderQuery, GifRenderType};

pub async fn render_gif(guid: String, query: GifRenderQuery, db_pool: Pool<Sqlite>) -> Result<Box<Vec<u8>>, String> {
    let pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err( "Unable to find pixel image".to_string())
        }
    };

    let frame_count = match queries::get_frame_count(pixel.id, &mut db_pool.clone()).await {
        Ok(count) => count,
        Err(_) => {
            return Err("Failed to get frame count".to_string());
        }
    };

    let color_map = &[];
   
    // Create fake "file"
    let mut image = Cursor::new(Vec::new());
    
    let mut encoder = Encoder::new(&mut image, pixel.width as u16, pixel.height as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let frame_order: Vec::<i32> = match query.get_render_type() {
        GifRenderType::Backward => {
            let mut order: Vec::<i32> = (0..frame_count).collect();
            order.reverse();
            order
        },
        GifRenderType::Forward => (0..frame_count).collect(),
        GifRenderType::Both => {
            // Get the out and return values
            let mut first: Vec::<i32> = (0..frame_count).collect();
            let mut second: Vec::<i32> = (1..frame_count-1).collect();
            second.reverse();
            first.append(&mut second);
            first
        }
        GifRenderType::None => (0..frame_count).collect() // Default is forwards
    };
    // log::info!("{:?} -> {:?}", query.get_render_type(), frame_order);
    for frame_counter in frame_order.iter() {
        let single_pixel = Rgba([0, 0, 0, 255]);
        let mut nxt = RgbaImage::from_pixel(pixel.width as u32, pixel.height as u32, single_pixel);

        let pixels = match queries::get_pixels_for_image(pixel.id, *frame_counter, 1, &mut db_pool.clone()).await {
            Ok(pixels) => pixels,
            Err(err) => {
                log::error!("Error finding pixels {}", err);
                continue;
            }
        };

        for pix in pixels.iter() {
        
            for x in 0..pixel.pixelwidth {
                for y in 0..pixel.pixelwidth {
                    let offset_x = pix.x * pixel.pixelwidth;
                    let offset_y = pix.y * pixel.pixelwidth;

                    // TODO: Allow color subs in the gif
                    // let col_hex = color_to_hex_string(pix.r as u8, pix.g as u8, pix.b as u8);

                    // let color = match query_subs.get(&col_hex) {
                    //     Some(new_col) => {
                    //         // log::info!("Subbing color: {} -> {}", new_col.clone(), col_hex);
                    //         let (r, g, b) = hex_string_to_color(new_col.to_string());
                    //         Rgba([r,g,b,(pix.alpha * 255.0) as u8])
                    //     },
                    //     None => Rgba([pix.r as u8, pix.g as u8, pix.b as u8, (pix.alpha * 255.0) as u8 ])
                    // };

                    // TODO: Proper color matching with the background
                    let red = (pix.r as f64 * pix.alpha) as u8;
                    let green = (pix.g as f64 * pix.alpha) as u8;
                    let blue = (pix.b as f64 * pix.alpha) as u8;
                    
                    let color = Rgba([red, green, blue, 255]);
                    let nxt_x = (offset_x + x) as u32;
                    let nxt_y = (offset_y + y) as u32;
                    if nxt_x < nxt.width() || nxt_y < nxt.height() {
                        nxt.put_pixel( 
                            nxt_x as u32, 
                            nxt_y as u32, 
                            color
                        );
                    } else {
                        log::debug!("Index {}, {} is out of bounds for the image", nxt_x, nxt_y);
                    }
                }
            }
        }
        let frame = Frame::from_rgba(pixel.width as u16,pixel.height as u16, &mut nxt.into_raw());
        // frame.delay = 1;
        encoder.write_frame(&frame.clone()).unwrap();
    }

    Ok(Box::new(encoder.get_ref().get_ref().to_vec()))
}

