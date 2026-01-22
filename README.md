# Rust Retail App

Biblioteca **Core** escrita en Rust diseñada para ser el motor de aplicaciones de retail multiplataforma. Proporciona lógica de negocio compartida, gestión de estado y persistencia para clientes Web (Wasm), Android (Kotlin) e iOS (Swift).

## 🚀 Características Principales

- **Multiplataforma**: Un único código base en Rust que se compila a WebAssembly y librerías nativas móviles.
- **Arquitectura Limpia**: Separación clara entre Dominio, Casos de Uso e Infraestructura.
- **Autenticación**:
  - Login tradicional (Usuario/Contraseña).
  - Integración con Google Sign-In.
- **Gestión de Sesiones**: Persistencia segura de tokens y datos de usuario.
- **Almacenamiento Local**: Abstracción sobre almacenamiento seguro y bases de datos locales (SQLite/IndexedDB).
- **Cliente HTTP**: Cliente robusto basado en `reqwest`.

## 🛠️ Tecnologías

Este proyecto utiliza tecnologías modernas del ecosistema Rust:

*   **[Rust](https://www.rust-lang.org/)**: Lenguaje principal.
*   **[Tokio](https://tokio.rs/)**: Runtime asíncrono.
*   **[Reqwest](https://docs.rs/reqwest/)**: Cliente HTTP asíncrono.
*   **[Serde](https://serde.rs/)**: Serialización/Deserialización de datos.
*   **[UniFFI](https://github.com/mozilla/uniffi-rs)**: Generación de bindings para Kotlin y Swift.
*   **[Wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)**: Interoperabilidad con JavaScript/TypeScript.
*   **[Rusqlite](https://github.com/rusqlite/rusqlite)**: Base de datos SQLite embebida (para plataformas nativas).

## 📂 Estructura del Proyecto

```
rust-retail-app/
├── src/
│   ├── domain/           # Entidades, Modelos y Definiciones de Puertos (Interfaces)
│   ├── use_cases/        # Lógica de Negocio Pura (Login, Logout, Fetch Users)
│   ├── infrastructure/   # Implementaciones de los Puertos (HTTP, DB, Storage)
│   ├── bridge/           # Código puente para UniFFI y Wasm
│   ├── utils/            # Utilidades generales
│   └── lib.rs            # Punto de entrada y configuración de exportaciones
├── bindings/             # Archivos generados para Kotlin/Swift
└── Cargo.toml            # Dependencias y configuración del workspace
```

## 📦 Construcción y Ejecución

### Requisitos Previos

- Rust (latest stable)
- Android SDK / NDK (para Android)
- Xcode (para iOS)
- `wasm-pack` (para Web)

### Comandos de Build

El proyecto incluye scripts para facilitar la compilación:

- **Web**: `wasm-pack build --target web`
- **Android**: `./build_android.ps1` (o script equivalente)
- **iOS**: Configurado mediante `cargo-xcode` o scripts personalizados.

## 🤝 Contribución

1.  Haz un Fork del repositorio.
2.  Crea una rama para tu feature (`git checkout -b feature/nueva-feature`).
3.  Haz Commit de tus cambios (`git commit -m 'Añade nueva feature'`).
4.  Haz Push a la rama (`git push origin feature/nueva-feature`).
5.  Abre un Pull Request.

## 📄 Licencia

Este proyecto está bajo la Licencia **MIT**.
