//! Example usage of idenumbra.

use idenumbra::Idenumbra;

fn main() {
    for (country, value) in [("ARG", "12345678"), ("MEX", "BOXW310820HNERXN09")] {
        match Idenumbra::infer(country, value) {
            Ok(()) => println!("{country}: valid"),
            Err(e) => println!("{country}: {e}"),
        }
    }
    println!("available: {:?}", Idenumbra::supported_countries());
}
