// Import the necessary modules from the actix_cors and actix_web crates
use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
// Import the Deserialize trait from the serde crate
use serde::Deserialize;

use sqlx::PgPool;

use dotenv::dotenv;
use std::env;

// mod psql;

// Define a struct that represents the data you're expecting to receive in the POST request
// The Deserialize trait allows this struct to be constructed from a JSON string
#[derive(Deserialize)]
pub struct FormData {
    data: String,
}

// Define an asynchronous function that takes a Json<FormData> as a parameter
// The Json extractor automatically loads the JSON body of the request into the FormData struct
// The function prints the received data and returns a response indicating that the data was received
async fn index(form: web::Json<FormData>) -> impl Responder {
    println!("Received: {}", form.data);
    HttpResponse::Ok().body("Data received")
}

// pub async fn run() -> std::io::Result<()>{
//     println!("Starting We-Movie back-end");
    
//     // Connect to database
//     psql::connect().await;

//     // Create a new HTTP server
//     HttpServer::new(|| {
//         // Create a new CORS middleware
//         // The permissive method allows any origin, any headers, and any methods
//         let cors = Cors::permissive();

//         // Create a new Actix web application
//         // The application is wrapped with the CORS middleware
//         // The route "/create-user" is configured to handle POST requests with the index function
//         App::new()
//             .wrap(cors)
//             .route("/create-user", web::post().to(index))
//     })
//     // Bind the server to the address "127.0.0.1:8080"
//     .bind("127.0.0.1:8080")?
//     // Run the server
//     .run()
//     // Await the completion of the server
//     .await;

//     return Ok(())

// }

#[get("/users")]
async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    println!("Getting Users");
    let result = sqlx::query!("SELECT username FROM users")
        .fetch_all(db_pool.get_ref())
        .await;
    match result {
        Ok(users) => {
            let usernames: Vec<String> = users.iter().map(|user| user.username.to_string()).collect();
            HttpResponse::Ok().json(usernames)
        },
        _ => HttpResponse::InternalServerError().body("Error"),
    }
}

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.expect("Failed to create pool");

    HttpServer::new(move || {
            // Create a new CORS middleware
            // The permissive method allows any origin, any headers, and any methods
            let cors = Cors::permissive();

            App::new().wrap(cors).data(db_pool.clone()).service(get_users)}
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}