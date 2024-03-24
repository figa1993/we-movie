#!/bin/bash

# Install rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the React dependencies
cd we-movie-frontend; npm install