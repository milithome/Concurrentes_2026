/*
Ejercicio 5: Tu primer Thread (Concurrencia)

Objetivo: Empezar con la "concurrencia sin miedo".
Enunciado: Rust utiliza la librería estándar para manejar hilos.
Usando std::thread, creá un hilo nuevo (spawn) que imprima "Hola desde el hilo
secundario" 5 veces con una pequeña pausa, mientras el hilo principal (main)
imprime "Hola desde el principal" otras 5 veces.
Tip: No te olvides de usar .join().unwrap() al final para esperar a que el hilo
termine.
 */
use std::thread;
use std::time::Duration;
pub fn concus(){
    let handle = std::thread::spawn(||{
        for i in 1..6{
            println!("Hola desde el hilo secundario: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..6{
        println!("Hola desde el principal: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}