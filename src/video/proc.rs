// extern crate ffmpeg_next as ffmpeg;

use std::{fs, path::{Path, PathBuf}};
use std::process::Command;

pub fn process_pending_videos_into_frames() {
    // Make sure folders exist
    match fs::create_dir_all("./files/videos/processing") {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error creating folder {}", err);
            return;
        }
    }
    match fs::create_dir_all("./files/videos/processed") {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error creating folder {}", err);
            return;
        }
    }
    // List files
    let path = PathBuf::from("./files/videos/ready");
    match fs::read_dir(&path) { 
        Ok(res) =>   {
            res.for_each(|dir_entry| {
                match dir_entry.as_ref().unwrap().file_type() {
                    Ok(ft) => {
                        if !ft.is_file() {
                            return;
                        }
                    },
                    Err(err) => {
                        eprintln!("Error getting file type {}", err);
                        return;
                    }
                }
                let mut file_n = dir_entry.unwrap().file_name().into_string().unwrap();
                match Path::new(&file_n.clone()).extension() {
                    Some(f) => {
                        let ext = f.to_str().unwrap().to_string();
                        if ext != "mp4".to_string() {
                            // println!("Found a file that was not mp4: {}", file_n.clone());
                            return;
                        }
                    },
                    None => println!("Got none from extension")
                }
                let _ = file_n.drain((file_n.len()-4)..file_n.len());
                process_file(file_n);
            });
        },
        Err(err) => {
            eprintln!("Error reading dir {}", err);
            return;
        }
    }
   
}

fn process_file(guid: String) {
        // Move file to in-proc
        let start = format!("./files/videos/ready/{}.mp4", guid.clone());
        let end = format!("./files/videos/processing/{}.mp4", guid.clone());
     
        match fs::rename(start, end) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Error moving file {}", err);
                return;
            }
        }

        // Make output dir
        match fs::create_dir_all(format!("./files/videos/processed/{}", guid.clone())) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Error creating folder for output {}", err);
                return;
            }
        }

        // Command - ffmpeg -i processing/<guid.mp4> processed/{guid}/img%04d.png
        let output = Command::new("ffmpeg")
                                .args([
                                    "-i",
                                    format!("./files/videos/processing/{}.mp4", guid.clone()).as_str(),
                                    format!("./files/videos/processed/{}/img%04d.png", guid.clone()).as_str()
                                ])
                                .output()
                                .expect("failed to execute process");
    
        if output.status.code().unwrap() != 0 {
            // TODO: Log this stuff
            println!("Status: {}", output.status);
            println!("stdout: {}", String::from_utf8(output.stdout).expect("stdout bytes not valid utf-8"));
            println!("stderr: {}", String::from_utf8(output.stderr).expect("stderr bytes not valid utf-8"));
            println!("failed - stopping the processing");
            return;
        }

        let start = format!("./files/videos/processing/{}.mp4", guid.clone());
        let end = format!("./files/videos/processed/{}/video.mp4", guid.clone());
        
        match fs::rename(start, end) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Error moving file {}", err);
                return;
            }
        }
        let file_path = format!("./files/videos/ready/{}.txt", guid.clone());
        if Path::new(file_path.as_str()).exists() {
            let end = format!("./files/videos/processed/{}/details.txt", guid.clone());
            match fs::rename(file_path, end) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Error moving details file {}", err);
                    return;
                }
            }
        }
}
