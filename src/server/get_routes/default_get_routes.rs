use std::fs;
use serde_json::json;

use handlebars::Handlebars;
use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Sqlite,SqlitePool,Pool};

use image::{Pixel, Rgba, RgbaImage};
use image::io::Reader as ImageReader;

use crate::db::models::{PixelShading, PixelSaveFile};
use crate::db::{queries, models::{PixelPixel,IncomingPixel,IncomingShader}};
use crate::utils::{color_to_hex_string,hex_string_to_color};
use crate::image::gif;

use crate::server::query_params::{RenderQuery, GifRenderQuery};

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
    
    heartbeat
        .or(default)
        .boxed()
}