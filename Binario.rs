use std::fs::File;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {
    let datos = "Hola, este es un ejemplo de escritura y lectura en formato binario en Rust. viva linux abajo windowsbugs";
    
    // Escribir
    let mut archivo_salida = File::create("datos.bin")?;
    archivo_salida.write_all(datos.as_bytes())?;
    println!("Datos guardados en binario.");

    // Leer
    let mut archivo_entrada = File::open("datos.bin")?;
    let mut buffer = Vec::new();
    archivo_entrada.read_to_end(&mut buffer)?;
    
    let contenido = String::from_utf8_lossy(&buffer);
    println!("Leído del archivo: {}", contenido);

    Ok(())
}