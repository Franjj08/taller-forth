///En stack se puede guardar i16 y String
pub enum StackElemento {
    Integer(i16),
    Text(String),
}
///Uso Vec<> por size no esta definido y Vec<> contiene los mayores metodos de stack
pub struct Stack {
    elements: Vec<StackElemento>
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // Añade un entero a la pila
    pub fn push_int(&mut self, value: i16) {
        self.elements.push(StackElemento::Integer(value));
    }

    // Añade un String a la pila
    pub fn push_str(&mut self, value: String) {
        self.elements.push(StackElemento::Text(value));
    }

    // Extrae el último elemento (devuelve Option<StackElement>)
    pub fn pop(&mut self) -> Option<StackElemento> {
        self.elements.pop()
    }

    // Verifica si la pila está vacía
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn print_elements(&self) {
        println!("Contenido del stack:");
        for element in &self.elements {
            match element {
                StackElemento::Integer(n) => println!("Entero: {}", n),
                StackElemento::Text(s) => println!("Texto: \"{}\"", s),
            }
        }
    }
}

