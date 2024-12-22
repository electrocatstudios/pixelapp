use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::animation_queries;
use crate::image::{gif, png};


pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    // Get /video_upload - the video upload page
    let video_upload_page = warp::path("video_upload").map( || {
        let body: String = fs::read_to_string("templates/video/video_upload.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
    });

    // Get /video - the video menu
    let video_menu_page = warp::path("video").map( || {
        let body: String = fs::read_to_string("templates/video/video_menu.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
    });

    // Get /video_listing - the video listing page - show all videos
    let video_listing_page = warp::path("video_listing").map( || {
        let body: String = fs::read_to_string("templates/video/video_listing.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
    });

    // GET /api/video - get a list of the videos available
    let get_video_list = warp::path!("api" / "video")
        .and_then(get_video_list_impl);

    video_upload_page
        .or(video_listing_page)
        .or(video_menu_page)
        .or(get_video_list)
        .or(video_upload_page)
        .boxed()
}

async fn get_video_list_impl() -> Result<Box<dyn Reply>, Rejection> {
    let paths = fs::read_dir("./files/videos/processed").unwrap();

    for path in paths {
        match path.as_ref().unwrap().file_type() {
            Ok(ft) => {
                if ft.is_dir() {
                    println!("Name: {}", path.unwrap().path().display())
                }
            },
            Err(_) => {}
        }        
    }
    
    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "videos": []}))
        )
    )
}