# Proyecto de Sistema Solar con Renderizado 3D

## Descripción General

Este proyecto implementa un renderizador 3D desde cero en Rust utilizando raylib-rs, presentando un sistema solar completo con mecánica orbital kepleriana realista. El sistema incluye un agujero negro central con disco de acreción, múltiples planetas con shaders procedurales únicos, y una nave espacial controlable. Todo el pipeline de renderizado está implementado manualmente, incluyendo transformaciones de matriz, proyección de perspectiva, rasterización de triángulos y shaders personalizados.

## Características Principales

### Shaders Procedurales
- **Rocky (Planeta Rocoso)**: Shader estilo Tierra/Marte con continentes, océanos, nubes dinámicas y casquetes polares
- **GasGiant (Gigante Gaseoso)**: Shader inspirado en Júpiter con bandas atmosféricas, corrientes de chorro, La Gran Mancha Roja y tormentas secundarias
- **Ringed (Planeta Anillado)**: Shader estilo Saturno con superficie helada y sistema de anillos procedurales con bandas detalladas
- **Magenta (GJ 504 b)**: Gigante gaseoso magenta/púrpura con patrones de tormenta luminosos y efectos de pulso polar
- **WaterWorld (Kepler-22 b)**: Mundo oceánico con variaciones de profundidad, islas dispersas, nubes dinámicas y reflejos especulares en agua
- **BlackHole (Agujero Negro)**: Shader de agujero negro con esfera de fotones, radiación de Hawking, distorsión espaciotemporal y efectos de borde
- **AccretionDisk (Disco de Acreción)**: Disco de acreción giratorio con gradientes de temperatura, patrones espirales, turbulencia y efectos Doppler

### Orbitas dibujadas
- Se pueden observar las orbitas de los planetas

### Sistema de Cámara
- Cámara que sigue la nave espacial en tercera persona
- Controles intuitivos (WASD para movimiento, flechas para rotación)

### Efectos Visuales
- Campo de estrellas de fondo
- Estelas de trayectoria
- Colores personalizados para cada tipo de planeta

## Instalación

1. Clona el repositorio:
```bash
git clone <url-del-repositorio>
cd <nombre-del-proyecto>
```

2. Asegúrate de tener Rust instalado (versión 1.70 o superior):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Compila y ejecuta el proyecto:
```bash
cargo run --release
```

## Dependencias

### Crates de Rust
- **raylib** (5.0): Biblioteca de ventanas, entrada y estructuras de datos básicas
- **tobj**: Carga de archivos .OBJ para modelos 3D

### Recursos Externos
- **Modelos 3D** (formato .OBJ):
  - `models/sphere.obj`: Esfera base para planetas y agujero negro
  - `models/torus.obj`: Toro para el disco de acreción
  - `models/nave.obj`: Modelo de nave espacial

- **Texturas** (formato PNG):
  - `textures/perlin_noise.png`: Ruido Perlin para distorsión espaciotemporal
  - `textures/accretion_noise.png`: Textura para patrones de acreción
  - `textures/turbulence_noise.png`: Textura para efectos de turbulencia

### Requisitos del Sistema
- Sistema operativo: Windows, Linux o macOS
- Memoria RAM: 4GB mínimo (8GB recomendado)
- GPU: Compatible con OpenGL 3.3 o superior

## Estructura del Proyecto

```
proyecto/
│
├── src/
│   ├── main.rs                 # Punto de entrada, game loop y renderizado principal
│   ├── framebuffer.rs          # Buffer de color y profundidad
│   ├── render.rs               # Pipeline de renderizado 3D completo
│   ├── shader.rs               # Sistema de shaders (vertex y fragment)
│   ├── camera.rs               # (Integrada en render.rs) - Cámara y proyección
│   ├── matrix.rs               # Operaciones de matrices 4x4 y transformaciones
│   ├── vertex.rs               # Estructura de vértice (posición, normal, UV)
│   ├── fragment.rs             # Estructura de fragmento para rasterización
│   ├── obj.rs                  # Cargador de archivos .OBJ
│   ├── uniforms.rs             # Variables uniformes para shaders
│   ├── texture_manager.rs      # Manejo y muestreo de texturas
│   ├── solar_system.rs         # Sistema solar, planetas y cuerpos centrales
│   ├── orbital_math.rs         # Matemática orbital kepleriana
│   ├── orbital_renderer.rs     # Renderizado de órbitas y estelas
│   ├── spaceship.rs            # Física y control de nave espacial
│   ├── line.rs                 # Algoritmo de línea de Bresenham
│   ├── triangle.rs             # Rasterización de triángulos
│   └── light.rs                # Sistema de iluminación básico
│
├── models/
│   ├── sphere.obj              # Modelo de esfera
│   ├── torus.obj               # Modelo de toro
│   └── nave.obj                # Modelo de nave espacial
│
├── textures/
│   ├── perlin_noise.png        # Textura de ruido Perlin
│   ├── accretion_noise.png     # Textura para disco de acreción
│   └── turbulence_noise.png    # Textura de turbulencia
│
├── Cargo.toml                  # Configuración de dependencias
└── README.md                   # Este archivo
```

### Descripción de Módulos Principales

#### Renderizado
- **framebuffer.rs**: Gestiona el buffer de color (Image) y el z-buffer para prueba de profundidad
- **render.rs**: Implementa el pipeline completo de transformación 3D y rasterización
- **shader.rs**: Define todos los shaders procedurales y su lógica de ejecución

#### Matemática y Física
- **matrix.rs**: Operaciones matriciales, view matrix, projection matrix, viewport matrix
- **orbital_math.rs**: Ecuación de Kepler, conversión de anomalías, cálculo de posiciones orbitales
- **spaceship.rs**: Física de vuelo, interpolación de rotación, sistema de banking

#### Sistema Solar
- **solar_system.rs**: Gestión de planetas, parámetros orbitales, actualización de posiciones
- **orbital_renderer.rs**: Proyección 3D a 2D de órbitas y estelas

#### Utilidades
- **obj.rs**: Parser de archivos Wavefront .OBJ
- **texture_manager.rs**: Muestreo de texturas con wrapping y clamping
- **uniforms.rs**: Container para variables compartidas entre shaders

## Controles

### Nave Espacial
- **W/S**: Acelerar adelante/atrás
- **A/D**: Desplazamiento lateral (strafe)
- **Flechas Arriba/Abajo**: Pitch (cabeceo)
- **Flechas Izquierda/Derecha**: Yaw (guiñada)

### Sistema Solar
- **ESPACIO**: Pausar/reanudar simulación
- **O**: Mostrar/ocultar órbitas
- **T**: Mostrar/ocultar estelas

