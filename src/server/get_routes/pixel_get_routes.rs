use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Sqlite,SqlitePool,Pool};

use image::{Pixel, Rgba, RgbaImage};
use image::io::Reader as ImageReader;

use crate::db::models::{PixelShading, PixelSaveFile};
use crate::db::{queries, models::{PixelPixel,IncomingPixel,IncomingShader}};
use crate::utils::{color_to_hex_string,hex_string_to_color};
use crate::image::gif;

use crate::server::query_params::{RenderQuery, GifRenderQuery};

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
   
    // GET /home - get the main front page
    let home = warp::path::end().map(|| {
        let body: String = fs::read_to_string("templates/index.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // GET /menu - get the pixel menu
    let get_pixel_menu = warp::path("menu").map(|| {
        let body: String = fs::read_to_string("templates/pixel/pixel_menu.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // Get /new - get the page for creating a new pixel image
    let new_image_page = warp::path("new").map(|| {
        let body: String = fs::read_to_string("templates/pixel/setuppixel.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // GET /new_collection - get the new collection page
    let new_collection_page = warp::path("new_collection").map(|| {
        let body: String = fs::read_to_string("templates/pixel/newcollection.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // Get /load
    let load_image_page = warp::path("load").map(|| {
        let body: String = fs::read_to_string("templates/pixel/saved.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
    });

    // GET /render/<guid>
    let load_render_page = warp::path!("render" / String).map(|guid: String| {
        let body: String = fs::read_to_string("templates/pixel/render.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_3", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_3", &json!({"guid":guid})).unwrap()
        )
    });

    // GET /pixel/<guid> - get the page for an existing pixel
    let pixel_page = warp::path!("pixel" / String)
        .and(db_conn.clone())
        .and_then(render_pixel_page);

    // GET /api/pixel - get list of pixels
    let get_pixel_list_route = warp::get()
        .and(warp::path!("api" / "pixel"))
        .and(db_conn.clone())
        .and_then(get_pixel_list);

    // GET /img/<guid>/<type> - return the image rendered as an image of <type>
    let get_image_render = warp::path!("img" / String / String)
        .and(db_conn.clone())
        .and_then(get_image_rendered);

    // GET /img/render/<guid>/<frame>/<direction>/<angle>/<flip>
    let get_image_render_single = warp::path!("img" / "render" / String / u32 / String / u32 / bool)
        .and(db_conn.clone())
        .and_then(get_image_render_single_impl);
    
    // GET /render/<guid> - get the output sprite sheet
    let get_rendered_spritesheet = warp::path!("img" / "spritesheet" / String)
        .and(warp::get())
        .and(warp::filters::query::raw()
                .or(warp::any().map(|| String::default()))
                .unify()
            )
        .and(db_conn.clone())
        .and_then(get_rendered_spritesheet_impl);

    // GET /render/<guid> - get the output sprite sheet
    let get_gif_render = warp::path!("img" / "gif" / String)
       .and(warp::get())
       .and(warp::filters::query::raw()
               .or(warp::any().map(|| String::default()))
               .unify()
           )
       .and(db_conn.clone())
       .and_then(get_gif_render_impl);

    // GET /api/<guid> - return list of pixels for image
    let get_image_pixel_list = warp::path!("api" / String)
        .and(db_conn.clone())
        .and_then(get_image_pixel_list);

    // GET /api/details/<guid> - get information about pixel (width, height etc)
    let get_pixel_details = warp::path!("api" / "details" / String)
        .and(db_conn.clone())
        .and_then(get_pixel_details_impl);

    // GET /api/shader/<guid>
    let get_image_shader_list = warp::path!("api" / "shader" / String)
        .and(db_conn.clone())
        .and_then(get_image_shader_list);

    // GET /api/info/<guid> - fetch information about rendering frames
    let get_render_info = warp::path!("api" / "info" / String)
        .and(db_conn.clone())
        .and_then(get_render_info_impl);

    // GET /api/saveasfile
    let get_save_file = warp::path!("api" / "saveasfile" / String)
        .and(db_conn.clone())
        .and_then(get_save_file_impl);
        
    // GET /api/collection - get the list of collections
    let get_collection_list = warp::path!("api" / "collection")
        .and(db_conn.clone())
        .and_then(get_collection_list_impl);
 
    // GET /js/<file> - get named js file
    let get_js = warp::path("js").and(warp::fs::dir("./assets/js/"));
    // GET /css/<file> - get named css file
    let get_css = warp::path("css").and(warp::fs::dir("./assets/css/"));
    // GET /font/<file> - get named font file
    let get_font = warp::path("font").and(warp::fs::dir("./assets/fonts/"));
    // GET /img/<file> - get named img file
    let get_img = warp::path("img").and(warp::fs::dir("./assets/img/"));
    

    get_img 
        .or(get_pixel_menu)
        .or(get_js)
        .or(get_css)
        .or(get_font)
        .or(new_image_page)
        .or(load_image_page)
        .or(get_image_render)
        .or(get_image_render_single)
        .or(load_render_page)
        .or(pixel_page)
        .or(new_collection_page)
        .or(get_pixel_list_route)
        .or(get_pixel_details)
        .or(get_image_pixel_list)
        .or(get_image_shader_list)
        .or(get_collection_list)
        .or(get_rendered_spritesheet)
        .or(get_save_file)
        .or(get_gif_render)
        .or(get_render_info)
        .or(home)
        .boxed()
}

async fn render_pixel_page(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let body: String = fs::read_to_string("templates/pixel/pixel.html").unwrap().parse().unwrap();
    let page_json = match queries::get_pixel_details_as_json(guid, &mut db_pool.clone()).await {
        Ok(page_json) => page_json,
        Err(err) => {
            log::error!("{}", err);
            return  Err(warp::reject());
        }
    };
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("tpl_1", body).unwrap();
    Ok(
        Box::new(
            warp::reply::html(
                handlebars.render("tpl_1",  &page_json).unwrap()
            )
        )
    )
}

async fn get_pixel_list(db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let rows = match queries::get_pixel_list(&mut db_pool.clone()).await {
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
            warp::reply::json(&json!({"status": "ok", "message": "", "pixelimages": rows}))
        )
    )
} 

async fn get_collection_list_impl(db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    match queries::get_collections(&mut db_pool.clone()).await {
        Ok(collections) => Ok(
            Box::new(
                warp::reply::json(&json!({"status": "ok", "message": "", "collections": collections}))
            )
        ),
        Err(err) => {
            Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    }
}


async fn get_pixel_details_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };
    Ok(
        Box::new(
            warp::reply::json(&json!(
                {
                    "status": "ok", 
                    "message": "", 
                    "width": pixel.width, 
                    "height": pixel.height, 
                    "pixel_size": pixel.pixelwidth, 
                    "name": pixel.name, 
                    "description": pixel.description
                }
            ))
        )
    )
}

async fn get_image_rendered(guid: String, image_type: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };
    // log::info!("For guid {} we found image with id {}", guid, pixel.id);

    // Get pixels and render to image - type of which is defined by image_type
    if image_type != "png"{
        // Currently only png supported
        return Err(warp::reject::not_found())
    }

    let pixels = match queries::get_pixels_for_image(pixel.id, 0, 1, &mut db_pool.clone()).await {
        Ok(pixels) => pixels,
        Err(err) => {
            log::error!("Error finding pixels {}", err);
            return Err(warp::reject::not_found())
        }
    };

    // log::info!("Found {} pixels during render", pixels.len());

    if pixels.len() == 0 {
        log::error!("No pixels found for image");
        let img: RgbaImage = match ImageReader::open("./assets/img/notfound.png") {
            Ok(img) => match img.decode() {
                Ok(img) => img.into_rgba8(),
                Err(err)  => {
                    log::error!("Error decoding notfound image {}", err);
                    return Err(warp::reject::not_found())
                }
            },
            Err(err) => {
                log::error!("Error opening notfound image {}", err);
                return Err(warp::reject::not_found())
            }
        };
        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();
        return Ok(
            Box::new(
                warp::reply::with_header(bytes, "Content-Type", "image/png")
            )
        )
    }

    // TODO: Include last updated time in filename
    let pathname = "images/".to_owned() + pixel.guid.clone().as_str() + ".png";
    
    // TODO: Check if file already exists

    // TODO: Check last updated time and see if the file has changed and then re-render if necessary

    // Create new image, render the pixels, save as temp file
    let mut image: RgbaImage = RgbaImage::new(pixel.width as u32, pixel.height as u32);
    for pix in pixels.iter() {
        let mut color = Rgba([pix.r as u8, pix.g as u8, pix.b as u8, (pix.alpha * 255.0) as u8 ]);
        match queries::get_shader_for_image_at_point(pixel.id, 0, pix.x, pix.y, &mut db_pool.clone()).await {
            Ok(shad) => {
                // log::info!("Found a pixel shader");
                let shader_col = Rgba([shad.r as u8, shad.g as u8, shad.b as u8, (shad.alpha * 255.0) as u8]);
                color.blend(&shader_col);
            },
            Err(_) => {}
        };

        for x in 0..pixel.pixelwidth {
            for y in 0..pixel.pixelwidth {
                let offset_x = pix.x * pixel.pixelwidth;
                let offset_y = pix.y * pixel.pixelwidth;
                let nxt_x = (offset_x + x) as u32;
                let nxt_y = (offset_y + y) as u32;
                if nxt_x < image.width() && nxt_y < image.height() {
                    image.put_pixel( 
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

    match image.save(&pathname) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error saving image {}", err);
            return Err(warp::reject::not_found())
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();
    Ok(
        Box::new(
            warp::reply::with_header(bytes, "Content-Type", "image/png")
        )
    )
}

async fn get_rendered_spritesheet_impl(guid: String, query_string: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };
    
    let query = RenderQuery{query: Some(query_string)};
    let query_subs = query.get_color_subs();
    
    // log::info!("Rendering Image {}", guid);
    let frame_count = match queries::get_frame_count(pixel.id, &mut db_pool.clone()).await {
        Ok(count) => count,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };
    // log::info!("We have found {} frames", frame_count);

    // DEBUG
    // match queries::get_all_shaders_for_image(pixel.id, &mut db_pool.clone()).await {
    //     Ok(shad) => {
    //         log::info!("Found {} shading items", shad.len());
    //         for s in shad.iter(){
    //             log::info!("Shader {}", s);
    //         }
    //     },
    //     Err(err) => {
    //         log::info!("Error while getting shads {}", err.to_string());
    //     }
    // };
    // END DEBUG

    let pathname: String = "spritesheets/".to_owned() + pixel.guid.clone().as_str() + ".png";
    let mut image: RgbaImage = RgbaImage::new( (pixel.width * frame_count) as u32, pixel.height as u32);
    
    for frame in 0..frame_count {
        let pixels = match queries::get_pixels_for_image(pixel.id, frame, 1, &mut db_pool.clone()).await {
            Ok(pixels) => pixels,
            Err(err) => {
                log::error!("Error finding pixels {}", err);
                return Err(warp::reject::not_found())
            }
        };

        for pix in pixels.iter() {
            // Calculate color
            let col_hex = color_to_hex_string(pix.r as u8, pix.g as u8, pix.b as u8);
            // Check for a color substitution
            let mut color = match query_subs.get(&col_hex) {
                Some(new_col) => {
                    // log::info!("Subbing color: {} -> {}", new_col.clone(), col_hex);
                    let (r, g, b) = hex_string_to_color(new_col.to_string());
                    Rgba([r,g,b,(pix.alpha * 255.0) as u8])
                },
                None => Rgba([pix.r as u8, pix.g as u8, pix.b as u8, (pix.alpha * 255.0) as u8 ])
            };
            // Check if shader exists and mix it in, if so
            match queries::get_shader_for_image_at_point(pixel.id, pix.frame, pix.x, pix.y, &mut db_pool.clone()).await {
                Ok(shad) => {
                    // log::info!("Found a pixel shader");
                    let shader_col = Rgba([shad.r as u8, shad.g as u8, shad.b as u8, (shad.alpha * 255.0) as u8]);
                    color.blend(&shader_col);
                },
                Err(_) => {}
            };
            
            for x in 0..pixel.pixelwidth {
                for y in 0..pixel.pixelwidth {
                    let offset_x = pix.x * pixel.pixelwidth;
                    let offset_y = pix.y * pixel.pixelwidth;

                    let nxt_x = (offset_x + x + (frame * pixel.width)) as u32;
                    let nxt_y = (offset_y + y) as u32;
                    if nxt_x < image.width() && nxt_y < image.height() {
                        image.put_pixel( 
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
    }
    
    match image.save(&pathname) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error saving image {}", err);
            return Err(warp::reject::not_found())
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();
    Ok(
        Box::new(
            warp::reply::with_header(bytes, "Content-Type", "image/png")
        )
    )
}

async fn get_image_pixel_list(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    let ret_vec: Vec::<PixelPixel> = match queries::get_all_pixels_for_image(pixel.id, &mut db_pool.clone()).await {
        Ok(p) => p,
        Err(err) => {
            return  Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "ok", "message": err.to_string(), "pixels": Vec::<IncomingPixel>::new()}))
                )
            )
        }
    };
    log::info!("get_image_pixel_list num results {}", ret_vec.len());

    let mut pixels_out: Vec::<IncomingPixel> = Vec::new();
    for p in ret_vec {
        pixels_out.push(IncomingPixel{
            x: p.x,
            y: p.y,
            r: p.r,
            g: p.g,
            b: p.b,
            alpha: p.alpha,
            frame: p.frame
        });
    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "pixels": pixels_out}))
        )
    )
} 

async fn get_image_shader_list(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    let ret_vec: Vec::<PixelShading> = match queries::get_all_shaders_for_image(pixel.id, &mut db_pool.clone()).await {
        Ok(p) => p,
        Err(err) => {
            return  Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "ok", "message": err.to_string(), "pixels": Vec::<IncomingPixel>::new()}))
                )
            )
        }
    };

    let mut pixels_out: Vec::<IncomingShader> = Vec::new();
    for p in ret_vec {
        pixels_out.push(IncomingShader{
            x: p.x,
            y: p.y,
            r: p.r,
            g: p.g,
            b: p.b,
            alpha: p.alpha,
            frame: p.frame
        });
    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "shaders": pixels_out}))
        )
    )
}

async fn get_render_info_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    // framecount
    let pixel = match queries::get_pixel_details(guid, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };
    let frame_count = match queries::get_frame_count(pixel.id, &mut db_pool.clone()).await {
        Ok(count) => count,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    // colors 
    // color_to_hex_string
    let mut colors_out: Vec::<String> = Vec::new();
    for frame in 0..frame_count {
        let pixels = match queries::get_pixels_for_image(pixel.id, frame, 1, &mut db_pool.clone()).await {
            Ok(pixels) => pixels,
            Err(err) => {
                log::error!("Error finding pixels {}", err);
                return Err(warp::reject::not_found())
            }
        };
        for p in pixels.iter() {
            let nxt = color_to_hex_string(p.r as u8, p.g as u8, p.b as u8);
            if !colors_out.iter().any(|e| e == &nxt) {
                colors_out.push(nxt.clone());
            }
        }
    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "framecount": frame_count, "colors": colors_out}))
        )
    )
}

// /img/render/<guid>/<frame>/<direction>/<angle>/<flip>
async fn get_image_render_single_impl(guid: String, frame: u32, _direction: String, _angle: u32, flip: bool, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };
    // log::info!("For guid {} we found image with id {}", guid, pixel.id);

    let pixels = match queries::get_pixels_for_image(pixel.id, frame as i32, 1, &mut db_pool.clone()).await {
        Ok(pixels) => pixels,
        Err(err) => {
            log::error!("Error finding pixels {}", err);
            return Err(warp::reject::not_found())
        }
    };

    // log::info!("Found {} pixels during render", pixels.len());

    if pixels.len() == 0 {
        log::error!("No pixels found for image");
        let img: RgbaImage = match ImageReader::open("./assets/img/notfound.png") {
            Ok(img) => match img.decode() {
                Ok(img) => img.into_rgba8(),
                Err(err)  => {
                    log::error!("Error decoding notfound image {}", err);
                    return Err(warp::reject::not_found())
                }
            },
            Err(err) => {
                log::error!("Error opening notfound image {}", err);
                return Err(warp::reject::not_found())
            }
        };
        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();
        return Ok(
            Box::new(
                warp::reply::with_header(bytes, "Content-Type", "image/png")
            )
        )
    }

    // TODO: Include last updated time in filename
    let pathname = "images/".to_owned() + pixel.guid.clone().as_str() + ".png";
    
    // TODO: Check if file already exists

    // TODO: Check last updated time and see if the file has changed and then re-render if necessary

    // Create new image, render the pixels, save as temp file
    let mut image: RgbaImage = RgbaImage::new(pixel.width as u32, pixel.height as u32);
    log::info!("Image dim {}:{}", image.width(), image.height());

    for pix in pixels.iter() {
        let mut color = Rgba([pix.r as u8, pix.g as u8, pix.b as u8, (pix.alpha * 255.0) as u8 ]);         
        match queries::get_shader_for_image_at_point(pixel.id, frame as i32, pix.x, pix.y, &mut db_pool.clone()).await {
            Ok(shad) => {
                // log::info!("Found a pixel shader");
                let shader_col = Rgba([shad.r as u8, shad.g as u8, shad.b as u8, (shad.alpha * 255.0) as u8]);
                color.blend(&shader_col);
            },
            Err(_) => {}
        };
        for x in 0..pixel.pixelwidth {
            for y in 0..pixel.pixelwidth {
                let offset_x = pix.x * pixel.pixelwidth;
                let offset_y = pix.y * pixel.pixelwidth;
                let nxt_x = if flip {
                    ((image.width() as i32 - 1) - (offset_x + x)) as u32 
                } else {
                    (offset_x + x) as u32 
                };
                let nxt_y = (offset_y + y) as u32;
                if nxt_x < image.width() && nxt_y < image.height() {
                    image.put_pixel( 
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

    match image.save(&pathname) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error saving image {}", err);
            return Err(warp::reject::not_found())
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();
    Ok(
        Box::new(
            warp::reply::with_header(bytes, "Content-Type", "image/png")
        )
    )
}

async fn get_gif_render_impl(guid: String, query_string: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
       
    let query = GifRenderQuery::new(query_string);

    let bytes: Vec<u8> = match gif::render_gif(guid, query, db_pool).await {
        Ok(b) => *b,
        Err(err) => {
            log::error!("Error rendering gif: {}", err);
            return Err(warp::reject::not_found());
        }
    };
    
    Ok(
        Box::new(
            warp::reply::with_header(bytes, "Content-Type", "image/gif")
        )
    )
}

async fn get_save_file_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    // log::info!("Downloading save file impl");
    let pixel = match queries::get_pixel_details(guid, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    let pixel_list: Vec::<PixelPixel> = match queries::get_all_pixels_for_image(pixel.id, &mut db_pool.clone()).await {
        Ok(p) => p,
        Err(err) => {
            log::error!("Error while loading pixels during get_save_file {}", err.to_string());
            return Err(
                warp::reject::not_found()
            )
        }
    };

    let new_desc = match pixel.description {
        Some(desc) => desc,
        None => "".to_string()
    };

    let ret = PixelSaveFile{
        name: pixel.name.clone(),
        description: new_desc,
        width: pixel.width,
        height: pixel.height,
        pixelwidth: pixel.pixelwidth,
        pixels: pixel_list,
        shaders: Vec::<PixelShading>::new()
    };

    Ok(
        Box::new(
            warp::reply::json(&json!(ret))
        )
    )
}