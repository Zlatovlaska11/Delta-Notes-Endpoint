pub mod auth;
pub mod database;
pub mod enpoint;
pub mod filehalndler;

//#[shuttle_runtime::main]
//async fn tide(#[shuttle_shared_db::Postgres] conn_str: String) -> shuttle_tide::ShuttleTide<()> {
//       enpoint::server::start_server(conn_str).await
//}

use crate::filehalndler::file_list::get_files::get_list;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("test.json")
        .unwrap();

    for x in get_list(0, None).unwrap() {
        writeln!(file, "{}", x.to_string()).unwrap()
    }
}
