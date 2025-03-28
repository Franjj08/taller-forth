mod modules;
use crate::modules::archivo;
#[cfg(test)]
mod tests;

fn main() {
    let ruta :Vec<String> = archivo::leer_ruta();
    println!("Ruta obtenida: {:?}", ruta);

    let contenido = archivo::leer_archivo(ruta);
    println!("Contenido obtenido: {:?}", contenido);

}
