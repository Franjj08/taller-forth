#[cfg(test)]
use crate::modules::file;
use crate::modules::stack;

fn set_up_route(case: &str) -> Vec<String>{
    let route = match case {
        "valid" => "pruebas/prueba.fth",
        "no_exist" => "prueba/no_exist.fth",
        _ =>"Option no exist"
    };
    vec!["target\\debug\\Taller-Forth.exe".to_string(), route.to_string()]
}

fn set_up_stack(case: &str) ->stack::Stack {
    let input = match case{
        "valid" =>"1 2 +".to_string(),
        "no_exist" =>" ".to_string(),
        _ =>"Option no exist".to_string()
    };
    let stack = file::tokenize(input);
    stack
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
fn test_tokenize_is_not_empty() {
    let stack = set_up_stack("valid");
    assert!(!stack.is_empty());
}

#[test]
fn test_tokenize_correct() {
    let mut stack = set_up_stack("valid");
    assert_eq!(stack.pop_str(), Ok("+".to_string()));
    assert_eq!(stack.pop_int(), Ok(2));
    assert_eq!(stack.pop_int(), Ok(1));
    assert!(stack.is_empty());
}
#[test]
fn test_stack_pop_int_error() {
    let mut stack = set_up_stack("no_exist");
    assert_eq!(stack.pop_str(), Err("stack-underflow"));
}
#[test]
fn test_stack_pop_str_error() {
    let mut stack = set_up_stack("no_exist");
    assert_eq!(stack.pop_str(), Err("stack-underflow"));
}
