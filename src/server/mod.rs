use std::net::SocketAddr;
use warp::Filter;

mod get_routes;
mod post_routes;

pub async fn start(addr: impl Into<SocketAddr>) {
    let get_routes = get_routes::make_routes();
    let post_routes = post_routes::make_routes();

    let routes = post_routes.or(get_routes);


    warp::serve(routes)
        .run(addr)
        .await;
}