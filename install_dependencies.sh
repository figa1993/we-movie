#!/bin/bash

# Install redis
sudo apt get update; \
apt-get install redis-server

##### Configure redis #####
# create directory used by redis at runtime
sudo mkdir -p /var/redis;

# Install rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the React dependencies
cd we-movie-frontend; npm install