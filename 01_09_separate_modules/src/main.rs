mod goodbye;
mod greeting;

fn main() {
    // function from greeting.rs
    greeting::description(); // greeting messages

    // function from inline submodule within greeting.rs
    greeting::formal::english(); // hello
    greeting::formal::spanish(); // hola

    // functions from seperate submodule greeting/casual.rs
    greeting::casual::english(); // hey
    greeting::casual::spanish(); // oye

    // function from goodbye/mod.rs
    goodbye::description(); // goodbye messages

    // functions from inline submodule within goodbye/mod.rs
    goodbye::formal::english(); // goodbye
    goodbye::formal::spanish(); // adios

    // functions from separate submodule greeting/casual.rs
    goodbye::casual::english(); // see you later
    goodbye::casual::spanish(); // hasta luego
}
