use actix_rt;

use we_movie_backend;

// Define the main function
// The actix_rt::main attribute turns the function into an asynchronous main function
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    we_movie_backend::run().await
}