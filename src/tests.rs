#[cfg(test)]
use crate::modules::archivo;

fn set_up_ruta(caso: &str) -> Vec<String>{
    let ruta = match caso {
        "valida" => "pruebas/prueba.fth",
        "no_existe" => "prueba/no_existe.fth",
        _ =>"Opcion no exite"
    };
    vec!["target\\debug\\Taller-Forth.exe".to_string(), ruta.to_string()]
}
#[test]
fn test_leer_ruta_basica() {
    let args = archivo::leer_ruta();
    assert!(!args.is_empty());
}

#[test]
fn test_leer_archivo_valido() {
    let ruta = set_up_ruta("valida");
    let contenido = archivo::leer_archivo(ruta);
    assert_eq!(contenido,  "1 2 3 +".to_string());
}

#[test]
fn test_leer_archivo_inexistente() {
    let ruta = set_up_ruta("no_existe");
    let contenido = archivo::leer_archivo(ruta);
    assert_eq!(contenido,  "Error al leer el archivo".to_string());
}

#[test]
fn test_tokenizar_string() {
    let input = "1 2 +".to_string();
    let resultado = archivo::tokenizar(input);
    assert_eq!(resultado,vec!["1", "2", "+"]);
}
