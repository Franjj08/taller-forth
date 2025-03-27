#[cfg(test)]
use crate::modules::archivo;

#[test]
fn test_leer_ruta_basica() {
    // Test básico que verifica que recibe al menos el nombre del programa
    let args = archivo::leer_ruta();
    assert!(!args.is_empty());
}