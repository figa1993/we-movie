#!/bin/sh
# start the redis server
echo "starting redis server"
sudo redis-server /etc/redis/redis.conf

# Start the Docker container
docker-compose up -d

# Wait for PostgreSQL to initialize
echo "Waiting for PostgreSQL to initialize..."
sleep 20

# Start the Rust backend
cd we-movie-backend 
cargo run & 


# Start the React frontend
cd ../we-movie-frontend 
npm start &