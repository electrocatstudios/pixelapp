use std::fs;
use serde_json::json;
use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Sqlite,SqlitePool,Pool};

use image::{RgbaImage,Rgba};
use image::io::Reader as ImageReader;

use crate::db::queries;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
    .allow_any_origin().allow_methods(&[warp::http::Method::GET, warp::http::Method::POST]);

    let heartbeat = warp::path!("heartbeat")
        .map(|| format!("ok"))
        .with(cors);
    
    // GET 404 - catch all
    let default = warp::any().map(|| {
        let body: String = fs::read_to_string("templates/404.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // GET /home - get the main front page
    let home = warp::path::end().map(|| {
        let body: String = fs::read_to_string("templates/index.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // Get /new - get the page for creating a new pixel image
    let new_image_page = warp::path("new").map(|| {
        let body: String = fs::read_to_string("templates/setuppixel.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // Get /load
    let load_image_page = warp::path("load").map(|| {
        let body: String = fs::read_to_string("templates/saved.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
    });

    // GET /pixel/<guid> - get the page for an existing pixel
    let pixel_page = warp::path!("pixel" / String)
        .and(db_conn.clone())
        .and_then(render_pixel_page);

    // // GET /api/pixel - get list of pixels
    let get_pixel_list_route = warp::get()
        .and(warp::path!("api" / "pixel"))
        .and(db_conn.clone())
        .and_then(get_pixel_list);

    let get_image_render = warp::path!("img" / String / String)
        .and(db_conn.clone())
        .and_then(get_image_rendered);
        // .and_then(|pathname| async move {warp::fs::file(pathname)});

    // GET /js/<file> - get named js file
    let get_js = warp::path("js").and(warp::fs::dir("./assets/js/"));
    // GET /css/<file> - get named css file
    let get_css = warp::path("css").and(warp::fs::dir("./assets/css/"));
    // GET /font/<file> - get named font file
    let get_font = warp::path("font").and(warp::fs::dir("./assets/fonts/"));
    // GET /img/<file> - get named img file
    let get_img = warp::path("img").and(warp::fs::dir("./assets/fonts/"));
    

    heartbeat 
        .or(get_img)
        .or(get_js)
        .or(get_css)
        .or(get_font)
        .or(new_image_page)
        .or(load_image_page)
        .or(get_image_render)
        .or(pixel_page)
        .or(get_pixel_list_route)
        .or(home)
        .or(default)
        .boxed()
}

async fn render_pixel_page(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let body: String = fs::read_to_string("templates/pixel.html").unwrap().parse().unwrap();
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

async fn get_image_rendered(guid: String, image_type: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let pixel = match queries::get_pixel_details(guid, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

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
        for x in 0..pixel.pixelwidth {
            for y in 0..pixel.pixelwidth {
                let offset_x = pix.x * pixel.pixelwidth;
                let offset_y = pix.y * pixel.pixelwidth;
                image.put_pixel( 
                    (offset_x + x) as u32, 
                    (offset_y + y) as u32, 
                    Rgba([pix.r as u8, pix.g as u8, pix.b as u8, pix.alpha as u8])
                );
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