use std::fs; 

pub mod proc;

pub struct VideoUploadDetails {
    pub guid: String,
    pub name: String,
    pub description: String
}

impl VideoUploadDetails {
    pub fn new(guid: String) -> Self {
        VideoUploadDetails{
            guid: guid,
            name: "".to_string(),
            description: "".to_string()
        }
    }

    pub fn save(self) {
        let filename = format!("./files/videos/ready/{}.txt", self.guid.clone());
        let file_content = format!("{}\n{}", self.name.clone(), self.description.clone());
        fs::write(filename, file_content).expect("Unable to write file");
    }
}