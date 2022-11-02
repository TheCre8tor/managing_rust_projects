fn main() {
    // Absolute path ->
    crate::greetings::english();
    crate::greetings::casual::english();

    // Relative path ->
    greetings::english();
}

// Inline Module ->
/* Anything we put inside the greetings module
 * will be contained within the scope of this
 * module. */

mod greetings {
    pub fn english() {
        println!("hello");

        // Relative path ->
        spanish();
        casual::english();
    }

    fn spanish() {
        println!("hola");
    }

    // Nexted module ->
    pub mod casual {
        pub fn english() {
            println!("hey");

            // Absolute path ->
            crate::greetings::spanish();

            // Relative path ->
            /* super tells Rust to build the relative path
            starting with the parent of the current module. */
            super::spanish();
        }
    }
}
