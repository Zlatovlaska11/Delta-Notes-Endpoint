pub mod file_list;
pub mod file_serving;
pub mod get_courses;
mod test;

pub mod handler {
    use std::u8;

    use serde_json::json;
    use serde_json::Value;

    use super::file_list::get_files::get_list;

    pub fn course_list(id: u8) -> Value {
        if id == u8::MAX {
            todo!("return err");
        }

        let json_list = json!(get_list(id, None).unwrap());

        json_list
    }
}
