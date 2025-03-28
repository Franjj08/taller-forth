use std::{env, fs};
///Leer input de la terminal y convierte en un vector de string
pub fn leer_route() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

///Leer contenido del archivo y convierte a string
pub fn leer_file(args:Vec<String>) -> String {
    if args.len() < 2 {
        print!("no hay argumento suficiente")
    }

    let file = &args[1];

    match fs::read_to_string(file) {
        Ok(content) =>content,
        Err(_) =>"Error al leer el archivo".to_string(),
    }
}

///Convierte el contenido a un stack 
pub fn tokenize(input:String) ->Vec<String> {
    input.split_whitespace()  
        .map(|s| s.to_string())  
        .collect()
}