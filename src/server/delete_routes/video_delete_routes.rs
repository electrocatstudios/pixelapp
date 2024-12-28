use warp::{Filter, filters::BoxedFilter, Reply, Rejection};
use sqlx::{Sqlite,SqlitePool,Pool};
use serde_json::json;

use crate::db::video_queries;

pub(super) async fn make_routes(db_conn: &mut BoxedFilter<(SqlitePool,)>) -> BoxedFilter<(impl Reply,)> {
    // let cors = warp::cors()
    //     .allow_any_origin().allow_methods(&[warp::http::Method::DELETE]);

    let view_delete = warp::path!("api" / "view" / String)
        .and(warp::delete())
        .and(db_conn.clone())
        .and_then(view_delete_impl);

    view_delete
        .boxed()
}

async fn view_delete_impl(guid: String, db_pool: Pool<Sqlite>) -> Result<Box<dyn Reply>, Rejection> {
    match video_queries::delete_view_with_guid(guid, &mut db_pool.clone()).await {
        Ok(_) => {},
        Err(err) => {
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": err.to_string()}))
                )
            )
        }
    }
    Ok(
        Box::new(
            warp::reply::json(&json!({"status": "Ok", "message": "".to_string()}))
        )
    )
}  
