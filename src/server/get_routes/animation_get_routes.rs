use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::animation_queries;


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

   
    // GET /api/animation - get the list of animations
    let get_animation_list = warp::path!("api" / "animation")
        .and(db_conn.clone())
        .and_then(get_animation_list_impl);

    let get_animation = warp::path!("api" / "animation" / String)
        .and(db_conn.clone())
        .and_then(get_animation_impl);

    new_animation_page
        .or(get_animation_list)
        .or(get_animation)
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
    let animation = match animation_queries::get_animation_from_guid(guid, &mut db_pool.clone()).await {
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