use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use serde::{Deserialize, Serialize};

use crate::video::video_utils::get_image_count_for_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoModel {
    pub guid: String,
    pub name: String,
    pub description: String,
    pub frames: usize
}

impl VideoModel {
    pub fn new(guid: String) -> Self {
        VideoModel {
            guid: guid,
            name: "".to_string(),
            description: "".to_string(),
            frames: 0
        }
    }

    pub fn from_guid(guid: String) -> Result<VideoModel, String> {
        let folder = format!("./files/videos/processed/{}", guid);
        if !Path::new(folder.clone().as_str()).exists() {
            return Err("Folder does not exist".to_string());
        }

        let image_count: usize = match get_image_count_for_video(guid.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err)
        };

        let details = format!("{}/details.txt", folder);
        if !Path::new(folder.clone().as_str()).exists() {
            return Err("Details file does not exist".to_string());
        }

        let mut ret = VideoModel::new(guid);
        ret.frames = image_count;

        let reader = BufReader::new(File::open(details.as_str()).expect("Cannot open file.txt"));
        for (idx,line) in reader.lines().enumerate() {
            if idx == 0 {
                ret.name = line.unwrap();
            } else if idx == 1 {
                ret.description = line.unwrap()
            }
        }
        Ok(ret)
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct VideoView {
    pub id: i32,
    pub guid: String,
    pub video_guid: String,
    pub name: String
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct VideoViewFrame {
    pub id: i32,
    pub video_view_id: i32,
    pub frame: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub img: Vec::<u8>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoViewExt {
    pub guid: String,
    pub name: String,
    pub frames: Vec::<VideoViewExtFrame>
}

impl VideoViewExt {
    pub fn default() -> Self {
        VideoViewExt {
            guid: "".to_string(),
            name: "".to_string(),
            frames: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoViewExtFrame{
    pub frame_id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub img: Vec::<u8>
}

impl VideoViewExtFrame {
    pub fn new_from_video_view_frame(vvf: &VideoViewFrame) -> Self {
        let mut out_vec = Vec::<u8>::new();
        for val in &vvf.img {
            out_vec.push(*val);
        }
        VideoViewExtFrame {
            frame_id: vvf.frame,
            x: vvf.x,
            y: vvf.y,
            width: vvf.width,
            height: vvf.height,
            img: out_vec
        }
    }
}

// Incoming request to create a view from a set of frames
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewCreateDesc{
    pub guid: String,
    pub name: String,
    pub frames: Vec::<ViewCreateDescFrame>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewCreateDescFrame{
    pub frame_id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewList {
    pub name: String,
    pub guid: String
}