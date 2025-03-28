#[cfg(test)]
use crate::modules::file;

fn set_up_route(case: &str) -> Vec<String>{
    let route = match case {
        "valid" => "pruebas/prueba.fth",
        "no_exist" => "prueba/no_exist.fth",
        _ =>"Option no exist"
    };
    vec!["target\\debug\\Taller-Forth.exe".to_string(), route.to_string()]
}
#[test]
fn test_read_route_basic() {
    let args = file::leer_route();
    assert!(!args.is_empty());
}

#[test]
fn test_read_file_valid() {
    let route = set_up_route("valid");
    let content = file::leer_file(route);
    assert_eq!(content,  "1 2 3 +".to_string());
}

#[test]
fn test_read_file_no_exist() {
    let route = set_up_route("no_exist");
    let content = file::leer_file(route);
    assert_eq!(content,  "Error al leer el archivo".to_string());
}

#[test]
fn test_tokenize_content() {
    let input = "1 2 +".to_string();
    let resultado = file::tokenize(input);
    assert_eq!(resultado,vec!["1", "2", "+"]);
}
