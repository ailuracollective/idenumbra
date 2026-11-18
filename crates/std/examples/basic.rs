//! Example usage of idenumbra.

use idenumbra::Idenumbra;

fn main() {
    for (country, value) in [("ARG", "12345678"), ("MEX", "GALA850315HNERXN05")] {
        match Idenumbra::infer(country, value) {
            Ok(()) => println!("{country}: valid"),
            Err(e) => println!("{country}: {e}"),
        }
    }
    println!("available: {:?}", Idenumbra::supported_countries());
}
