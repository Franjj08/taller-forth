mod modules;
use crate::modules::archivo;
#[cfg(test)]
mod tests;

fn main() {
    let ruta :Vec<String> = archivo::leer_ruta();
    println!("Ruta obtenida: {:?}", ruta);

    let contenido = archivo::leer_archivo(ruta);
    println!("Contenido obtenido: {:?}", contenido);

    let contenido_tokenizado = archivo::tokenizar(contenido);
    println!("{:?}", contenido_tokenizado);

}
