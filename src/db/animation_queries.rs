
use std::vec;
use sqlx::Sqlite;
use sqlx::pool::Pool;
use uuid::Uuid; 
use serde_json::json;
use std::fs;

use super::animation_models::{Animation, AnimationDesc, AnimationDetails, AnimationLimb, AnimationLimbDetails, AnimationLimbMove };
use super::DBError;


pub async fn get_animation_list(pool: &mut Pool<Sqlite>) -> Result<vec::Vec::<Animation>, DBError> {
    // Do the actual request to get the list
    match sqlx::query_as::<_,Animation>(
        "SELECT * FROM animation"
    ).fetch_all(&*pool).await {
        Ok(anims) => Ok(anims),
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn create_new_animation(data: AnimationDesc, pool: &mut Pool<Sqlite>) -> Result<String, DBError> {
    let guid: String = format!("{:?}", Uuid::new_v4());

    match sqlx::query(
        "INSERT INTO animation(name, description, \
        width, height, length, guid) VALUES \
        ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.width)
        .bind(&data.height)
        .bind(&data.length)
        .bind(&guid.clone())
        .execute(&*pool).await {
            Ok(_) => Ok(guid),
            Err(err) => Err(DBError::DatabaseError(err.to_string()))
        }
}

pub async fn get_animation_from_guid(guid: String, pool: &mut Pool<Sqlite>) -> Result<AnimationDetails, DBError> {
    let animation = match sqlx::query_as::<_,Animation>(
        "SELECT * FROM animation WHERE guid=$1"
        )
        .bind(guid)
        .fetch_one(&*pool).await {
            Ok(anim) => anim,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };

    let mut ret = AnimationDetails::from_animation_model(&animation);

    let animation_limbs = match sqlx::query_as::<_,AnimationLimb>(
        "SELECT * FROM animation_limb WHERE animation_id=$1"
        )
        .bind(animation.id)
        .fetch_all(&*pool).await {
            Ok(limbs) => limbs,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
    
    for al in animation_limbs.iter() {
        let animation_limb_moves = match sqlx::query_as::<_,AnimationLimbMove>(
            "SELECT * FROM animation_limb_move WHERE animation_limb_id=$1"
            )
            .bind(al.id)
            .fetch_all(&*pool).await {
                Ok(limbs) => limbs,
                Err(_err) => Vec:: new()
            };
        ret.animation_limbs.push(
            AnimationLimbDetails::from_animation_limb_model(&al, animation_limb_moves)
        );
    }

    Ok(ret)
}

pub async fn get_animation_details_as_json(guid: String, pool: &mut Pool<Sqlite>) -> Result<serde_json::Value, DBError> {
    
    let anim = match get_animation_from_guid(guid.clone(), pool).await {
        Ok(a) => a,
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    };

    let menubar: String = fs::read_to_string("templates/snippets/animation_menubar.html").unwrap().parse().unwrap();
    let ret = &json!({
        "name": anim.name.clone(),
        "width": anim.width,
        "height": anim.height,
        "length": anim.length,
        "guid": guid,
        "menubar": &menubar
    });

    Ok(ret.clone())
}
