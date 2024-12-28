use warp::{Filter, filters::BoxedFilter, Reply};
use sqlx::SqlitePool;

mod delete_routes;
mod video_delete_routes;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    let pixel_routes = delete_routes::make_routes(&mut db_conn.clone()).await;
    let video_routes = video_delete_routes::make_routes(&mut db_conn.clone()).await;

    pixel_routes
        .or(video_routes)
        .boxed()
}