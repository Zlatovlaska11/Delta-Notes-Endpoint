pub mod auth;
pub mod database;
pub mod enpoint;
pub mod filehalndler;

#[shuttle_runtime::main]
async fn tide(#[shuttle_shared_db::Postgres] conn_str: String) -> shuttle_tide::ShuttleTide<()>
{
    enpoint::server::start_server(conn_str).await
}
