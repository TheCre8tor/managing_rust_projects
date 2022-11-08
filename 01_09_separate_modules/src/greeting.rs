// module ->
pub fn description() {
    println!("greeting messages");
}

// inline sub-module ->
pub mod formal {
    pub fn english() {
        println!("hello");
    }

    pub fn spanish() {
        println!("hola");
    }
}

// sub-module in greeting/ directory ->
pub mod casual;
