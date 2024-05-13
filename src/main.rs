use async_std::println;

pub mod auth;
pub mod database;
pub mod filebucket;
pub mod enpoint;

#[shuttle_runtime::main]
async fn tide(#[shuttle_shared_db::Postgres] conn_str: String) -> shuttle_tide::ShuttleTide<()> {
    //filebucket::filebucket::get_files().await.unwrap();
    enpoint::server::start_server(conn_str).await
}
