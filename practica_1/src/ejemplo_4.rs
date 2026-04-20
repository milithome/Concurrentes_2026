/*
Ejercicio 4: El Juego del Traspaso (Ownership)

Objetivo: Comprender cómo Rust mueve la "dueñidad" de los datos en memoria.

Enunciado: 1. Creá una función llamada tomar_propiedad que reciba un String y
simplemente lo imprima.
2. En el main, creá un String llamado mi_cadena.
3. Llamá a la función pasando mi_cadena.
4. Intentá imprimir mi_cadena nuevamente en el main después de la llamada a la
función.
Pregunta: ¿Qué dice el compilador?
¿Cómo lo arreglarías usando una referencia (&) para que el main no pierda el
acceso?
 */
//LE PRESTO LA REFERENCIA DE ESCRITURA
fn tomar_propiedad( mut string: &String){
    print!("{:?}\n",string)
}

pub fn ownshp(){
    let mut mi_cadena = String::from("me estan relevando jijo");
    tomar_propiedad( &mi_cadena);
    print!("!{:?}",mi_cadena)
}
//LE PRESTO LA REFERENCIA DE LECTURA
/*
fn tomar_propiedad( string: &String){
    print!("{:?}\n",string)
}

pub fn ownshp(){
    let mi_cadena = String::from("me estan relevando jijo");
    tomar_propiedad( &mi_cadena);
    print!("!{:?}",mi_cadena)
}
 */