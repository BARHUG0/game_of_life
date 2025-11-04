# Game of Life

Implementación del Juego de la Vida de Conway en Rust usando Raylib.
[Gameplay](https://www.youtube.com/watch?v=b5in-MjPctg)

## Requisitos

- Rust (edition 2024)
- raylib 5.5.1

## Instalación

```bash
cargo build --release
```

## Ejecución

```bash
cargo run
```

## Características

- Simulación en tiempo real del autómata celular de Conway
- Ventana de 1900x1000 píxeles (1_900_000 células)
- Estado inicial aleatorio (75% células vivas)
- Colores: células vivas (dorado), células muertas (azul oscuro)

## Estructura

- `conway.rs` - Lógica del autómata celular
- `framebuffer.rs` - Manejo del buffer de píxeles
- `main.rs` - Loop principal y renderizado
