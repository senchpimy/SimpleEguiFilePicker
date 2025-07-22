
# Simple File Picker

![Badge](https://img.shields.io/badge/Rust-eframe%20%2B%20egui-orange?logo=rust)

**Files** es un explorador de archivos gráfico simple escrito en Rust usando [`eframe`](https://crates.io/crates/eframe) y [`egui`](https://crates.io/crates/egui). Permite navegar directorios desde una interfaz rápida, con filtrado por tipo de archivo y previsualización de carpetas.

## Características

* Interfaz minimalista y fluida basada en egui.
* Navegación por carpetas.
* Búsqueda en tiempo real.
* Filtro por extensión (imágenes, videos, documentos, o personalizada).
* Selector clickeable de archivos.
* Soporte para cierre de ventana y retorno de ruta por stdout.

## Requisitos

* Rust (edition 2021 o superior)
* [`eframe`](https://crates.io/crates/eframe), [`egui_extras`](https://crates.io/crates/egui_extras)
* [`directories`](https://crates.io/crates/directories)

## Uso

```bash
cargo run -- [tipo | .ext1 .ext2 ...]
```

### Ejemplos

```bash
cargo run -- images         # Muestra solo archivos .png, .jpg, .jpeg, .gif
cargo run -- videos         # Muestra .mp4 y .webm
cargo run -- .txt .md       # Muestra archivos .txt y .md
cargo run                   # Sin filtro, muestra todo
```

Cuando se selecciona un archivo, su ruta completa se imprime por `stdout` y la aplicación se cierra.

## Estado

Funcional y estable para uso básico. Ideal como selector de archivos embebido en otros proyectos Rust con GUI.
