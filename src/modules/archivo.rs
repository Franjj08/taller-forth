use std::{env, fs};
///Leer input de la terminal y convierte en un vector de string
pub fn leer_ruta() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

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



// fn main() -> Result<(), Box<dyn Error>> {
//     let message: String = fs::read_to_string("message.txt")?;
//     println!("{}", message);
//     Ok(())
// }