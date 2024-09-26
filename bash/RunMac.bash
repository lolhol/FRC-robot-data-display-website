#!bin/bash

echo "Starting server using macOS"

echo "Installing dependencies for Rust..."

if ! command -v rustup &> /dev/null
then
    echo "rustup not found, installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup update
else
    rustup update
fi

echo "Building backend..."
cargo build --release

echo "Downloading Server Deps..."

if ! command -v node &> /dev/null
then
    echo "Node not found, installing..."
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
    nvm install 20
fi

echo "Downloading deps..."

npm install

echo "Starting backend and Server..."

(cargo run --release) & (npm run dev)
