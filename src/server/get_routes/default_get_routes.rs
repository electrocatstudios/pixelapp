use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply};
use sqlx::SqlitePool;


pub(super) async fn make_routes(_db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
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
    
    heartbeat
        .or(default)
        .boxed()
}