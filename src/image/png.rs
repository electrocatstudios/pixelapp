use std::{collections::HashMap, fs::File, io::{BufReader, Read}, path::Path};

use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;

use crate::{db::animation_models::{AnimationDetails, AnimationLimbMoveDetails, LimbEndPoint}, utils};

use super::ImageRenderError;


pub fn render_animation_png(animation_details: AnimationDetails, perc: f64) -> Result<Box<Vec<u8>>, ImageRenderError> {
    if perc < 0.0 || perc > 1.0 {
        return Err(ImageRenderError::PercentageNotValid(format!("Percentage {} is not between 0 and 1", perc)));
    }
    let background_color = Rgba([0, 0, 0, 255]);
    let mut nxt: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::from_pixel(animation_details.width as u32, animation_details.height as u32, background_color);

    let mut parent_list = HashMap::<&String, LimbEndPoint>::new();

    for limb in animation_details.animation_limbs.iter() {
        // Find index of pieces
        if limb.animation_limb_moves.len() < 1 {
            continue;
        }
        let col = utils::hex_string_to_color(limb.color.to_string());
        let col = Rgba([col.0, col.1, col.2, 255u8]);
        if limb.animation_limb_moves.len() < 2 || perc == 0.0 {
            // We only have one for this limb - just use that or first frame
            let lm = limb.animation_limb_moves[0];
            let start = if limb.parent != "".to_string() {
                let p = parent_list.get(&limb.parent).unwrap();
                (lm.x as f32 + p.x, lm.y as f32 + p.y)
            }else {    
                (lm.x as f32, lm.y as f32)
            };

            let end = (
                start.0 + (lm.rot.sin() * lm.length) as f32 ,
                start.1 + (lm.rot.cos() * lm.length) as f32,
            );

            draw_line_segment_mut(&mut nxt, start, end, col);
            parent_list.insert(&limb.name, LimbEndPoint::new(end.0, end.1));
        } else if perc == 1.0 {
            // Last frame - use last frame details
            let lm = limb.animation_limb_moves.last().unwrap();
            let start = if limb.parent != "".to_string() {
                let p = parent_list.get(&limb.parent).unwrap();
                (lm.x as f32 + p.x, lm.y as f32 + p.y)
            }else {    
                (lm.x as f32, lm.y as f32)
            };

            let end = (
                start.0 + (lm.rot.sin() * lm.length) as f32 ,
                start.1 + (lm.rot.cos() * lm.length) as f32,
            );

            draw_line_segment_mut(&mut nxt, start, end, col);
            parent_list.insert(&limb.name, LimbEndPoint::new(end.0, end.1));
        } else {
            let mut prev_limb = AnimationLimbMoveDetails::default();
            let mut next_limb = AnimationLimbMoveDetails::default();
            let mut found = false;
            for (idx, limb_move) in limb.animation_limb_moves.iter().enumerate() {
                if idx > 0 {
                    if prev_limb.perc < perc && perc <= limb_move.perc {
                        next_limb = *limb_move;
                        found = true;
                        break;
                    }
                }
                prev_limb = *limb_move;
            }

            if found {
                // Render the frame
                let adjust_perc = (perc - prev_limb.perc) / (next_limb.perc - prev_limb.perc);
                let x =  prev_limb.x + (adjust_perc * (next_limb.x - prev_limb.x));
                let y = prev_limb.y + (adjust_perc * (next_limb.y - prev_limb.y));
                let rot = prev_limb.rot + (adjust_perc * (next_limb.rot - prev_limb.rot));
                let length = prev_limb.length + (adjust_perc * (next_limb.length - prev_limb.length));
                
                let start = if limb.parent != "".to_string() {
                    let p = parent_list.get(&limb.parent).unwrap();
                    (x as f32 + p.x, y as f32 + p.y)
                }else {    
                    (x as f32, y as f32)
                };
                let end = (
                    start.0 + (rot.sin() * length) as f32 ,
                    start.1 + (rot.cos() * length) as f32,
                );

                draw_line_segment_mut(&mut nxt, start, end, col);
                parent_list.insert(&limb.name, LimbEndPoint::new(end.0, end.1));
            }

        }
    }
 
    let mut bytes: Vec<u8> = Vec::new();
    nxt.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png
    ).unwrap();

    Ok(Box::new(bytes))
}

pub fn crop_video_frame(vid_guid: String, frame_id: i32, offset_x: i32, offset_y: i32, width: i32, height: i32) -> Result<Box<Vec<u8>>, ImageRenderError> {
    // TODO: Proper checks to make sure no out of bounds and proper error formatting
    let filename = format!("./files/videos/processed/{}/img{:0>4}.png", vid_guid, frame_id);

    let im = image::open(&filename).unwrap().to_rgba8();

    let background_color = Rgba([0, 0, 0, 255]);
    
    let mut ret: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::from_pixel(width as u32, height as u32, background_color);
    
    for x in 0..width as u32 {
        for y in 0..height as u32 {
            let pix = im.get_pixel(offset_x as u32  + x, offset_y as u32 + y);
            ret.put_pixel(x, y, *pix);
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    ret.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png
    ).unwrap();

    Ok(Box::new(bytes))
}