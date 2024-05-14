pub mod courses {
    use std::{
        collections::HashMap,
        fs::{self, read_dir, FileType},
    };

    struct course {
        sections: HashMap<u32, Vec<String>>,
        files_count: u32,
    }

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

        fn count_files(file_path: String) -> Vec<String> {
            let mut files: Vec<String> = Vec::new();
            let paths = fs::read_dir("./").unwrap();

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
    }

    fn get_list(course_id: u32) {
        let mut topics: Vec<String> = Vec::new();
    }
}
