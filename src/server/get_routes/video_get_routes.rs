use std::{fs::{self, File}, io::{BufReader, Read}, path::Path};
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::SqlitePool;

use crate::{db::video_models::VideoModel, server::query_params::VideoFrameQuery, video::video_utils::get_image_count_for_video};


pub(super) async fn make_routes(_db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
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

    // GET /video_view/<guid> - Get the video view page for given guid
    let video_view_create_page = warp::path!("video_view" / String).map( |guid: String| {
        let body: String = fs::read_to_string("templates/video/view_create.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_3", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_3", &json!({"guid":guid})).unwrap()
        )
    });

    // GET /api/video - get a list of the videos available
    let get_video_list = warp::path!("api" / "video")
        .and_then(get_video_list_impl);

    // GET /api/frame/<guid>/<frame_count> - return index of ids of images that fit within range (evenly sampled)
    let get_frame_indexes = warp::path!("api" / "frames" / String / usize)
        .and(warp::filters::query::raw()
               .or(warp::any().map(|| String::default()))
               .unify()
           )
        .and_then(get_frame_indexes_impl);

    // GET /img/frame/<guid>/<frame>
    let get_video_frame = warp::path!("img" / "videoframe" / String / i32)
        .and_then(get_video_frame_impl);

    video_upload_page
        .or(video_listing_page)
        .or(video_menu_page)
        .or(video_view_create_page)
        .or(get_video_list)
        .or(video_upload_page)
        .or(get_frame_indexes)
        .or(get_video_frame)
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

const MAX_FRAME_LIMIT: usize = 30;

async fn get_frame_indexes_impl(guid: String, max_frames: usize, query_string: String,) -> Result<Box<dyn Reply>, Rejection> {
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

    let vid_query = VideoFrameQuery::from_query(frame_count, query_string);    
    let max_frames = if max_frames > MAX_FRAME_LIMIT {
        MAX_FRAME_LIMIT
    } else {
        max_frames
    };

    let mut frames = Vec::<usize>::new();
    if vid_query.diff != 0 && vid_query.diff < max_frames {
        frames = (vid_query.start..vid_query.end).collect();
    } else if max_frames + vid_query.start >= frame_count {
        frames = (vid_query.start..frame_count).collect();
    } else if frame_count > 0 && vid_query.diff > max_frames{
        let step = vid_query.diff as f64 / max_frames as f64;
        let mut cur = vid_query.start as f64;
        while cur < vid_query.diff as f64 {
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

async fn get_video_frame_impl(guid: String, frame: i32) -> Result<Box<dyn Reply>, Rejection> {
    let filename = format!("./files/videos/processed/{}/img{:0>4}.png", guid, frame);
    let path = Path::new(&filename);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut bytes: Vec<u8> = Vec::new();
    reader.read_to_end(&mut bytes).unwrap();
    // image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();
    Ok(
        Box::new(
            warp::reply::with_header(bytes, "Content-Type", "image/png")
        )
    )
}