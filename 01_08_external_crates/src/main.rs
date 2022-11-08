/* [as] KEYWORD provides a new name when bringing items into scope */
use rand::{thread_rng as range, Rng};

fn main() {
    let mut rng = range();
    let num: u32 = rng.gen();

    println!("The random number is: {}", num);
}
