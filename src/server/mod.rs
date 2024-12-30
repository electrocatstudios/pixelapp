use std::net::SocketAddr;
use warp::Filter;
use warp::filters::BoxedFilter;
use sqlx_sqlite::SqlitePool;

mod get_routes;
mod post_routes;
mod delete_routes;
pub mod query_params;

pub async fn start(addr: impl Into<SocketAddr>, db_conn: &mut BoxedFilter<(SqlitePool,)>) {
    let get_routes = get_routes::make_routes(&mut db_conn.clone()).await;
    let post_routes = post_routes::make_routes(&mut db_conn.clone()).await;
    let delete_routes = delete_routes::make_routes(&mut db_conn.clone()).await;

    let routes = delete_routes.or(post_routes).or(get_routes);

    warp::serve(routes)
        .run(addr)
        .await;
}