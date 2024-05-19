#[cfg(test)]
pub mod test{
    use crate::filehalndler::get_courses::courses::{self, Courses};
    
    #[test]
    fn files_test() {
        let courses: Courses = Courses::new(1);

    }
}
