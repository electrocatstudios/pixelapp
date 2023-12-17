use std::fs;
use serde_json::json;
use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Sqlite,SqlitePool,Pool};

use crate::db::queries;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
    .allow_any_origin().allow_methods(&[warp::http::Method::GET, warp::http::Method::POST]);

    let heartbeat = warp::path!("heartbeat")
        .map(|| format!("ok"))
        .with(cors);
    
    // GET 404 - catch all
    let default = warp::any().map(|| {
        let body: String = fs::read_to_string("templates/404.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // GET /home - get the main front page
    let home = warp::path::end().map(|| {
        let body: String = fs::read_to_string("templates/index.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // Get /new - get the page for creating a new pixel image
    let new_image_page = warp::path("new").map(|| {
        let body: String = fs::read_to_string("templates/setuppixel.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // // GET /api/pixel - get list of pixels
    let get_pixel_list_route = warp::get()
        .and(warp::path!("api" / "pixel"))
        .and(db_conn.clone())
        .and_then(get_pixel_list);

    // GET /js/<file> - get named js file
    let get_js = warp::path("js").and(warp::fs::dir("./assets/js/"));
    // GET /css/<file> - get named css file
    let get_css = warp::path("css").and(warp::fs::dir("./assets/css/"));
    // GET /font/<file> - get named font file
    let get_font = warp::path("font").and(warp::fs::dir("./assets/fonts/"));
    // GET /img/<file> - get named img file
    let get_img = warp::path("img").and(warp::fs::dir("./assets/fonts/"));
    

    heartbeat 
        .or(get_img)
        .or(get_js)
        .or(get_css)
        .or(get_font)
        .or(new_image_page)
        .or(get_pixel_list_route)
        .or(home)
        .or(default)
        .boxed()
}

async fn get_pixel_list(db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    let rows = match queries::get_pixel_list(&mut db_pool.clone()).await {
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
            warp::reply::json(&json!({"status": "ok", "message": "", "pixelimages": rows}))
        )
    )
} 