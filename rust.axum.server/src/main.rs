#![allow(non_snake_case)]

use std::net::TcpListener;
use rustserver::services::postgres::PostgresService;

// #[tokio::main]
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16)
        .thread_stack_size(3 * 1024 * 1024)
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    runtime.block_on(async {
        // Prepare the variables that the run method needs.
        let serverPort = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8888".to_string());
        let databaseService = PostgresService::new(None).await;

        // truncate users
        sqlx::query("truncate table \"User\";")
            .execute(&databaseService.connection)
            .await
            .expect("Failed to truncate users");

        // create users.
        for i in 0..100 {
            let id = uuid::Uuid::new_v4();
            let framework = "test framework".to_string();
            let first_name = format!("first_name_{}", i);
            let last_name = format!("last_name_{}", i);
            let email = format!("test_{}@test.com", i);
            sqlx::query(&format!(
                "
                    do
                    $do$
                    begin
                        insert into \"User\" (id, framework, first_name, last_name, email)
                        VALUES ('{id}', '{framework}', '{first_name}', '{last_name}', '{email}');
                    end
                    $do$
        "
            ))
                .execute(&databaseService.connection)
                .await
                .expect("Failed to insert User");
        }

        let address = format!("0.0.0.0:{}", serverPort);
        let listener = TcpListener::bind(&address).expect("TcpListener failed");
        println!("Listening at: {}", &address);
        rustserver::run(listener, databaseService)
            .expect("rustserver::run failed")
            .await
            .expect("axum::Server failed");
    });
}
