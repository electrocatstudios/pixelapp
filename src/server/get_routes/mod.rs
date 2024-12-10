use warp::{filters::BoxedFilter, Reply};
use sqlx::SqlitePool;

mod pixel_get_routes;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    // TODO: Get the animation_get_routes routes and combine them before returning
    pixel_get_routes::make_routes(&mut db_conn.clone()).await
}