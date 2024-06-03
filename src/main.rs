pub mod auth;
pub mod database;
pub mod enpoint;
pub mod filehalndler;


fn main() {
    filehalndler::handler::course_list(0);
}
