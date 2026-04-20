/*
Ejercicio 2: Modelando el Dominio

Objetivo: Uso de Structs y tuplas.

Enunciado: Definí una estructura Punto que represente una coordenada en 2D utilizando una tupla posicional
 de dos f64. Luego, definí una estructura Usuario con campos con nombre: nombre (String), edad (u8)
 y ubicacion (el struct Punto que creaste antes). Instanciá un usuario en main e imprimí su nombre.
 */

use std::io;
pub fn crear_usuario(){
    let mut nombre_input = String::new(); 

    println!("Ingrese el nombre del usuario:");

    // 2. Leemos la línea. 
    // .expect() es necesario porque leer de la terminal puede fallar
    io::stdin()
        .read_line(&mut nombre_input)
        .expect("Error al leer el nombre");

    // 3. Limpiamos el salto de línea (\n) que queda al presionar Enter
    let nombre = nombre_input.trim().to_string();

    // 4. Para números, leemos texto y luego lo "parseamos"
    println!("Ingrese la edad:");
    let mut edad_input = String::new();
    io::stdin()
        .read_line(&mut edad_input)
        .expect("Error al leer la edad");

    // Convertimos de String a u8
    let edad: u8 = edad_input.trim().parse().expect("Por favor, ingrese un número válido");

    let mut coordenada_x_input = String::new();
    println!("Ingrese la coordenada X del usuario:");
    io::stdin()
        .read_line(&mut coordenada_x_input)
        .expect("Error al leer la coordenadas");
    let x: f64 = coordenada_x_input.trim().parse().expect("X debe ser un número");

    let mut coordenada_y_input = String::new();
    println!("Ingrese la coordenada Y del usuario:");
    io::stdin()
        .read_line(&mut coordenada_y_input)
        .expect("Error al leer la coordenadas");
    let y: f64 = coordenada_y_input.trim().parse().expect("Y debe ser un número");

    // Instanciamos con los datos del usuario
    let usuario_nuevo = Usuario {
        nombre: nombre,
        edad: edad,
        ubicacion: Punto(x, y), // Valor por defecto para simplificar
    };

    println!("Usuario creado: {} de {} años se encuentra en {},{}.", usuario_nuevo.nombre, usuario_nuevo.edad, usuario_nuevo.ubicacion.0.to_string(), usuario_nuevo.ubicacion.1.to_string());
}
#[derive(Debug)]

struct Punto(f64,f64);

struct Usuario {
    nombre: String,
    edad: u8,
    ubicacion: Punto
}