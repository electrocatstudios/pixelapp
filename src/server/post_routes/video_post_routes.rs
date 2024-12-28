use serde_json::json;
use futures::{StreamExt, TryStreamExt};
// use std::convert::Infallible;
use std::fs;

use bytes::BufMut;
use uuid::Uuid;
use warp::{filters::{multipart::FormData, BoxedFilter}, Filter, Rejection, Reply};
use sqlx::{SqlitePool, Pool, Sqlite};
use std::thread;

use crate::{db::{video_models::ViewCreateDesc, video_queries}, image::png, video::{proc::process_pending_videos_into_frames, VideoUploadDetails}};

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    // POST /api/video_upload
    let video_upload = warp::post()
        .and(warp::path!("api" / "video_upload"))
        .and(warp::multipart::form().max_length(20_000_000))
        .and_then(video_upload_impl);


    let view_create = warp::post()
        .and(warp::path!("api" / "view_create"))
        .and(json_body_view_create())
        .and(db_conn.clone())
        .and_then(view_create_impl);

    video_upload
        .boxed()
}

// --- JSON Parsers
fn json_body_view_create() -> impl Filter<Extract = (ViewCreateDesc,), Error = warp::Rejection> + Clone {
    // Perform coercion of incoming data into a AnimationDesc
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}
// --- End JSON Parsers

async fn video_upload_impl(form: FormData) -> Result<Box<dyn Reply>, Rejection> {

    // println!("We are in the video upload func");
    let mut parts = form.into_stream();
    let uuid = Uuid::new_v4();
    let mut video_upload_details = VideoUploadDetails::new(format!("{}",uuid.clone()));

    while let Some(Ok(p)) = parts.next().await {
        if p.name() == "file" {
            // println!("Processing file component");
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "video/mp4" => {
                        file_ending = "mp4";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            match fs::create_dir_all("./files/videos/ready") {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Error creating folder {}", err);
                    return Err(warp::reject::reject());
                }
            }

            let file_name = format!("./files/videos/ready/{}.{}", uuid.clone(), file_ending);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;

        } else if p.name() == "name" {
            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;
            let filename = String::from_utf8(value).expect("stdout bytes not valid utf-8");
            video_upload_details.name = filename;
        } else if p.name() == "description" {
            let value = p
            .stream()
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await
            .map_err(|e| {
                eprintln!("reading file error: {}", e);
                warp::reject::reject()
            })?;
            let description = String::from_utf8(value).expect("stdout bytes not valid utf-8");
            video_upload_details.description = description;
        }
    }

    // Save the video upload details
    video_upload_details.save();

    // Finally launch the processor to pick up the file
    thread::spawn(move || process_pending_videos_into_frames());

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "file_id": uuid.to_string()}))
        )
    )
}

async fn view_create_impl(vcd: ViewCreateDesc, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    // Create new view in db
    let vid_id = match video_queries::create_new_view(vcd.name, vcd.guid.clone(), &mut db_pool.clone()).await {
        Ok(id) => id,
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    };
    
    // let vid_guid = vcd.guid.clone();
    // Cycle through each frame and cut out the piece described - then store in db
    for f in vcd.frames.iter() {
        // Get the image data for the frame as vec<u8>
        // vid_guid: String, frame_id: i32, offset_x: i32, offset_y: i32, width: i32, height: i32
        let img_data = match png::crop_video_frame(
                        vcd.guid.clone().clone(),
                                f.frame_id,
                                f.x,
                                f.y, 
                                f.width, 
                                f.height
                            ){
                                Ok(i) => i,
                                Err(err) => {
                                    return Ok(
                                        Box::new(
                                            warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                                        )
                                    )
                                }
                            };
        
        match video_queries::create_new_frame(vid_id, f, *img_data, &mut db_pool.clone() ).await {
            Ok(_) => {},
            Err(err) => {
                return Ok(
                    Box::new(
                        warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                    )
                )
            }
        }
    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": ""}))
        )
    )
}