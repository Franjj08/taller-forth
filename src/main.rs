mod modules;

use crate::modules::file;
#[cfg(test)]
mod tests;
use crate::modules::stack;
fn main() {
    let ruta :Vec<String> = file::leer_route();
    println!("Ruta obtenida: {:?}", ruta);

    let contenido = file::leer_file(ruta);
    println!("Contenido obtenido: {:?}", contenido);

    let stack = file::tokenize(contenido);
    stack.print_elements();


}
