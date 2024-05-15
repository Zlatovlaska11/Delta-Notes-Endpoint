pub mod courses {
    use std::{
        collections::HashMap,
        fs::{self, read_dir, FileType},
    };
    #[derive(Debug, Clone)]
    struct course {
        sections: HashMap<String, Vec<String>>,
        files_count: u32,
    }

    #[derive(Debug, Clone)]
    enum Courses {
        Czech(Vec<course>),
        Chemistry(Vec<course>),
        English(Vec<course>),
        Tech(Vec<course>),
        Networking(Vec<course>),
        Physics(Vec<course>),
    }

    impl Courses {
        fn new(id: u32) -> Courses {
            match id {
                0 => todo!(),
                _ => todo!(),
            }
        }

        pub fn course_files(&self, file_path: String) -> Vec<String> {
            let mut files: Vec<String> = Vec::new();
            let paths = fs::read_dir(file_path).unwrap();

            for path in paths {
                if path
                    .as_ref()
                    .unwrap()
                    .path()
                    .display()
                    .to_string()
                    .contains("pptx")
                    == true
                {
                    files.push(path.unwrap().path().display().to_string())
                }
            }

            files
        }

        pub fn get_full_course(self, subject_filepath: String) -> HashMap<String, Vec<String>> {
            let mut map: HashMap<String, Vec<String>> = HashMap::new();

            for file in read_dir(subject_filepath.clone()).expect("something wrong") {
                if file.as_ref().is_ok_and(|f| f.metadata().unwrap().is_dir()) {
                    map.insert(
                        subject_filepath.clone(),
                        self.clone().course_files(
                            file.unwrap()
                                .path()
                                .to_path_buf()
                                .to_string_lossy()
                                    .to_string(),
                        ),
                    );
                }
            }
            map
        }
    }

    fn get_list(course_id: u32) {
        let mut topics: Vec<String> = Vec::new();
    }
}
