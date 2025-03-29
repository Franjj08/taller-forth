///En stack se puede guardar i16 y String
pub enum StackElement {
    Integer(i16),
    Text(String),
}
///Uso Vec<> por size no esta definido y Vec<> contiene los mayores metodos de stack
pub struct Stack {
    pub elements: Vec<StackElement>
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // Añade un entero a la pila
    pub fn push_int(&mut self, value: i16) {
        self.elements.push(StackElement::Integer(value));
    }


    // Añade un String a la pila
    pub fn push_str(&mut self, value: String) {
        self.elements.push(StackElement::Text(value));
    }

    // Extrae el último elemento (devuelve Option<StackElement>)
    pub fn pop(&mut self) -> Option<StackElement> {
        self.elements.pop()
    }

    // Verifica si la pila está vacía
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Extrae un `i16` de la pila con manejo de errores.
    pub fn pop_int(&mut self) -> Result<i16, &'static str> {
        match self.elements.pop() {
            Some(StackElement::Integer(value)) => Ok(value),
            Some(StackElement::Text(_)) => Err("invalid-Integer"),
            None =>{
                println!("stack-underflow");
                Err("stack-underflow")
            },
        }
    }

    /// Extrae un `String` de la pila con manejo de errores.
    pub fn pop_str(&mut self) -> Result<String, &'static str> {
        match self.elements.pop() {
            Some(StackElement::Text(value)) => Ok(value),
            Some(StackElement::Integer(_)) => Err("invalid-Text"),
            None =>{
                println!("stack-underflow");
                Err("stack-underflow")
            },
        }
    }

    pub fn print_elements(&self) {
        println!("Contenido del stack:");
        for element in &self.elements {
            match element {
                StackElement::Integer(n) => println!("Entero: {}", n),
                StackElement::Text(s) => println!("Texto: \"{}\"", s),
            }
        }
    }
}

