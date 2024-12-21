use serde_json::json;
use futures::{StreamExt, TryStreamExt};
// use std::convert::Infallible;
use std::fs;

use bytes::BufMut;
use uuid::Uuid;
use warp::{filters::{multipart::FormData, BoxedFilter}, Filter, Rejection, Reply};
use sqlx::{SqlitePool, Pool, Sqlite};

use crate::{db::{animation_models::{AnimationDesc, AnimationSaveDesc, AnimationUpdateDesc}, animation_queries}, video::VideoUploadDetails};
use crate::video::proc::process_pending_videos_into_frames;
use std::thread;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {


    // POST /api/animation_new - create new animation
    let create_new_animation = warp::post()
        .and(warp::path!("api" / "animation_new"))
        .and(json_body_new_animation())
        .and(db_conn.clone())
        .and_then(create_new_animation_impl);

    // POST /api/animation_save - save animation limb data
    let save_animation_limbs = warp::post()
        .and(warp::path!("api" / "animation_save"))
        .and(json_body_save_animation())
        .and(db_conn.clone())
        .and_then(save_animation_limbs_impl);

    
    // POST /api/animation_details/<guid> - update animation based on new details
    let update_animation_details = warp::post()
        .and(warp::path!("api" / "animation_details" / String))
        .and(json_body_update_animation())
        .and(db_conn.clone())
        .and_then(update_animation_impl);

    // POST /api/video_upload
    let video_upload = warp::post()
        .and(warp::path!("api" / "video_upload"))
        .and(warp::multipart::form().max_length(20_000_000))
        .and_then(video_upload_impl);

    create_new_animation
        .or(save_animation_limbs)
        .or(update_animation_details)
        .or(video_upload)
        .boxed()
}

// -- JSON Parsers
fn json_body_new_animation() -> impl Filter<Extract = (AnimationDesc,), Error = warp::Rejection> + Clone {
    // Perform coercion of incoming data into a AnimationDesc
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn json_body_save_animation() -> impl Filter<Extract = (AnimationSaveDesc,), Error = warp::Rejection> + Clone {
    // Perform coercion of incoming data into a AnimationDesc
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn json_body_update_animation() -> impl Filter<Extract = (AnimationUpdateDesc,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// -- Route implementations

// Create the new pixel from the components
async fn create_new_animation_impl(anim: AnimationDesc, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let anim_id = match animation_queries::create_new_animation(anim, &mut db_pool.clone()).await {
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
            warp::reply::json(&json!({"status": "ok", "message": "", "animationid": anim_id}))
        )
    )
}

async fn save_animation_limbs_impl(save_desc: AnimationSaveDesc, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let anim = match animation_queries::get_animation_from_guid(save_desc.guid, &mut db_pool.clone()).await {
        Ok(anim) => {anim},
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    };
    match animation_queries::update_limbs_for_animation(anim.id, save_desc.limbs, &mut db_pool.clone()).await {
        Ok(_) => {       
            Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "ok", "message": ""}))
                )
            )
        },
        Err(err) => {
            Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    }   
}

async fn update_animation_impl(guid: String, aus: AnimationUpdateDesc, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let anim = match animation_queries::get_animation_from_guid(guid, &mut db_pool.clone()).await {
        Ok(anim) => {anim},
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    };

    match animation_queries::update_animation_details(anim.id, aus, &mut db_pool.clone()).await {
        Ok(_) => {},
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    }

    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": ""}))
        )
    )
}

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