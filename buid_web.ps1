# 1. Definición de Rutas
$ANGULAR_PROJECT_ROOT = "C:\Users\BDGSA\Documents\projects\angular\front-spa\web-retail"
$ANGULAR_ASSETS_PATH = "$ANGULAR_PROJECT_ROOT\src\assets\retail-shop"

# 2. Limpieza de Carpeta Destino
Write-Host "--- Iniciando limpieza de archivos antiguos ---" -ForegroundColor Yellow
if (Test-Path $ANGULAR_ASSETS_PATH) {
    # Borra el contenido pero mantiene la carpeta para evitar problemas de permisos
    Remove-Item -Path "$ANGULAR_ASSETS_PATH\*" -Recurse -Force
    Write-Host "Carpeta de destino limpiada." -ForegroundColor Gray
} else {
    New-Item -ItemType Directory -Path $ANGULAR_ASSETS_PATH -Force | Out-Null
}

# 3. Generación de interfaces de TypeScript (ts-rs)
Write-Host "--- Generando interfaces de TypeScript (ts-rs) ---" -ForegroundColor Cyan
cargo test

# 4. Compilación para Web (WASM)
Write-Host "--- Compilando para Web (WASM) ---" -ForegroundColor Cyan
# Compilamos directamente a la carpeta de Angular
wasm-pack build --target web --out-dir $ANGULAR_ASSETS_PATH

# 5. Sincronización de modelos
Write-Host "--- Sincronizando modelos con Angular ---" -ForegroundColor Cyan
if (Test-Path "bindings") {
    Copy-Item -Path "bindings\*.ts" -Destination $ANGULAR_ASSETS_PATH -Force
    Write-Host "Modelos copiados con éxito." -ForegroundColor Green
} else {
    Write-Host "Error: No se encontró la carpeta 'bindings'. Revisa el test de exportación." -ForegroundColor Red
    exit 1
}


# 6. Actualización de Dependencias Angular
Write-Host "--- Intentando actualizar dependencias en Angular ---" -ForegroundColor Yellow
Push-Location $ANGULAR_PROJECT_ROOT

#6 Usamos -ErrorAction Stop para que el bloque try/catch capture el error de npm
try {
    # Ejecutamos npm install capturando la salida
    npm install --loglevel error
    
    if ($LASTEXITCODE -ne 0) { 
        throw "NPM salió con código de error $LASTEXITCODE" 
    }
    
    Write-Host "Dependencias actualizadas con éxito." -ForegroundColor Green
} catch {
    Write-Host "`n[ERROR CRÍTICO EN NPM]" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor White
    Write-Host "Sugerencia: Cierra cualquier terminal que esté ejecutando 'ng serve' e intenta de nuevo." -ForegroundColor Yellow
}
Pop-Location

Write-Host "`n¡Proceso completado!" -ForegroundColor DarkGreen