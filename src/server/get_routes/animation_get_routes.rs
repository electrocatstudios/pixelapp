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
    // Get /animation_new - get the page for creating a new pixel image
    let new_animation_page = warp::path("new").map(|| {
        let body: String = fs::read_to_string("templates/setuppixel.html").unwrap().parse().unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            handlebars.render("tpl_1", &json!({})).unwrap()
        )
    });

    new_animation_page.boxed()
}