pub mod auth;
pub mod database;

pub mod enpoint;

#[shuttle_runtime::main]
async fn tide(#[shuttle_shared_db::Postgres] conn_str: String) -> shuttle_tide::ShuttleTide<()> {
    database::db_work::create_table("users".to_string(), conn_str.clone()).await;
    enpoint::server::start_server(conn_str).await

}
