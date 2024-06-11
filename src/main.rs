use auth::auth::get_token;
use rocket::{routes, serde::json::Json, Rocket};
use server::server_rocket;

pub mod auth;
pub mod enpoint;
pub mod filehalndler;
mod server;
mod tests;

#[rocket::main]
async fn main() {
    let builder = Rocket::build()
        .attach(server::CORS)
        .configure(rocket::Config::figment().merge(("port", 8080)))
        .mount(
            "/",
            routes![server_rocket::list, server_rocket::all_options, server_rocket::login_endpoint, server_rocket::reg_endpoint, server_rocket::get_pptx_link, server_rocket::validate_token],
        )
        .launch()
        .await
        .unwrap();

    builder.launch().await.unwrap();
}

