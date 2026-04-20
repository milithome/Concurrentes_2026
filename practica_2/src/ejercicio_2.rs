/*
Ejercicio 2: Múltiples Workers y Recolección de Handles

Objetivo: Escalar el modelo a múltiples hilos de ejecución.

Enunciado: Vamos a simular el procesamiento de 5 lotes de datos.
Escribí un bucle for que itere del 1 al 5. En cada iteración,
hacé el spawn de un hilo que duplique el número de la iteración y lo retorne.
Guardá todos los JoinHandle en un Vec. Finalmente, recorré ese vector,
hacé el join de cada hilo y sumá todos los resultados parciales para
obtener un total. Imprimí el resultado final.
 */
use std::thread;
pub fn multiples_hilos(){
    let mut hilitos : Vec<thread::JoinHandle<i32>> = vec![];
    for i in 1..6{
        let hilo_duplicate = thread::spawn(move || {
            let num = i*2;
            num
        });
        hilitos.push(hilo_duplicate);
    };
    let mut total = 0;
    for h in hilitos {
        total += h.join().unwrap();
    }
    println!("El total es -> {}", total);
}