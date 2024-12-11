use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Pool, Sqlite, SqlitePool};


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


    new_animation_page
        .or(get_animation_list)
        .boxed()
}


async fn get_animation_list_impl(_db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    Ok(
        Box::new(
            warp::reply::json(&json!(
                {
                    "status": "ok"
                }
            ))
        )
    )
}