pub mod courses {
    use std::fs;

    pub fn get_files(filename: String, coure_id: u8) -> Result<String, ()> {
        let dirs: [String; 5] = [
            "public/files/Český jazyk a literatura 1.A (2023⧸24)".to_string(),
            "public/files/Přírodní vědy 1.A, 1.B (Eko, Bi, Ch) (2023⧸24)".to_string(),
            "public/files/Výpočetní technika 1".to_string(),
            "public/files/Počítačové systémy a sítě 1.A (Drvota, 2023⧸24)".to_string(),
            "public/files/Fyzika 1.ročník (2023⧸24)".to_string(),
        ];

        let file = get_file(filename, dirs[coure_id as usize].clone());

        file
    }

    fn get_file(filename: String, folder: String) -> Result<String, ()> {
        let paths = fs::read_dir(folder.clone()).unwrap();

        for path in paths {
            if path.as_ref().unwrap().metadata().unwrap().is_dir() {
                let pth = path.as_ref().unwrap().path().display().to_string();

                let file = get_file(filename.clone(), pth);

                if file.is_ok() {
                    return file;
                }
            }
            if path
                .as_ref()
                .expect("some file err")
                .path()
                .display()
                .to_string()
                .contains(&filename)
            {
                return Ok(path.unwrap().path().display().to_string());
            }
        }

        Err(())
    }
}
