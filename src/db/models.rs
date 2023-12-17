use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct PixelImage {
    pub id: i32, // By default, using barrel's types::primary() results in i32
    pub name: String,
    pub description: String,
    pub owner_id: i32,
    pub width: i32,
    pub height: i32,
    pub pixelwidth: i32,
    pub guid: String,
}
