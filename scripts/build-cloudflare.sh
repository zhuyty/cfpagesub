#!/bin/bash
set -e

echo "🚀 Building subconverter-rs for Cloudflare Pages"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Check if pnpm is installed
if ! command -v pnpm &> /dev/null; then
    echo "📦 Installing pnpm..."
    npm install -g pnpm
fi

# Build WASM package for web target
echo "🔧 Building WASM package..."
wasm-pack build --release --target web --out-dir pkg

# Update package.json in pkg for Cloudflare compatibility
echo "📝 Updating WASM package.json..."
cd pkg
cat > package.json << EOF
{
  "name": "subconverter-wasm",
  "version": "$(grep -m 1 "version" ../Cargo.toml | sed 's/.*"\(.*\)".*/\1/')",
  "files": [
    "subconverter_bg.wasm",
    "subconverter.js",
    "subconverter.d.ts",
    "snippets/"
  ],
  "module": "subconverter.js",
  "types": "subconverter.d.ts",
  "sideEffects": [
    "./snippets/*"
  ]
}
EOF
cd ..

# Copy WASM files to www project
echo "📂 Copying WASM files to www project..."
mkdir -p www/node_modules/subconverter-wasm
rm -rf www/node_modules/subconverter-wasm/*
cp -r pkg/* www/node_modules/subconverter-wasm/

# Install dependencies and build Next.js app
echo "📦 Installing www dependencies..."
cd www
pnpm install

echo "🏗️ Building Next.js application..."
pnpm build

echo "✅ Build completed successfully!"
echo "📁 Output directory: www/.next"
