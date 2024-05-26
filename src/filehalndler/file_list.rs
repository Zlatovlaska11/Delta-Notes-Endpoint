pub mod get_files {

    use std::fs;

    use crate::filehalndler::get_courses::courses::get_course_filepath;
    use serde::{self, Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct CoureFile {
        filename: String,
        filepath: String,
    }

    pub fn get_list(id: u8) -> Result<Vec<serde_json::Value>, std::io::Error> {
        let dir = get_course_filepath(id)?;

        let paths = fs::read_dir(dir.clone()).unwrap();

        let mut vec: Vec<serde_json::Value> = Vec::new();

        for path in paths {
            vec.push(serde_json::json!(CoureFile {
                filename: path.as_ref().unwrap().file_name().into_string().unwrap(),
                filepath: path
                    .expect("error with the json")
                    .path()
                    .display()
                    .to_string(),
            }));
        }

        Ok(vec)
    }
}
