
use std::vec;
use sqlx::Sqlite;
use sqlx::pool::Pool;
use uuid::Uuid; 

// use super::animation_models::{Animation, AnimationDesc, AnimationDetails, AnimationLimb, AnimationLimbDetails, AnimationLimbMove, AnimationSaveLimbDesc, AnimationUpdateDesc };
use super::video_models::{VideoView, VideoViewExt, VideoViewExtFrame, VideoViewFrame, ViewCreateDescFrame};
use super::DBError;


pub async fn get_view_list(pool: &mut Pool<Sqlite>) -> Result<vec::Vec::<VideoView>, DBError> {
    // Do the actual request to get the list
    match sqlx::query_as::<_,VideoView>(
        "SELECT * FROM video_view"
    ).fetch_all(&*pool).await {
        Ok(anims) => Ok(anims),
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_view_from_guid(guid: String, pool: &mut Pool<Sqlite>) -> Result<VideoView, DBError> {
    // Do the actual request to get the list
    match sqlx::query_as::<_,VideoView>(
        "SELECT * FROM video_view WHERE guid=$1"
    )
    .bind(guid)
    .fetch_one(&*pool).await {
        Ok(vv) => Ok(vv),
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_view_from_id(id: i32, pool: &mut Pool<Sqlite>) -> Result<VideoView, DBError> {
    // Do the actual request to get the list
    match sqlx::query_as::<_,VideoView>(
        "SELECT * FROM video_view WHERE id=$1"
    )
    .bind(id)
    .fetch_one(&*pool).await {
        Ok(vv) => Ok(vv),
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_view_frames_from_view_id(id: i32, pool: &mut Pool<Sqlite>) -> Result<Vec::<VideoViewFrame>, DBError> {
    match sqlx::query_as::<_,VideoViewFrame>(
        "SELECT * FROM video_view_frame WHERE video_view_id=$1"
    )
    .bind(id)
    .fetch_all(&*pool).await {
        Ok(vv) => Ok(vv),
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_view_details(guid: String, pool: &mut Pool<Sqlite>) -> Result<VideoViewExt, DBError> {
    let mut ret = VideoViewExt::default();
    let view = match get_view_from_guid(guid.clone(), &mut pool.clone()).await {
        Ok(v) => v,
        Err(err) => return Err(err)
    };

    ret.guid = view.guid;
    ret.name = view.name;

    // Get the frames and push them into return result
    let frames = match get_view_frames_from_view_id(view.id, &mut pool.clone()).await {
        Ok(f) => f,
        Err(err) => return Err(err)
    };
    for f in frames.iter() {
        ret.frames.push(VideoViewExtFrame::new_from_video_view_frame(f));
    }

    Ok(ret)
}

pub async fn create_new_view(name: String, video_id: String, pool: &mut Pool<Sqlite>) -> Result<i32, DBError> {
    let guid: String = format!("{:?}", Uuid::new_v4());

    // Returns the id of the newly created row
    let res = match sqlx::query(
        "INSERT INTO video_view(guid, video_guid, name) VALUES  \
            ($1, $2, $3)"
    )
        .bind(guid)
        .bind(video_id)
        .bind(name)
        .execute(&*pool).await {
            Ok(r) => r,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
    let res_id = res.last_insert_rowid() as i32;
    Ok(res_id)
}

pub async fn create_new_frame(video_view_id: i32, frame: &ViewCreateDescFrame, img_data: Vec::<u8>, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "INSERT INTO video_view_frame(video_view_id, \
        frame, x, y, width, height, img) VALUES \
        ($1, $2, $3, $4, $5, $6, $7)"
    )
        .bind(video_view_id)
        .bind(frame.frame_id)
        .bind(frame.x)
        .bind(frame.y)
        .bind(frame.width)
        .bind(frame.height)
        .bind(img_data)
        .execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(DBError::DatabaseError(err.to_string()))
    }
}

pub async fn get_view_frame_with_video_id(id: i32, frame: i32, pool: &mut Pool<Sqlite>) -> Result<VideoViewFrame, DBError> {
    match sqlx::query_as::<_,VideoViewFrame>(
        "SELECT * FROM video_view_frame WHERE video_view_id=$1 AND frame=$2"
    )
    .bind(id)
    .bind(frame)
    .fetch_one(&*pool).await {
        Ok(vv) => Ok(vv),
        Err(err) => Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_image_view_image_data(guid: String, frame: i32, pool: &mut Pool<Sqlite>) -> Result<Vec::<u8>, DBError> {
    let vv = match get_view_from_guid(guid.clone(), &mut pool.clone()).await {
        Ok(vv) => vv,
        Err(err) => return Err(DBError::DatabaseError(err.to_string()))
    };
    
    let res = match get_view_frame_with_video_id(vv.id, frame, &mut pool.clone()).await {
        Ok(vf) => vf,
        Err(err) => return Err(DBError::DatabaseError(err.to_string()))
    };
    Ok(res.img)
}

pub async fn delete_view_with_guid(guid: String, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    let vv = match get_view_from_guid(guid.clone(), &mut pool.clone()).await {
        Ok(vv) => vv,
        Err(err) => return Err(DBError::DatabaseError(err.to_string()))
    };

    match sqlx::query(
        "DELETE FROM video_view_frame WHERE video_view_id=$1"
    )
    .bind(vv.id)
    .execute(&*pool).await {
        Ok(_) => {},
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }

    match sqlx::query(
        "DELETE FROM video_view WHERE id=$1"
    )
    .bind(vv.id)
    .execute(&*pool).await {
        Ok(_) => {},
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }

    Ok(())
}