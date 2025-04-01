# Calculadora Básica en Rust

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

Una calculadora de terminal simple escrita en Rust que realiza operaciones aritméticas básicas.

## Características

- Operaciones básicas: suma, resta, multiplicación y división
- Interfaz de línea de comandos intuitiva
- Manejo de errores para entradas inválidas
- Detección de división por cero
- Resultados con precisión de punto flotante

## Requisitos

- Rust 1.70.0 o superior
- Cargo (gestor de paquetes de Rust)

## Instalación

1. Clona el repositorio:
```bash
git clone https://github.com/EvanM-Git/calculator.git
cd calculator
```

2. Compila el proyecto:
```bash
cargo build --release
```

3. Ejecuta el programa:
```bash
cargo run --release
```

O instala globalmente:
```bash
cargo install --path .
```

## Uso

1. Selecciona una operación (1-4):
   - 1: Sumar
   - 2: Restar
   - 3: Multiplicar
   - 4: Dividir

2. Ingresa los dos números cuando se te solicite

3. El programa mostrará el resultado de la operación

4. Para salir, presiona Ctrl + C

## Ejemplo de ejecución

```
--- Calculadora Básica ---
1. Sumar
2. Restar
3. Multiplicar
4. Dividir
Presiona Control + C si quieres salir

Selecciona una opción (1-4): 1
Por favor dame el primer valor de tu operación: 5
Ahora el segundo valor: 3
El resultado de tu operación es: 8
```

## Estructura del proyecto

```
calculadora-rust/
├── src/
│   └── main.rs      # Código fuente principal
├── Cargo.toml       # Configuración del proyecto
├── README.md        # Este archivo
└── LICENSE          # Licencia MIT
```
