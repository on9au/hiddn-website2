#!/usr/bin/env bash

set -e

echo "🔧 First-time setup starting..."

export NVM_DIR="$HOME/.nvm"

# Check for Rust
if ! command -v cargo &>/dev/null; then
    echo "🚀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
else
    echo "✅ Rust is already installed."
fi

# Install typeshare-cli
if ! command -v typeshare &>/dev/null; then
    echo "📦 Installing typeshare-cli..."
    cargo install typeshare-cli
else
    echo "✅ typeshare-cli is already installed."
fi

# Install sqlx-cli
if ! command -v sqlx &>/dev/null; then
    echo "📦 Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features mysql
else
    echo "✅ sqlx-cli is already installed."
fi

# Setup Node.js with nvm
if [ ! -s "$NVM_DIR/nvm.sh" ]; then
    echo "⬇️ Installing nvm..."
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
else
    echo "✅ nvm is already installed."
fi

# Load nvm and install Node
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
nvm install
nvm use

# Install npm deps
echo "📦 Installing npm dependencies..."
cd client
npm install
cd ..

# Create .env if missing
if [ ! -f .env ]; then
    echo "📄 Creating .env from .env.example..."
    cp .env.example .env
    echo "✏️ Please edit the .env file to set proper values."
else
    echo "✅ .env already exists."
fi

# Ask to run sqlx prepare
read -p "📂 Do you want to prepare SQLx offline data now? [y/N]: " sqlx_prep
if [[ "$sqlx_prep" =~ ^[Yy]$ ]]; then
    echo "📦 Running sqlx prepare..."
    cargo sqlx prepare
fi

# Ask to start MySQL Docker container
# ✅ Ask to start MySQL Docker container
read -p "🚀 Do you want to start the MySQL Docker container now? [y/N]: " mysql_start
if [[ "$mysql_start" =~ ^[Yy]$ ]]; then
    echo "📦 Starting MySQL container..."
    sudo bash ./mysql_docker.sh start || {
        echo "❌ Failed to start MySQL container."
        exit 1
    }
fi

echo "✅ Setup complete!"
