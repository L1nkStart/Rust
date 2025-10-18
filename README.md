# Aplicación de Terminal en Rust

Esta es una aplicación de consola en Rust que ofrece tres funcionalidades principales:
1. **Generador de secuencia de Fibonacci**: Genera los primeros N números de la secuencia de Fibonacci
2. **Convertidor de temperatura**: Convierte entre Celsius y Fahrenheit en ambas direcciones
3. **Juego de adivinanza**: Juego interactivo donde debes adivinar un número aleatorio del 1 al 10

## Requisitos

Para ejecutar esta aplicación, necesitas tener Rust instalado en tu sistema.

### Instalación de Rust

1. **Windows**: Descarga e instala Rust desde [https://rustup.rs/](https://rustup.rs/)
   - Ejecuta el instalador `rustup-init.exe`
   - Sigue las instrucciones en pantalla
   - Reinicia tu terminal después de la instalación

2. **Verificar la instalación**:
   ```bash
   rustc --version
   cargo --version
   ```

## Cómo ejecutar la aplicación

1. Abre una terminal en el directorio del proyecto
2. Ejecuta el siguiente comando:
   ```bash
   cargo run
   ```

## Uso de la aplicación

### Menú principal
Al ejecutar la aplicación, verás un menú con las siguientes opciones:
```
=== APLICACIÓN DE TERMINAL ===
1. Generar secuencia de Fibonacci
2. Convertidor de temperatura
3. Juego de adivinanza
4. Salir
```

### 1. Generador de Fibonacci
- Selecciona la opción 1
- Ingresa el número de términos que deseas generar
- La aplicación mostrará la secuencia completa

**Ejemplo:**
```
Ingresa cuántos números de la secuencia quieres generar: 8
Secuencia de Fibonacci con 8 números:
1: 0
2: 1
3: 1
4: 2
5: 3
6: 5
7: 8
8: 13
```

### 2. Convertidor de temperatura
- Selecciona la opción 2
- Elige el tipo de conversión:
  - 1: Celsius a Fahrenheit
  - 2: Fahrenheit a Celsius
- Ingresa la temperatura a convertir

**Ejemplos:**
```
Celsius a Fahrenheit:
Ingresa la temperatura en Celsius: 25
25°C = 77.00°F

Fahrenheit a Celsius:
Ingresa la temperatura en Fahrenheit: 77
77°F = 25.00°C
```

### 3. Juego de adivinanza
- Selecciona la opción 3
- El juego genera un número secreto aleatorio entre 1 y 10
- Tienes exactamente 3 intentos para adivinarlo
- Después de cada intento incorrecto, recibes una pista si el número es mayor o menor
- Si no adivinas en los 3 intentos, se revela el número correcto

**Ejemplo de juego exitoso:**
```
--- Juego de Adivinanza ---
¡Bienvenido al juego de adivinanza!
He pensado un número entre 1 y 10.
Tienes 3 intentos para adivinarlo.

Intentos restantes: 3
Ingresa tu número (1-10): 5
El número que pensé es más PEQUEÑO que 5.

Intentos restantes: 2
Ingresa tu número (1-10): 3
El número que pensé es más GRANDE que 3.

Intentos restantes: 1
Ingresa tu número (1-10): 4
¡Felicidades! ¡Adivinaste el número 4!
¡Lo hiciste en el último intento!
```

**Ejemplo de juego fallido:**
```
Intentos restantes: 1
Ingresa tu número (1-10): 8
¡Se acabaron los intentos! El número era: 6
¡Mejor suerte la próxima vez!
```

## Estructura del proyecto

```
aplicacion_terminal/
├── Cargo.toml          # Configuración del proyecto Rust
├── README.md           # Este archivo
└── src/
    └── main.rs         # Código fuente principal
```


