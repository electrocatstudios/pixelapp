use warp::{Filter, filters::BoxedFilter, Reply};
use sqlx::SqlitePool;

mod animation_post_routes;
mod default_post_routes;
mod pixel_post_routes;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    let animation_routes = animation_post_routes::make_routes(&mut db_conn.clone()).await;
    let default_routes = default_post_routes::make_routes(&mut db_conn.clone()).await;
    let pixel_routes = pixel_post_routes::make_routes(&mut db_conn.clone()).await;

    animation_routes
        .or(pixel_routes)
        .or(default_routes)
        .boxed()
}