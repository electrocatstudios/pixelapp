use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::{db::video_models::VideoModel, video::video_utils::get_image_count_for_video};


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

    // GET /api/frame/<guid>/<frame_count> - return index of ids of images that fit within range (evenly sampled)
    let get_frame_indexes = warp::path!("api" / "frames" / String / i32)
        .and_then(get_frame_indexes_impl);

    video_upload_page
        .or(video_listing_page)
        .or(video_menu_page)
        .or(get_video_list)
        .or(video_upload_page)
        .or(get_frame_indexes)
        .boxed()
}

async fn get_video_list_impl() -> Result<Box<dyn Reply>, Rejection> {
    let paths = fs::read_dir("./files/videos/processed").unwrap();
    // println!("Inside the get video list dir");
    let mut ret = Vec::<VideoModel>::new();
    for path in paths {
        match path.as_ref().unwrap().file_type() {
            Ok(ft) => {
                if ft.is_dir() {
                    let filename = path.unwrap().file_name().into_string().unwrap();
                    match VideoModel::from_guid(filename) {
                        Ok(vm) => ret.push(vm),
                        Err(err) => println!("Error getting video details: {}", err)
                    }
                }
            },
            Err(_) => {}
        }        
    }
    
    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "videos": ret}))
        )
    )
}

const MAX_FRAME_LIMIT: i32 = 50;

async fn get_frame_indexes_impl(guid: String, max_frames: i32) -> Result<Box<dyn Reply>, Rejection> {
    let frame_count = match get_image_count_for_video(guid.clone()) {
        Ok(c) => c,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err, "frames": [], "frame_count": 0 }))
                )
            )
        }
    };

    let max_frames = if max_frames > MAX_FRAME_LIMIT {
        MAX_FRAME_LIMIT
    } else {
        max_frames
    };

    let mut frames = Vec::<usize>::new();
    if max_frames >= frame_count as i32 {
        frames = (1..frame_count).collect();
    } else if frame_count > 0 {
        let step = frame_count as f64 / max_frames as f64;
        let mut cur = 1.0;
        while cur < frame_count as f64 {
            frames.push(cur as usize);
            cur += step;
        }
    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "frames": frames, "frame_count": frame_count}))
        )
    )
}