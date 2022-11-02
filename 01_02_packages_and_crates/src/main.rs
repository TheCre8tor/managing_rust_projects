/* Rust Module System Elements ->
*  1. Modules
*  2. Paths
*  3. Crates -> [Library / Binary]
*  4. Packages -> [Whole project]

? Package ->
? A package contains one library and many binary crates
? that provide a set of functionality.

? we can create a new cargo package using the command:
? cargo new {package_name}

! main.rs file -> executable binary crate
! lib.rs -> library crate

? We put all reusable code in a library crate which get
? built, and then the binary crate uses it.

! To build the program all at ones use: cargo build --workspace
* */

mod some_module;

use packages_and_crates; // library crate for [managing_project] package

fn main() {
    println!("Running the managing_project executable.");
    some_module::mod_func();
    packages_and_crates::lib_func();
}
