use warp::Filter;
use std::fs;
use serde_json::json;
use handlebars::Handlebars;
use warp::{filters::BoxedFilter, Reply};
use sqlx::SqlitePool;

pub(super) fn make_routes(db_conn: &mut SqlitePool) -> BoxedFilter<(impl Reply,)> {
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

    let new_image_page = warp::path("new").map(|| {
        let body: String = fs::read_to_string("templates/setuppixel.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    // GET /api/pixel - get list of pixels
    let get_pixel_list = get_pixel_list(db_conn);

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
        .or(get_pixel_list)
        .or(home)
        .or(default)
        .boxed()
}

fn get_pixel_list(db_conn: &mut SqlitePool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_in = db_conn.clone();
    warp::path!("api" / "pixel")
            .and(warp::any())
            .map( move || {
                let _db = match db_in.try_acquire(){
                    Some(db) => db,
                    None => {
                        return warp::reply::json(&json!({"status": "fail", "message": "database connection failure"}))
                    }
                };
                warp::reply::json(&json!({"status": "ok", "message": ""}))
            })
} 