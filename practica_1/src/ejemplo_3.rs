/*
Ejercicio 3: Jugando con Arrays y Slices

Objetivo: Manipulación de colecciones estáticas.

Enunciado: Creá un array de 5 números enteros (i32). Luego, creá un slice
que contenga solo el segundo, tercer y cuarto elemento del array.
Imprimí el slice.
 */

pub fn arranashe(){
    let v : Vec<i32> = vec![1,2,3,4,5];

    let slice = &v[1..5];
    print!("{:?}",slice);
}