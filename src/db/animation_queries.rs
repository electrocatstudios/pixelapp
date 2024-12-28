
use std::vec;
use sqlx::Sqlite;
use sqlx::pool::Pool;
use uuid::Uuid; 
use serde_json::json;
use std::fs;

use super::animation_models::{Animation, AnimationDesc, AnimationDetails, AnimationLimb, AnimationLimbDetails, AnimationLimbMove, AnimationSaveLimbDesc, AnimationUpdateDesc };
use super::video_queries::{self, get_view_from_guid};
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
    let view_id: Option<i32> = match data.view_guid {
        Some(view_guid) => {
            match get_view_from_guid(view_guid, &mut pool.clone()).await {
                Ok(view) => Some(view.id),
                Err(err) => return Err(DBError::DatabaseError(err.to_string()))
            }
        },
        None => None
    };

    match sqlx::query(
        "INSERT INTO animation(name, description, \
        width, height, length, guid, view_id) VALUES \
        ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.width)
        .bind(&data.height)
        .bind(&data.length)
        .bind(&guid.clone())
        .bind(view_id)
        .execute(&*pool).await {
            Ok(_) => Ok(guid),
            Err(err) => Err(DBError::DatabaseError(err.to_string()))
        }
}

pub async fn get_animation_from_guid(guid: String, pool: &mut Pool<Sqlite>) -> Result<Animation, DBError> {
    // Return the animation - not for serializing to JSON (contains id)
    match sqlx::query_as::<_,Animation>(
        "SELECT * FROM animation WHERE guid=$1"
    )
    .bind(guid)
    .fetch_one(&*pool).await {
        Ok(anim) => Ok(anim),
        Err(err) => Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_animation_from_id(id: i32, pool: &mut Pool<Sqlite>) -> Result<Animation, DBError> {
    // Return the animation - not for serializing to JSON (contains id)
    match sqlx::query_as::<_,Animation>(
        "SELECT * FROM animation WHERE id=$1"
    )
    .bind(id)
    .fetch_one(&*pool).await {
        Ok(anim) => Ok(anim),
        Err(err) => Err(DBError::UnknownError(err.to_string()))
    }
}

pub async fn get_animation_details_from_guid(guid: String, pool: &mut Pool<Sqlite>) -> Result<AnimationDetails, DBError> {
    // Return the AnimationDetails - which you can serialize, doesn't contain ID
    let animation = match get_animation_from_guid(guid.clone(), &mut pool.clone()).await {
        Ok(animation) => animation,
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    };

    let mut ret = AnimationDetails::from_animation_model(&animation);

    let animation_limbs = match sqlx::query_as::<_,AnimationLimb>(
        "SELECT * FROM animation_limb WHERE animation_id=$1 ORDER BY id ASC"
        )
        .bind(animation.id)
        .fetch_all(&*pool).await {
            Ok(limbs) => limbs,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
    
    for al in animation_limbs.iter() {
        let animation_limb_moves = match sqlx::query_as::<_,AnimationLimbMove>(
            "SELECT * FROM animation_limb_move WHERE animation_limb_id=$1 ORDER BY perc ASC
            "
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
    let view_guid = match anim.view_id {
        Some(id) => {
            match video_queries::get_view_from_id(id, &mut pool.clone()).await {
                Ok(v) => v.guid,
                Err(err) => return Err(DBError::UnknownError(err.to_string()))
            }
        },
        None => "".to_string()
    };

    let ret = &json!({
        "name": anim.name.clone(),
        "width": anim.width,
        "height": anim.height,
        "length": anim.length,
        "guid": guid,
        "menubar": &menubar,
        "view_guid": view_guid.clone()
    });

    Ok(ret.clone())
}

pub async fn get_limb_list_for_animation_id(id: i32, pool: &mut Pool<Sqlite> ) -> Result<Vec::<AnimationLimb>, DBError> {
    let animation_limbs = match sqlx::query_as::<_,AnimationLimb>(
        "SELECT * FROM animation_limb WHERE animation_id=$1 ORDER BY id ASC"
        )
        .bind(id)
        .fetch_all(&*pool).await {
            Ok(limbs) => limbs,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
    Ok(animation_limbs)
}

pub async fn delete_animation_limb_moves_from_animation_limb_id(id: i32, pool: &mut Pool<Sqlite>) -> Result<(),DBError> {
    match sqlx::query(
        "DELETE FROM animation_limb_move WHERE animation_limb_id=$1"
        )
        .bind(id)
        .fetch_all(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(DBError::UnknownError(err.to_string()))
        }
}

pub async fn update_limbs_for_animation(id: i32, limbs: Vec::<AnimationSaveLimbDesc>, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    // Remove all previous limbs
    let current_limbs = match get_limb_list_for_animation_id(id, pool).await {
        Ok(limbs) => limbs,
        Err(err) => return Err(DBError::UnknownError(err.to_string()))
    };

    for cl in current_limbs.iter() {
        match delete_animation_limb_moves_from_animation_limb_id(cl.id, pool).await {
            Ok(_) => {},
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        }
    }
    // Done cleaning out previous limbs

    // For each limb insert the new limb - and get the id - then push the limb_moves
    for limb in limbs.iter() {
        let res = match sqlx::query(
            "INSERT INTO animation_limb(animation_id, name, color, parent) \
                VALUES ($1, $2, $3, $4)"
            )
            .bind(id)
            .bind(limb.name.to_string())
            .bind(limb.color.to_string())
            .bind(limb.parent.to_string())
            .execute(&*pool).await {
                Ok(res) => res,
                Err(err) => return Err(DBError::UnknownError(err.to_string()))
            };
        let res_id = res.last_insert_rowid() as i32;

        // Insert each limb move for the new limb
        for limb_move in limb.limb_moves.iter() {
            match sqlx::query(
                "INSERT INTO animation_limb_move(animation_limb_id, x, y, rot, length, perc) \
                    VALUES ($1, $2, $3, $4, $5, $6)"
                )
                .bind(res_id)
                .bind(limb_move.x)
                .bind(limb_move.y)
                .bind(limb_move.rot)
                .bind(limb_move.length)
                .bind(limb_move.perc)
                .execute(&*pool).await {
                    Ok(res) => res,
                    Err(err) => return Err(DBError::UnknownError(err.to_string()))
                };
        }
    }
    Ok(())
}

pub async fn update_animation_details(id: i32, aus: AnimationUpdateDesc, pool: &mut Pool<Sqlite>) -> Result<(), DBError> {
    match sqlx::query(
        "UPDATE animation SET width=$1, height=$2, length=$3 WHERE id=$4"
        )
        .bind(aus.width)
        .bind(aus.height)
        .bind(aus.time_length)
        .bind(id)
        .execute(&*pool).await {
            Ok(res) => res,
            Err(err) => return Err(DBError::UnknownError(err.to_string()))
        };
    Ok(())
}
