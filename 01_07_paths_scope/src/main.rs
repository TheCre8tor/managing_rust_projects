/* Explanation:
 * To avoid having to write out a full path
 * every time we need to call a casual greeting
 * function, we can bring the formal submodule
 * into scope for this [source file] with a [use]
 * statement */

use greeting::casual;

fn main() {
    /* Explanation:
     * We use a relative path to access the english
     * function from within the formal submodule of
     * the greeting module.
     * */

    // Long path ->
    greeting::casual::english();

    // Short path ->
    casual::english();
    casual::spanish();
}

mod greeting {
    pub mod formal {
        pub fn _english() {
            println!("hello");
        }

        pub fn _spanish() {
            println!("hola");
        }
    }

    pub mod casual {
        pub fn english() {
            println!("hey");
        }

        pub fn spanish() {
            println!("oye");
        }
    }
}
