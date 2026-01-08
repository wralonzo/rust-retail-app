# build_android.ps1
$env:ANDROID_NDK_HOME = "C:\Android\ndk\23.1.7779620"

Write-Host "🚧 Compilando para Android..." -ForegroundColor Cyan
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi build --release

Write-Host "✨ Generando Bindings de Kotlin..." -ForegroundColor Magenta
cargo run --bin uniffi-bindgen generate --library target/aarch64-linux-android/release/librust_retail.so --language kotlin --out-dir ./generated-kotlin --no-format

Write-Host "✅ ¡Proceso completado! Archivos listos en /target y /generated-kotlin" -ForegroundColor Green