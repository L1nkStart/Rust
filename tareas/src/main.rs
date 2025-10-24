use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        mostrar_menu();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        
        match input.trim() {
            "1" => generar_fibonacci(),
            "2" => convertir_temperatura(),
            "3" => juego_adivinanza(),
            "4" => {
                println!("¡Gracias por usar la aplicación!");
                break;
            }
            _ => println!("Opción no válida. Por favor, selecciona 1, 2, 3 o 4."),
        }
        
        println!();
    }
}

fn mostrar_menu() {
    println!("=== APLICACIÓN DE TERMINAL ===");
    println!("1. Generar secuencia de Fibonacci");
    println!("2. Convertidor de temperatura");
    println!("3. Juego de adivinanza");
    println!("4. Salir");
    println!("Selecciona una opción (1-4): ");
}

fn generar_fibonacci() {
    println!("\n--- Generador de Secuencia de Fibonacci ---");
    println!("Ingresa cuántos números de la secuencia quieres generar: ");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    
    let n: u32 = match input.trim().parse() {
        Ok(num) => {
            if num == 0 {
                println!("El número debe ser mayor que 0.");
                return;
            }
            num
        }
        Err(_) => {
            println!("Por favor, ingresa un número válido.");
            return;
        }
    };
    
    println!("\nSecuencia de Fibonacci con {} números:", n);
    
    let mut a = 0;
    let mut b = 1;
    
    for i in 0..n {
        if i == 0 {
            println!("{}: {}", i + 1, a);
        } else if i == 1 && n > 1 {
            println!("{}: {}", i + 1, b);
        } else {
            let siguiente = a + b;
            a = b;
            b = siguiente;
            println!("{}: {}", i + 1, b);
        }
    }
}

fn convertir_temperatura() {
    println!("\n--- Convertidor de Temperatura ---");
    println!("1. Celsius a Fahrenheit");
    println!("2. Fahrenheit a Celsius");
    println!("Selecciona el tipo de conversión (1-2): ");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    
    match input.trim() {
        "1" => celsius_a_fahrenheit(),
        "2" => fahrenheit_a_celsius(),
        _ => {
            println!("Opción no válida. Por favor, selecciona 1 o 2.");
            return;
        }
    }
}

fn celsius_a_fahrenheit() {
    println!("Ingresa la temperatura en Celsius: ");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    
    let celsius: f64 = match input.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Por favor, ingresa un número válido.");
            return;
        }
    };
    
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{}°C = {:.2}°F", celsius, fahrenheit);
}

fn fahrenheit_a_celsius() {
    println!("Ingresa la temperatura en Fahrenheit: ");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    
    let fahrenheit: f64 = match input.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Por favor, ingresa un número válido.");
            return;
        }
    };
    
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F = {:.2}°C", fahrenheit, celsius);
}

fn juego_adivinanza() {
    println!("\n--- Juego de Adivinanza ---");
    println!("¡Bienvenido al juego de adivinanza!");
    println!("He pensado un número entre 1 y 10.");
    println!("Tienes 3 intentos para adivinarlo.\n");
    
    // Generar número aleatorio del 1 al 10 usando el tiempo actual
    let numero_secreto = generar_numero_aleatorio();
    let mut intentos_restantes = 3;
    let mut adivinado = false;
    
    while intentos_restantes > 0 && !adivinado {
        println!("Intentos restantes: {}", intentos_restantes);
        println!("Ingresa tu número (1-10): ");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        
        let numero_usuario: u32 = match input.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 10 {
                    println!("Por favor, ingresa un número entre 1 y 10.");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue;
            }
        };
        
        intentos_restantes -= 1;
        
        if numero_usuario == numero_secreto {
            adivinado = true;
            println!("¡Felicidades! ¡Adivinaste el número {}!", numero_secreto);
            if intentos_restantes == 2 {
                println!("¡Lo hiciste en el primer intento!");
            } else if intentos_restantes == 1 {
                println!("¡Lo hiciste en el segundo intento!");
            } else {
                println!("¡Lo hiciste en el último intento!");
            }
        } else if intentos_restantes > 0 {
            if numero_usuario < numero_secreto {
                println!("El número que pensé es más GRANDE que {}.", numero_usuario);
            } else {
                println!("El número que pensé es más PEQUEÑO que {}.", numero_usuario);
            }
            println!();
        }
    }
    
    if !adivinado {
        println!("¡Se acabaron los intentos! El número era: {}", numero_secreto);
        println!("¡Mejor suerte la próxima vez!");
    }
}

fn generar_numero_aleatorio() -> u32 {
    let tiempo = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error con el tiempo del sistema");
    
    let nano = tiempo.as_nanos();
    
    ((nano % 10) + 1) as u32
}
