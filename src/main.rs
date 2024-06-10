use auth::auth::get_token;
use dotenv::dotenv;

pub mod auth;
pub mod enpoint;
pub mod filehalndler;
mod tests;

#[shuttle_runtime::main]
async fn tide() -> shuttle_tide::ShuttleTide<()> {
    dotenv().ok();
    let conn_str = std::env::var("POSTGRES_URL").expect("no postgres url specified");
    enpoint::server::start_server(conn_str).await
}

