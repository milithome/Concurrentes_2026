/*
Ejercicio 1: Fork-Join Básico con Retorno

Objetivo: Crear un hilo, pasarle un dato, hacer un cálculo y recuperar
el resultado.

Enunciado: Creá una variable base con el valor 10. Hacé un "fork" creando
un hilo que capture esa variable, calcule su cuadrado y lo retorne.
En el hilo principal, hacé el "join" para capturar ese valor retornado
e imprimilo por consola.

 */
use std::thread;
pub fn basic_f_j(){
    let num : i32 = 10;
    //// FORK: Creamos el hilo. Usamos 'move' para que el hilo tome posesión de 'base'
    let worker_handle = thread::spawn(move || {
        println!("Worker: Calculando el cuadrado de {}...", num);
        //calculo el cuadrado
        num*num
    });
    //hago el join() para que se termine de ejcutar correctamente el hilo
    //y me de el resultado
    println!("Cuadrado del numero -> {} es = {}", num, worker_handle.join().unwrap());
}