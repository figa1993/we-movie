// Import the necessary modules from the actix_cors and actix_web crates
use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, cookie::CookieBuilder, HttpRequest};

// Import actix_session which provides capability to use Redis for Session Management
use actix_session::{Session, SessionMiddleware, storage::RedisSessionStore};
use actix_web::cookie::Key;

// Import awc (actix web client) which provides capability to make HTTP client requests

use oauth2::reqwest::async_http_client;
use serde::Deserialize;

use sqlx::PgPool;

use oauth2::{TokenUrl, AuthUrl,
    basic::BasicClient,
    ClientId, ClientSecret,
    RedirectUrl,
    TokenResponse
};

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
// mod psql;

// Define a struct that represents the data you're expecting to receive in the POST request
// The Deserialize trait allows this struct to be constructed from a JSON string
#[derive(Deserialize)]
pub struct FormData {
    data: String,
}

// // Define an asynchronous function that takes a Json<FormData> as a parameter
// // The Json extractor automatically loads the JSON body of the request into the FormData struct
// // The function prints the received data and returns a response indicating that the data was received
// async fn index(form: web::Json<FormData>) -> impl Responder {
//     println!("Received: {}", form.data);
//     HttpResponse::Ok().body("Data received")
// }


// #[get("/users")]
// async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
//     println!("Getting Users");
//     let result = sqlx::query!("SELECT username FROM users")
//         .fetch_all(db_pool.get_ref())
//         .await;
//     match result {
//         Ok(users) => {
//             let usernames: Vec<String> = users.iter().map(|user| user.username.to_string()).collect();
//             HttpResponse::Ok().json(usernames)
//         },
//         _ => HttpResponse::InternalServerError().body("Error"),
//     }
// }

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String
}

fn build_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    // In prod, http://localhost:8000 will get replaced by whatever the production URL is
    let redirect_url = "http://localhost:8080/oauth_callback".to_string();
        
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

#[get("/oauth_callback")]
async fn ouath_callback(query: web::Query<HashMap<String, String>>, client: web::Data<BasicClient>, session: Session) -> impl Responder {
    println!("OAuth2 callback entered");
    let code = query.get("code").unwrap_or(&"".to_string()).clone();
    let state = query.get("state").unwrap_or(&"".to_string()).clone();

    // Exchange the code for an access token
    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await;

        match token_result {
            Ok(access_token_response) => {
                // @TODO: Use the Access Token to get an ID token with data that can be used as a key for finding user data
                // https://developers.google.com/identity/sign-in/web/backend-auth
                let tokeninfo_url = format!(
                    "https://www.googleapis.com/oauth2/v3/tokeninfo?access_token={}",
                    access_token_response.access_token().secret()
                );

                let http_client = awc::Client::default();

                let id_token_request = http_client.get(&tokeninfo_url).send().await;
                match id_token_request {
                    Ok(mut id_token_response) => {
                        let id_token_result = id_token_response.json::<String>().await;
                        match id_token_result {
                            Ok(id_token) => {

                                // ID Token validation is optional so we skip it.

                                let cookie = CookieBuilder::new("id_token", id_token.clone())
                                .http_only(true)
                                .finish();
                                session.insert("id_token",id_token);
                                HttpResponse::Found()
                                .cookie(cookie)
                                .header("Location", "http://localhost:3000/JWTCallback")
                                .finish()
                            }
                            Err (e) => {
                                HttpResponse::InternalServerError().body( format!("Malformed ID Token JSON payload received with error:{:?}",e))
                            }                        
                        }
                    }
                    Err(e) => {
                        HttpResponse::InternalServerError().body( format!("Retrieving ID Token failed with error:{:?}",e))
                    }
                }
            },
            Err(e) => HttpResponse::InternalServerError().body( format!("Exchanging code for access token failed with error:{:?}",e)),
        }
}

#[get("/UserHomePage")]
async fn protected(req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("id_token") {
        let id_token = cookie.value();
        // Optionally, validate the ID Token
        // let client_secret = env::var("OAUTH_CLIENT_SECRET").expect("OAUTH_CLIENT_SECRET not set");
        // let id_token_claims: Claims = decode(
        //     id_token,
        //     &DecodingKey::from_secret(client_secret.as_ref()),
        //     &Validation::new(Algorithm::RS256)
        // ).expect("Invalid ID token").claims;

        HttpResponse::Ok().body(format!("Your ID token is {}", id_token))
    } else {
        HttpResponse::Unauthorized().body("No token provided")
    }
}

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();

    println!("Initializing Database connection");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.expect("Failed to create pool");

    println!("Initializing Redis session store");
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379".to_string())
        .await
        .expect("Failed to create Redis session store");

    // Generate a secret key for session cookies
    let secret_key = Key::generate();

    // Create state
    let google_oauth_client_id = env::var("REACT_APP_GOOGLE_OAUTH_CLIENT_ID").expect("REACT_APP_GOOGLE_OAUTH_CLIENT_ID must be set");
    let google_oauth_client_secret = env::var("REACT_APP_GOOGLE_OAUTH_CLIENT_SECRET").expect("REACT_APP_GOOGLE_OAUTH_CLIENT_SECRET must be set");
    let oauth_client = build_oauth_client(google_oauth_client_id,google_oauth_client_secret);

    println!("Starting Http Server");
    HttpServer::new(move || {
            // Create a new CORS middleware
            // The permissive method allows any origin, any headers, and any methods
            let cors = Cors::permissive();

            App::new()
                .wrap(cors)
                .wrap(SessionMiddleware::new(redis_store.clone(), secret_key.clone()))
                .data(db_pool.clone())
                .data(oauth_client.clone())
                .service(ouath_callback)
                // .service(get_users)
            }
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}