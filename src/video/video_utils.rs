use std::path::Path;
use std::fs;

pub fn get_image_count_for_video(guid: String) -> Result<usize, String> {
    let folder = format!("./files/videos/processed/{}", guid);
    if !Path::new(folder.clone().as_str()).exists() {
        return Err("Folder does not exist".to_string());
    }
    
    let mut ret_count = 0;
    match fs::read_dir(&folder) {
        Ok(res) => {
            res.for_each(|dir_entry| {
                let file_n = dir_entry.unwrap().file_name().into_string().unwrap();
                match Path::new(&file_n.clone()).extension() {
                    Some(f) => {
                        let ext = f.to_str().unwrap().to_string();
                        if ext == "png".to_string() {
                            ret_count += 1;
                        }
                    },
                    None => {}
                }
            });
        },
        Err(err) => {
            println!("Error walking dir for image count: {}", err.to_string());
            return Err("Error getting file count".to_string());
        }
    }
    Ok(ret_count)
}

        