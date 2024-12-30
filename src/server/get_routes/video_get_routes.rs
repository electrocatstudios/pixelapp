use std::{fs::{self, File}, io::{BufReader, Read}, path::Path};
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::{db::{video_models::{VideoModel, ViewList}, video_queries}, server::query_params::VideoFrameQuery, video::video_utils::get_image_count_for_video};


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

    // GET /video_view/<guid> - Get the video view page for given guid
    let video_view_create_page = warp::path!("video_view" / String).map( |guid: String| {
        let body: String = fs::read_to_string("templates/video/view_create.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_3", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_3", &json!({"guid":guid})).unwrap()
        )
    });

    // GET /view_preview/<guid> - Get a preview of the view - the host page
    let view_preview_page = warp::path!("view_preview" / String)
        .and(db_conn.clone())
        .and_then(get_view_preview_page_impl);

    let view_listing_page = warp::path("view_listing").map( || {
        let body: String = fs::read_to_string("templates/video/view_listing.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_2", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_2", &json!({})).unwrap()
        )
    });

    // GET /api/video - get a list of the videos available
    let get_video_list = warp::path!("api" / "video")
        .and_then(get_video_list_impl);

    // GET /api/view - get a list of views available
    let get_view_list = warp::path!("api" / "view")
        .and(db_conn.clone())
        .and_then(get_view_list_impl);

    // GET /api/view/<guid> - get list of frames attached to view
    let get_view_details = warp::path!("api" / "view" / String)
        .and(db_conn.clone())
        .and_then(get_view_details_impl);

    // GET /api/frames/<guid>/<frame_count> - return index of ids of images that fit within range (evenly sampled)
    let get_frame_indexes = warp::path!("api" / "frames" / String / usize)
        .and(warp::filters::query::raw()
               .or(warp::any().map(|| String::default()))
               .unify()
           )
        .and_then(get_frame_indexes_impl);

    // GET /api/view_dim - get the dimensions of the frames
    let get_view_dimensions = warp::path!("api" / "view_dim" / String)
           .and(db_conn.clone())
           .and_then(get_view_dimensions_impl);

    // GET /img/frame/<guid>/<frame>
    let get_video_frame = warp::path!("img" / "videoframe" / String / i32)
        .and_then(get_video_frame_impl);

    // GET /img/viewframe/<guid>/<frame> - pass in VIEW guid and the frame - get an image back
    let get_view_frame = warp::path!("img" / "viewframe" / String / i32)
        .and(db_conn.clone())
        .and_then(get_view_frame_impl);

    video_upload_page
        .or(video_listing_page)
        .or(view_listing_page)
        .or(video_menu_page)
        .or(video_view_create_page)
        .or(view_preview_page)
        .or(get_video_list)
        .or(video_upload_page)
        .or(get_frame_indexes)
        .or(get_video_frame)
        .or(get_view_frame)
        .or(get_view_list)
        .or(get_view_details)
        .or(get_view_dimensions)
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

async fn get_view_details_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let view = match video_queries::get_view_details(guid.clone(), &mut db_pool.clone()).await {
        Ok(view) => view,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string(), "frames": []}))
                )
            )
        }
    };

    let mut frames = Vec::<i32>::new();
    for f in view.frames.iter() {
        frames.push(f.frame_id);
    }
    
    return Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "frames": frames}))
        )
    )
}

const MAX_FRAME_LIMIT: usize = 50;

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
    
    // Debug 
    // println!("vid query {} -> {} [{}/{}]", vid_query.start, vid_query.end, vid_query.diff, max_frames);
    
    let mut frames = Vec::<usize>::new();
    if vid_query.diff != 0 && vid_query.diff <= max_frames {
        frames = (vid_query.start..vid_query.end).collect();
    } else if max_frames + vid_query.start >= frame_count {
        frames = (vid_query.start..frame_count).collect();
    } else if frame_count > 0 && vid_query.diff > max_frames{
        let step = vid_query.diff as f64 / max_frames as f64;
        let mut cur = vid_query.start as f64;
        while cur < vid_query.end as f64 {
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

async fn get_view_dimensions_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    // Find view from string
    // return dimensions of first frame
    let view = match video_queries::get_view_from_guid(guid, &mut db_pool.clone()).await {
        Ok(v) => v,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string(), "width": 0, "height": 0}))
                )
            )
        }
    };
    let frame = match video_queries::get_view_frame_with_video_id(view.id, -1, &mut db_pool.clone()).await {
        Ok(f) => f,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string(), "width": 0, "height": 0}))
                )
            )
        }
    };
    let width = frame.width;
    let height = frame.height;
    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "width": width, "height": height}))
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

async fn get_view_frame_impl(guid: String, frame: i32, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let img_data = match video_queries::get_image_view_image_data(guid, frame, &mut db_pool.clone()).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(warp::reject::not_found())
        }
    };

    Ok(
        Box::new(
            warp::reply::with_header(img_data, "Content-Type", "image/png")
        )
    )
}

async fn get_view_list_impl(db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let video_views = match video_queries::get_view_list(&mut db_pool.clone()).await {
        Ok(v) => v,
        Err(err) => {
            return  Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string(), "views": []}))
                )
            )
        }
    };
    let mut ret = Vec::<ViewList>::new();

    for vv in video_views.iter() {
        ret.push(ViewList{
            name: vv.name.clone(),
            guid: vv.guid.clone()
        });
    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "views": ret}))
        )
    )
}

async fn get_view_preview_page_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let body: String = fs::read_to_string("templates/video/view_preview.html").unwrap().parse().unwrap();
    let (width,height) = match video_queries::get_dimensions_of_view(guid.clone(), &mut db_pool.clone()).await {
        Ok((w,h)) => (w,h),
        Err(err) => {
            log::error!("Error occurred getting page {}", err.to_string());
            (100, 100)
        } 
    };
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("tpl_3", body).unwrap();
    Ok(
        Box::new(
            warp::reply::html(
                handlebars.render("tpl_3", &json!({"guid":guid, "width": width, "height": height})).unwrap()
            )
        )
    )
}
