use warp::{Filter, filters::BoxedFilter, Reply};
use sqlx::SqlitePool;

mod animation_get_routes;
mod default_get_routes;
mod pixel_get_routes;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    let animation_routes = animation_get_routes::make_routes(&mut db_conn.clone()).await;
    let default_get_routes = default_get_routes::make_routes(&mut db_conn.clone()).await;
    let pixel_routes = pixel_get_routes::make_routes(&mut db_conn.clone()).await;

    animation_routes
        .or(pixel_routes)
        .or(default_get_routes)
        .boxed()
}