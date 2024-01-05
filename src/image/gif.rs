use sqlx::{Sqlite, Row};
use sqlx::pool::Pool;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use image::{Rgba,RgbaImage};

use gif::{Frame, Encoder, Repeat};

use std::vec::Vec;

use crate::db::{queries, DBError};
use crate::server::query_params::GifRenderQuery;

pub async fn render_gif(guid: String, _query: GifRenderQuery, db_pool: Pool<Sqlite>) -> Result<Vec<u8>, String> {
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
    // let mut output: Vec<u8> = Vec::new();
    {
        let mut encoder = Encoder::new(&mut image, pixel.width as u16, pixel.height as u16, color_map).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        for frame_counter in 0..frame_count {
            let mut nxt = RgbaImage::new(pixel.width as u32, pixel.height as u32);

            let mut frame = Frame::from_rgba(pixel.width as u16,pixel.height as u16, &mut nxt.clone().into_raw());
            frame.delay = 1;
            
            let pixels = match queries::get_pixels_for_image(pixel.id, frame_counter, 1, &mut db_pool.clone()).await {
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

                        // let col_hex = color_to_hex_string(pix.r as u8, pix.g as u8, pix.b as u8);

                        // let color = match query_subs.get(&col_hex) {
                        //     Some(new_col) => {
                        //         // log::info!("Subbing color: {} -> {}", new_col.clone(), col_hex);
                        //         let (r, g, b) = hex_string_to_color(new_col.to_string());
                        //         Rgba([r,g,b,(pix.alpha * 255.0) as u8])
                        //     },
                        //     None => Rgba([pix.r as u8, pix.g as u8, pix.b as u8, (pix.alpha * 255.0) as u8 ])
                        // };
                        let color = Rgba([pix.r as u8, pix.g as u8, pix.b as u8, (pix.alpha * 255.0) as u8 ]);
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

            // TODO: Add in reverse and both renders
            // query.get_render_type()

            encoder.write_frame(&frame.clone()).unwrap();
        }
        // encoder.write_to(&output)
    }
    // let mut ret = Vec::new();
    // image.seek(SeekFrom::Start(0)).unwrap();    
    // image.read_to_end(&mut ret).unwrap();
    
    // image.seek(0);
    Ok(image.get_ref().to_owned())
}