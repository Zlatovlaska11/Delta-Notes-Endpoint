pub mod get_files {

    use std::fs::{self, FileType};

    use crate::filehalndler::get_courses::courses::get_course_filepath;
    use serde::{self, Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CoureFile {
        pub filename: String,
        pub filepath: String,
    }

    pub fn get_list(id: u8, filepath: Option<String>) -> Result<Vec<CoureFile>, std::io::Error> {
        let paths;

        if filepath.is_some() {
            paths = fs::read_dir(filepath.unwrap()).unwrap();
        } else {
            let dir = get_course_filepath(id)?;

            paths = fs::read_dir(dir.clone()).unwrap();
        }

        let mut vec: Vec<CoureFile> = Vec::new();

        for path in paths {
            if path.as_ref().unwrap().metadata().unwrap().is_dir() {
                let pth = path.as_ref().unwrap().path().display().to_string();

                for x in get_list(id, Some(pth)).unwrap() {
                    vec.push(x)
                }
            }
            if path
                .as_ref()
                .unwrap()
                .path()
                .display()
                .to_string()
                .contains("pptx")
                || path
                    .as_ref()
                    .unwrap()
                    .path()
                    .display()
                    .to_string()
                    .contains("ppt")
            {
                vec.push(CoureFile {
                    filename: path.as_ref().unwrap().file_name().into_string().unwrap(),
                    filepath: path
                        .expect("error with the json")
                        .path()
                        .display()
                        .to_string(),
                });
            }
        }

        Ok(vec)
    }
}
