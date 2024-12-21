use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::animation_queries;
use crate::image::{gif, png};

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    // Get /animation_new - get the page for creating a new pixel image
    let new_animation_page = warp::path("animation_new").map(|| {
        let body: String = fs::read_to_string("templates/setupanimation.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // Get /load
    let load_animation_page = warp::path("load_animation").map(|| {
        let body: String = fs::read_to_string("templates/saved_animations.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
    });

    // Get /video_upload - the video upload page
    let video_upload_page = warp::path("video_upload").map( || {
        let body: String = fs::read_to_string("templates/video_upload.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
   
    });

    
    // Get /animation/<guid> - get the page for an existing animation
    let animation_page = warp::path!("animation" / String)
        .and(db_conn.clone())
        .and_then(render_animation_page);


    // GET /api/animation - get the list of animations
    let get_animation_list = warp::path!("api" / "animation")
        .and(db_conn.clone())
        .and_then(get_animation_list_impl);

    // GET /api/animation - get details about individual animation
    let get_animation = warp::path!("api" / "animation" / String)
        .and(db_conn.clone())
        .and_then(get_animation_impl);

    // GET /render/<guid> - get the output sprite sheet
    let get_gif_render = warp::path!("img" / "animation_gif" / String)
        .and(warp::get())
        .and(db_conn.clone())
        .and_then(get_animationgif_render_impl);

    // GET /img/animation_render/<guid>/<frame>/<total_frames>
    let get_image_render_single = warp::path!("img" / "animation_render" / String / u32 / u32)
        .and(db_conn.clone())
        .and_then(get_image_animationrender_single_impl);

    new_animation_page
        .or(get_animation_list)
        .or(get_animation)
        .or(load_animation_page)
        .or(animation_page)
        .or(get_gif_render)
        .or(get_image_render_single)
        .or(video_upload_page)
        .boxed()
}


async fn get_animation_list_impl(db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let rows = match animation_queries::get_animation_list(&mut db_pool.clone()).await {
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
            warp::reply::json(&json!({"status": "ok", "message": "", "animations": rows}))
        )
    )
}

async fn get_animation_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let animation = match animation_queries::get_animation_details_from_guid(guid, &mut db_pool.clone()).await {
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
            warp::reply::json(&json!({"status": "ok", "message": "", "animation": animation}))
        )
    )
}

async fn render_animation_page(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let body: String = fs::read_to_string("templates/animation.html").unwrap().parse().unwrap();
    let page_json = match animation_queries::get_animation_details_as_json(guid, &mut db_pool.clone()).await {
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

async fn get_animationgif_render_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {

    // let bytes: Vec<u8> = match gif::render_animationgif(guid, db_pool).await {
    let bytes: Vec<u8> = match gif::render_animationgif(guid, db_pool).await {
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

// GET /img/animation_render/<guid>/<frame>/<total_frames>
async fn get_image_animationrender_single_impl(guid: String, frame: u32, total_frames: u32, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let animation = match animation_queries::get_animation_details_from_guid(guid.clone(), &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_) => {
            return Err(
                warp::reject::not_found()
            )
        }
    };

    let perc = frame as f64 / total_frames as f64;
    let ret = match png::render_animation_png(animation, perc){
        Ok(img) => img,
        Err(err) => {
            log::error!("Error rendering png image {}", err.to_string());
            return Err(warp::reject::not_found())
        }
    };

    Ok(
        Box::new(
            warp::reply::with_header(ret, "Content-Type", "image/png")
        )
    )
}