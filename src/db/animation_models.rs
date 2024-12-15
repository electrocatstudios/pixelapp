use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Animation {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub width: i32,
    pub height: i32,
    pub length: i32,
    pub guid: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct AnimationLimb {
    pub id: i32,
    pub animation_id: i32,
    pub name: String,
    pub color: String,
    pub parent: String
}


#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct AnimationLimbMove {
    pub id: i32,
    pub animation_limb_id: i32,
    pub x: f64,
    pub y: f64,
    pub rot: f64,
    pub length: f64,
    pub perc: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationDesc {
    pub name: String,
    pub description: String,
    pub width: i32,
    pub height: i32,
    pub length: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationSaveDesc {
    pub guid: String,
    pub limbs: Vec::<AnimationSaveLimbDesc>
}

// Incoming JSON messages for saving
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationSaveLimbDesc {
    pub name: String,
    pub color: String,
    pub parent: String,
    pub limb_moves: Vec::<AnimationSaveLimbMovesDesc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationSaveLimbMovesDesc {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
    pub length: f64,
    pub perc: f64
}
// End incoming JSON messages 

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationDetails {
    pub name: String,
    pub description: String,
    pub width: i32,
    pub height: i32,
    pub guid: String,
    pub length: i32,
    pub animation_limbs: Vec::<AnimationLimbDetails>
}

impl AnimationDetails {
    pub fn _default() -> Self {
        AnimationDetails {
            name: "".to_string(),
            description: "".to_string(),
            width: 0,
            height: 0,
            guid: "".to_string(),
            length: 2000,
            animation_limbs: Vec::new()
        }
    }

    pub fn from_animation_model(anim: &Animation) -> Self {
        AnimationDetails {
            name: anim.name.clone(),
            description: anim.description.clone(),
            width: anim.width,
            height: anim.height,
            guid: anim.guid.clone(),
            length: anim.length,
            animation_limbs: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationLimbDetails {
    pub name: String,
    pub color: String,
    pub parent: String,
    pub animation_limb_moves: Vec::<AnimationLimbMoveDetails>
}

impl AnimationLimbDetails {
    pub fn from_animation_limb_model(al: &AnimationLimb, alms: Vec::<AnimationLimbMove>) -> Self {
        let mut almds = Vec::new();
        for alm in alms.iter() {
            almds.push(
                AnimationLimbMoveDetails {
                    x: alm.x,
                    y: alm.y,
                    rot: alm.rot,
                    length: alm.length,
                    perc: alm.perc
                }
            )
        }

        AnimationLimbDetails {
            name: al.name.clone(),
            color: al.color.clone(),
            parent: al.parent.clone(),
            animation_limb_moves: almds
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct AnimationLimbMoveDetails {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
    pub length: f64,
    pub perc: f64
}