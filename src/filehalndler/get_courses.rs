pub mod courses {
    use std::{
        collections::HashMap,
        default,
        fs::{self, read_dir, FileType},
    };

    use std::error::Error;

    #[derive(Debug, Clone)]
    struct Course {
        sections: HashMap<String, Vec<String>>,
        files_count: u32,
    }

    #[derive(Debug, Clone)]
    enum Courses {
        Czech(Course),
        Chemistry(Course),
        Tech(Course),
        Networking(Course),
        Physics(Course),
    }

    trait FileWork {
        fn new(&self, id: u32) -> Result<Courses, ()>;

        fn course_files(&self, file_path: String) -> Vec<String>;

        fn get_full_course(self, subject_filepath: String) -> HashMap<String, Vec<String>>;
    }

    impl FileWork for Courses {
        fn new(&self, id: u32) -> Result<Courses, ()> {
            let dirs: [String; 5] = [
                "Český jazyk a literatura 1.A (2023⧸24)".to_string(),
                "Přírodní vědy 1.A, 1.B (Eko, Bi, Ch) (2023⧸24)".to_string(),
                "Výpočetní technika 1".to_string(),
                "Počítačové systémy a sítě 1.A (Drvota, 2023⧸24)".to_string(),
                "Fyzika 1.ročník (2023⧸24)".to_string(),
            ];

            let files = self.clone().get_full_course(dirs[0].to_string());

            let course: Course = Course {
                sections: files.clone(),
                files_count: files.keys().count() as u32,
            };

            match id {
                0 => Ok(Courses::Czech(course)),
                1 => Ok(Courses::Chemistry(course)),
                2 => Ok(Courses::Tech(course)),
                3 => Ok(Courses::Networking(course)),
                4 => Ok(Courses::Physics(course)),
                default => Err(()),
            }
        }

        fn course_files(&self, file_path: String) -> Vec<String> {
            let mut files: Vec<String> = Vec::new();
            let paths = fs::read_dir(file_path).unwrap();

            // add here the recursive file explore because of some subjects have more layers of
            // files

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

        fn get_full_course(self, subject_filepath: String) -> HashMap<String, Vec<String>> {
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
