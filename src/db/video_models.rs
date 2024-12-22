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