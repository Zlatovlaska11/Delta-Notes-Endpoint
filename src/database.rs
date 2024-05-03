pub mod db_work {

    use async_std::println;
    use tokio_postgres::{row, Client, Error, GenericClient, NoTls};

    pub async fn get_connection(conn_str: String) -> Client {
        // Create a connection string

        // Parse the connection string
        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();

        // Spawn a task to process the connection

        client
    }

    pub async fn insert_data(filename: String, notes: String, conn_str: String) {
        let client = get_connection(conn_str).await;

        let query = "
        INSERT INTO notes (filename, notes)
        VALUES ($1, $2)
        ";

        // Execute the query

        let rows_affected = client.execute(query, &[&filename, &notes]).await.unwrap();

        println!("{:?}", rows_affected);
    }

    pub async fn select(filename: String, conn_str: String) {
        let client = get_connection(conn_str).await;

        let rows = client
            .query("SELECT * FROM notes WHERE filename = $1;", &[&filename])
            .await
            .unwrap();

        for row in rows {
            // Access each column of the row by index
            let column1: i32 = row.get(0);
            let column2: String = row.get(1);
            let column3: String = row.get(2);
            // Add more columns as needed

            println!(
                "column1: {}, column2: {}, column3: {}",
                column1, column2, column3
            );
        }
    }

    pub async fn create_table(table_name: String, conn_str: String) {
        let client = get_connection(conn_str).await;

        let rows = client
            .query(
                "CREATE TABLE users ( username: text, password: text, mail: text) ",
                &[],
            )
            .await
            .unwrap();
    }
}
