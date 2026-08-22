#!/bin/bash

# 0. Ubicación dinámica del script y directorio de trabajo
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 1. Definición de Rutas dinámicas (relativas al script o mediante variable de entorno)
ANGULAR_PROJECT_ROOT="${ANGULAR_PROJECT_ROOT:-$(cd "$SCRIPT_DIR/../web-wrapp" 2>/dev/null && pwd)}"
ANGULAR_ASSETS_PATH="$ANGULAR_PROJECT_ROOT/src/assets/retail-shop"

echo -e "\033[1;34mRuta de destino: $ANGULAR_ASSETS_PATH\033[0m"

# 2. Limpieza y Creación de Carpeta Destino
echo -e "\033[1;33m--- Preparando directorio de destino ---\033[0m"
# Forzamos la creación del árbol de carpetas completo
mkdir -p "$ANGULAR_ASSETS_PATH"

if [ -d "$ANGULAR_ASSETS_PATH" ]; then
    # Limpia el contenido previo
    rm -rf "$ANGULAR_ASSETS_PATH"/*
    echo "Carpeta preparada."
fi

# 3. Generación de interfaces de TypeScript (ts-rs)
echo -e "\033[1;36m--- Generando interfaces de TypeScript (ts-rs) ---\033[0m"
cargo test

# 4. Compilación para Web (WASM)
echo -e "\033[1;36m--- Compilando para Web (WASM) ---\033[0m"
# Usamos la ruta absoluta para evitar errores de contexto
wasm-pack build --target web --out-dir "$ANGULAR_ASSETS_PATH"

# 5. Sincronización de modelos (Opcional si wasm-pack ya escribe ahí)
echo -e "\033[1;36m--- Sincronizando modelos adicionales ---\033[0m"
if [ -d "bindings" ]; then
    cp -R bindings/*.ts "$ANGULAR_ASSETS_PATH/" 2>/dev/null || echo "No hay .ts en bindings para copiar."
    echo -e "\033[1;32mModelos sincronizados.\033[0m"
else
    echo -e "\033[1;31mAviso: No se encontró carpeta 'bindings', continuando...\033[0m"
fi

# 6. Actualización de Dependencias Angular
echo -e "\033[1;33m--- Actualizando Angular ---\033[0m"
if [ -d "$ANGULAR_PROJECT_ROOT" ]; then
    cd "$ANGULAR_PROJECT_ROOT"
    npm install
    cd "$SCRIPT_DIR"
else
    echo -e "\033[1;31mError: El root de Angular no existe en $ANGULAR_PROJECT_ROOT\033[0m"
fi

echo -e "\033[1;32m\n¡Proceso completado!\033[0m"