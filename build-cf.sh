#!/bin/bash
set -e

echo "🚀 Building subconverter-rs for Cloudflare Pages"

# Install Rust if not available
if ! command -v rustc &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Install wasm-pack if not available
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Install pnpm if not available
if ! command -v pnpm &> /dev/null; then
    echo "📦 Installing pnpm..."
    npm install -g pnpm
fi

# Build WASM package
echo "🔧 Building WASM package..."
wasm-pack build --release --target web --out-dir pkg

# Copy WASM files to www project
echo "📂 Copying WASM files..."
mkdir -p www/node_modules/subconverter-wasm
cp -r pkg/* www/node_modules/subconverter-wasm/

# Build Next.js app
echo "🏗️ Building Next.js application..."
cd www
pnpm install
pnpm build

echo "✅ Build completed!"
