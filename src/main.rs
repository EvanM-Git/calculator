use std::io;

fn main() {
    println!("\n--- Calculadora Básica ---");
    println!("1. Sumar");
    println!("2. Restar");
    println!("3. Multiplicar");
    println!("4. Dividir");
    println!("Presiona Control + C si quieres salir");

    let mut numero1_buff = String::new();
    let mut numero2_buff = String::new();
    let mut numero1: f64 = 0.0;
    let mut numero2: f64 = 0.0;
    let mut opcion_buff = String::new();
    let mut opcion: u8 = 0;
    let mut resultado: f64 = 0.0;

    loop {
        println!("Selecciona una opción (1-5): ");
        io::stdin()
            .read_line(&mut opcion_buff)
            .expect("No se pudo leer input de usuario");

        opcion = match opcion_buff.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    loop {
        println!("Porfavor dame el primer valor de tu operacion: ");
        io::stdin()
            .read_line(&mut numero1_buff)
            .expect("Error leer input de usuario");

        //Convertir
        numero1 = match numero1_buff.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Porfavor introdusca un numero valido");
                continue;
            }
        };

        println!("Ahora el segundo valor: ");
        io::stdin()
            .read_line(&mut numero2_buff)
            .expect("Error leer input de usuario");

        //Convertir
        numero2 = match numero2_buff.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Porfavor introdusca un numero valido");
                continue;
            }
        };

        break;
    }

    match opcion {
        1 => resultado = numero1 + numero2,
        2 => resultado = numero1 - numero2,
        3 => resultado = numero1 * numero2,
        4 => {
            if numero2 != 0.0 {
                resultado = numero1 / numero2;
            } else {
                println!("No se puede division por cero");
            }
        }
        _ => println!("Tu opcion no es valida"),
    }

    println!("El resultado de tu operacion es: {resultado}");
}
