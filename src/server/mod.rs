use std::net::SocketAddr;
use warp::Filter;
use sqlx_sqlite::SqlitePool;

mod get_routes;
mod post_routes;

pub async fn start(addr: impl Into<SocketAddr>, db_conn: &mut SqlitePool) {
    let get_routes = get_routes::make_routes(&mut db_conn.clone());
    let post_routes = post_routes::make_routes(&mut db_conn.clone());

    let routes = post_routes.or(get_routes);


    warp::serve(routes)
        .run(addr)
        .await;
}