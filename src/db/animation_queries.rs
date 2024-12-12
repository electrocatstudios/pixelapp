
use std::vec;
use sqlx::Sqlite;
use sqlx::pool::Pool;
use uuid::Uuid; 

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
        width, height, guid) VALUES \
        ($1, $2, $3, $4, $5)"
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.width)
        .bind(&data.height)
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
        "SELECT * FROM animation_limb WHERE animat_id=$1"
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