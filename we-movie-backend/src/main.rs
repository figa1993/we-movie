// Import the necessary modules from the actix_cors and actix_web crates
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
// Import the Deserialize trait from the serde crate
use serde::Deserialize;

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

// Define the main function
// The actix_rt::main attribute turns the function into an asynchronous main function
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Back End");
    // Create a new HTTP server
    HttpServer::new(|| {
        // Create a new CORS middleware
        // The permissive method allows any origin, any headers, and any methods
        let cors = Cors::permissive();

        // Create a new Actix web application
        // The application is wrapped with the CORS middleware
        // The route "/create-user" is configured to handle POST requests with the index function
        App::new()
            .wrap(cors)
            .route("/create-user", web::post().to(index))
    })
    // Bind the server to the address "127.0.0.1:8080"
    .bind("127.0.0.1:8080")?
    // Run the server
    .run()
    // Await the completion of the server
    .await
}