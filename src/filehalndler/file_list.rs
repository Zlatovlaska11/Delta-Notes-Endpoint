pub mod get_files {

    use std::{fs, path::Path};

    use crate::filehalndler::get_courses::courses::get_course_filepath;
    use serde::{self, Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CoureFile {
        pub description: String,
        pub title: String,
        pub url: String,
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
                let url = path
                    .as_ref()
                    .expect("json err")
                    .path()
                    .display()
                    .to_string();

                let desc = extract_last_two_segments(&url).unwrap();
                let url = format!(
                    "{}/{}/{}",
                    "course",
                    id,
                    path.as_ref().unwrap().file_name().into_string().unwrap()
                );

                vec.push(CoureFile {
                    description: desc.clone(),
                    title: path.as_ref().unwrap().file_name().into_string().unwrap(),
                    url,
                });
            }
        }

        Ok(vec)
    }

    fn extract_last_two_segments(path: &str) -> Option<String> {
        // Create a Path object from the input string
        let path = Path::new(path);

        // Collect the components of the path into a vector
        let components: Vec<&str> = path.iter().filter_map(|os_str| os_str.to_str()).collect();

        // Ensure there are enough components to extract the desired segment
        if components.len() < 2 {
            return None;
        }

        // Get the second-to-last and last components
        let second_to_last = components[components.len() - 2];
        let last = components[components.len() - 1];

        // Join the components into the desired path segment
        Some(format!("{}/{}", second_to_last, last))
    }
}
