// module ->
pub fn description() {
    println!("goodbye messages");
}

// inline sub-module ->
pub mod formal {
    pub fn english() {
        println!("goodbye");
    }

    pub fn spanish() {
        println!("adios");
    }
}

// sub-module in greeting/ directory ->
pub mod casual;
