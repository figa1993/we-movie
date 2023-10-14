#!/bin/sh
# Start the Docker container
docker-compose up -d
# Start the Rust backend
cd we-movie-backend
cargo run
# Start the React frontend
cd ../we-movie-frontend
npm start