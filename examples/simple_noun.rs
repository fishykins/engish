use engish::builders::{PropperNounBuilder, WordBuilder};
use rand::rng;

/// This example demonstrates the basic usage of the `PropperNounBuilder`
/// to generate random, realistic-sounding proper nouns.
fn main() {
    let mut rng = rng();
    // Use the default builder which loads the default English language model.
    let builder = PropperNounBuilder::default();

    println!("Generating 10 random proper nouns:");

    for i in 0..10 {
        let noun = builder.build(&mut rng);
        println!("{}: {}", i + 1, noun);
    }
}
