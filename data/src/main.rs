use serde::{Serialize, Deserialize};
// bincode es un formato binario muy eficiente para Rust
use bincode; 
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
// Definimos una estructura de datos para el ejemplo
struct Usuario {
    id: u32,
    nombre: String,
    activo: bool,
}


fn main() {
    let usuario = Usuario {
        id: 1,
        nombre: String::from("Alex"),
        activo: true,
    };

    // 1. Serializar a Binario (Bincode)
    let binario: Vec<u8> = bincode::serialize(&usuario).unwrap();
    fs::write("usuario.dat", &binario).expect("No se pudo escribir");

    // 2. Deserializar desde Binario
    let datos_leidos = fs::read("usuario.dat").expect("No se pudo leer");
    let usuario_recuperado: Usuario = bincode::deserialize(&datos_leidos).unwrap();

    println!("Estructura recuperada: {:?}", usuario_recuperado);
}