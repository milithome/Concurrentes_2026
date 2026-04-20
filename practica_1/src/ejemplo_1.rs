/*
Ejercicio 1: El Convertidor de Temperatura

Objetivo: Practicar tipos de datos primitivos , mutabilidad y funciones.

Enunciado: Escribí una función llamada convertir_a_celsius que reciba una
temperatura en grados Fahrenheit (como f64) y devuelva su equivalente en
Celsius (también f64). En la función main, declará una variable mutable
con un valor inicial de 100.0, cambiala a 32.0, llamá a la función
e imprimí el resultado usando println!.
 */

pub fn ejemplo_1(){
    let mut temperatura = 100.0;
    temperatura = 32.0;
    let temperatura_c = convertir_a_celsius(temperatura);
    println!("Temperatura: {}", temperatura_c)
}

fn convertir_a_celsius(c: f64) -> f64 {
    let en_celsius = (c - 32.0)/1.8;
    en_celsius
}