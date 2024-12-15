use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::animation_queries;
use crate::image::gif;

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
        // .and(warp::filters::query::raw()
        //      .or(warp::any().map(|| String::default()))
        //      .unify()
        // )
        .and(db_conn.clone())
        .and_then(get_animationgif_render_impl);

    new_animation_page
        .or(get_animation_list)
        .or(get_animation)
        .or(load_animation_page)
        .or(animation_page)
        .or(get_gif_render)
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
