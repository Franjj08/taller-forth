use std::{env, fs};
///Leer input de la terminal y convierte en un vector de string
pub fn leer_ruta() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

///Leer contenido del archivo y convierte a string
pub fn leer_archivo(args:Vec<String>) -> String {
    if args.len() < 2 {
        print!("no hay argumento suficiente")
    }

    let archivo = &args[1];

    match fs::read_to_string(archivo) {
        Ok(contenido) =>contenido,
        Err(_) =>"Error al leer el archivo".to_string(),
    }
}

///Convierte el contenido a un vector de String
pub fn tokenizar(input:String) ->Vec<String> {
    input.split_whitespace()  
        .map(|s| s.to_string())  
        .collect()
}