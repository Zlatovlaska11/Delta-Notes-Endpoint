pub mod tests {
    use crate::auth::auth::{get_token, TokenClaims};
    use dotenv::dotenv;
    use tide::http::auth;

    #[tokio::test]
    async fn token_data() {
        dotenv().ok();
        let conn_str = std::env::var("POSTGRES_URL").expect("no postgres url specified");
        let token = get_token("zlatovlas".to_string(), "Kaktus".to_string(), conn_str).await;

        let token_data = jsonwebtoken::decode::<TokenClaims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret("test".as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .unwrap();

        assert_eq!(token_data.claims.username, "zlatovlas".to_string())
    }
}
