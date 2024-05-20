#[cfg(test)]
pub mod test{
    use crate::filehalndler::get_courses::{self, courses};
    
    #[test]
    fn files_test() {

        let coure = get_courses::courses::get_files("Středověká literatura.pptx".to_string(), 0);

        assert_eq!("public/files/Český jazyk a literatura 1.A (2023⧸24)/Evropská středověká literatura/Středověká literatura.pptx", coure.unwrap())
    }
}
