use warp::Filter;
use std::fs;
use serde_json::json;
use handlebars::Handlebars;
use warp::{filters::BoxedFilter, Reply};

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
    .allow_any_origin().allow_methods(&[warp::http::Method::GET, warp::http::Method::POST]);

    let heartbeat = warp::path!("heartbeat")
        .map(|| format!("ok"))
        .with(cors);
    
    // GET /home - get the main front page
    let home = warp::any().map(|| {
        let body: String = fs::read_to_string("templates/index.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    heartbeat 
        .or(home)
        .boxed()
}

