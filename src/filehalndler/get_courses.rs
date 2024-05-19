pub mod courses {
    use std::{
        collections::HashMap,
        fs::{self, read_dir}, ops::Deref,
    };

    #[derive(Debug, Clone)]
    pub struct Course {
        pub sections: HashMap<String, Vec<String>>,
        pub files_count: u32,
    }

    #[derive(Debug, Clone)]
    pub enum Courses {
        Czech { course: Course },
        Chemistry { course: Course },
        Tech { course: Course },
        Networking { course: Course },
        Physics { course: Course },
        Error,
    }

    trait FileGetting {
        
        fn get_files(&self, section: String, filename: String) -> String;
    }

    trait PreparingCourses {
        fn course_files(&self, file_path: String) -> Vec<String>;

        fn get_full_course(self, subject_filepath: String) -> HashMap<String, Vec<String>>;
    }

    impl PreparingCourses for Course {
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

    impl Courses {
        pub fn new(id: u32) -> Self {
            let crs: Courses;
            let course: Course = Course {
                files_count: u32::MAX,
                sections: HashMap::new(),
            };

            let dirs: [String; 5] = [
                "public/files/Český jazyk a literatura 1.A (2023⧸24)".to_string(),
                "public/files/Přírodní vědy 1.A, 1.B (Eko, Bi, Ch) (2023⧸24)".to_string(),
                "public/files/Výpočetní technika 1".to_string(),
                "public/files/Počítačové systémy a sítě 1.A (Drvota, 2023⧸24)".to_string(),
                "public/files/Fyzika 1.ročník (2023⧸24)".to_string(),
            ];
            let files = course
                .clone()
                .get_full_course(dirs[id as usize].to_string());

            let course: Course = Course {
                sections: files.clone(),
                files_count: files.keys().count() as u32,
            };
            match id {
                0 => crs = Courses::Czech { course },
                1 => crs = Courses::Chemistry { course },
                2 => crs = Courses::Tech { course },
                3 => crs = Courses::Networking { course },
                4 => crs = Courses::Physics { course },
                _default => return Courses::Error,
            }

            crs
        }
    }

    impl FileGetting for Courses {
        fn get_files(&self, section: String, filename: String) -> String {
        }
        // add code here
    }

}
