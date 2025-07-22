#!/bin/bash
set -e

echo "🚀 Building Next.js app for Cloudflare Pages"

# Navigate to www directory
cd www

# Install dependencies
echo "📦 Installing dependencies..."
npm install --include=dev

# Build the application
echo "🏗️ Building Next.js application..."
npm run build

# Clean up large files to meet Cloudflare Pages limits
echo "🧹 Cleaning up large files..."

# Remove cache directory
rm -rf .next/cache 2>/dev/null || true

# Remove large webpack files
find .next -name "*.pack" -size +20M -delete 2>/dev/null || true
find .next -name "*.map" -size +10M -delete 2>/dev/null || true

# Remove server chunks that are too large
find .next/server -name "*.js" -size +20M -delete 2>/dev/null || true

# Remove trace files
rm -rf .next/trace 2>/dev/null || true

# Show final size
echo "📊 Final build size:"
if [ -d "out" ]; then
    du -sh out 2>/dev/null || echo "Out directory size check failed"
    echo "📁 Contents of out directory:"
    ls -la out/ 2>/dev/null || echo "Cannot list out directory"
elif [ -d ".next" ]; then
    du -sh .next 2>/dev/null || echo "Next directory size check failed"
    echo "📁 Contents of .next directory:"
    ls -la .next/ 2>/dev/null || echo "Cannot list .next directory"
else
    echo "No build output directory found"
fi

echo "✅ Build completed and cleaned!"
