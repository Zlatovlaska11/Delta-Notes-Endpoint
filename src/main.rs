use tide::{Request, prelude};
use serde_json::Value as JsonValue;

#[tokio::main]
async fn main() {
    
    let mut app = tide::Server::new();


    let check_acces = |mut req: Request<()>| async move {
        let body: JsonValue = req.body_json().await.unwrap();

        let file_name = body["filename"];

    };

    app.at("/getNotes").post();
    app.listen("127.0.0.1:8080").await.unwrap();
}



