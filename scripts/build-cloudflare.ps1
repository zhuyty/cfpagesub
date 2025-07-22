# PowerShell script for building subconverter-rs for Cloudflare Pages
param(
    [switch]$SkipWasmPack,
    [switch]$SkipPnpm
)

Write-Host "🚀 Building subconverter-rs for Cloudflare Pages" -ForegroundColor Green

# Check if wasm-pack is installed
if (-not $SkipWasmPack -and -not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Installing wasm-pack..." -ForegroundColor Yellow
    try {
        Invoke-WebRequest -Uri "https://rustwasm.github.io/wasm-pack/installer/init.sh" -OutFile "wasm-pack-init.sh"
        bash wasm-pack-init.sh
        Remove-Item "wasm-pack-init.sh"
    } catch {
        Write-Error "Failed to install wasm-pack. Please install it manually."
        exit 1
    }
}

# Check if pnpm is installed
if (-not $SkipPnpm -and -not (Get-Command pnpm -ErrorAction SilentlyContinue)) {
    Write-Host "📦 Installing pnpm..." -ForegroundColor Yellow
    npm install -g pnpm
}

# Build WASM package for web target
Write-Host "🔧 Building WASM package..." -ForegroundColor Blue
try {
    wasm-pack build --release --target web --out-dir pkg
} catch {
    Write-Error "Failed to build WASM package"
    exit 1
}

# Update package.json in pkg for Cloudflare compatibility
Write-Host "📝 Updating WASM package.json..." -ForegroundColor Blue
Push-Location pkg

$version = (Select-String -Path "../Cargo.toml" -Pattern 'version\s*=\s*"([^"]+)"').Matches[0].Groups[1].Value

$packageJson = @{
    name = "subconverter-wasm"
    version = $version
    files = @(
        "subconverter_bg.wasm",
        "subconverter.js", 
        "subconverter.d.ts",
        "snippets/"
    )
    module = "subconverter.js"
    types = "subconverter.d.ts"
    sideEffects = @("./snippets/*")
} | ConvertTo-Json -Depth 3

$packageJson | Out-File -FilePath "package.json" -Encoding UTF8
Pop-Location

# Copy WASM files to www project
Write-Host "📂 Copying WASM files to www project..." -ForegroundColor Blue
$wasmDir = "www/node_modules/subconverter-wasm"
if (Test-Path $wasmDir) {
    Remove-Item -Recurse -Force $wasmDir
}
New-Item -ItemType Directory -Force -Path $wasmDir | Out-Null
Copy-Item -Recurse -Path "pkg/*" -Destination $wasmDir

# Install dependencies and build Next.js app
Write-Host "📦 Installing www dependencies..." -ForegroundColor Blue
Push-Location www
try {
    pnpm install
    
    Write-Host "🏗️ Building Next.js application..." -ForegroundColor Blue
    pnpm build
    
    Write-Host "✅ Build completed successfully!" -ForegroundColor Green
    Write-Host "📁 Output directory: www/.next" -ForegroundColor Cyan
} catch {
    Write-Error "Failed to build Next.js application"
    exit 1
} finally {
    Pop-Location
}
