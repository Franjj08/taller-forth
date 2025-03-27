use std::env;

pub fn leer_ruta() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}
// pub fn leer_archivo(){

// }
