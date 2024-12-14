use serde_json::json;

use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{SqlitePool, Pool, Sqlite};

use crate::db::{animation_models::{AnimationDesc, AnimationSaveDesc}, animation_queries};

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

    
    create_new_animation
        .or(save_animation_limbs)
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
    // TODO: Implement the limb save - animation_queries::update_limbs_for_animation(anim.id)
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
