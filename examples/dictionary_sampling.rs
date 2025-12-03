use engish::language::{Adjective, Dictionary, Noun, Verb};
use rand::rng;

/// This example demonstrates how to create a `Dictionary`, populate it with
/// different types of words, and then sample random words from it.
fn main() {
    // 1. Create a new dictionary.
    let mut dict = Dictionary::new();
    println!("Created a new, empty dictionary.");

    // 2. Populate it with some words.
    // Add some nouns
    dict.add_word(Noun::new_proper("Gandalf"));
    dict.add_word(Noun::new_common("wizard"));
    dict.add_word(Noun::new_common("staff"));
    dict.add_word(Noun::new_proper("Frodo"));

    // Add some verbs
    dict.add_word(Verb::new_regular("walk"));
    dict.add_word(Verb::new_regular("cast"));

    // Add some adjectives
    dict.add_word(Adjective::new_regular("grey"));
    dict.add_word(Adjective::new_regular("brave"));
    dict.add_word(Adjective::new_regular("small"));

    println!(
        "Dictionary populated with {} nouns, {} verbs, and {} adjectives.\n",
        dict.get_all::<Noun>().len(),
        dict.get_all::<Verb>().len(),
        dict.get_all::<Adjective>().len()
    );

    // Get a random number generator.
    let mut rng = rng();

    // 3. Sample a random word of a specific type.
    println!("--- Simple Random Sampling ---");
    if let Some(noun) = dict.choose::<Noun>(&mut rng) {
        println!("Randomly selected noun: {}", noun);
    }
    if let Some(verb) = dict.choose::<Verb>(&mut rng) {
        println!("Randomly selected verb: {}", verb);
    }
    println!();

    // 4. Sample a random word that matches a specific filter.
    println!("--- Filtered Random Sampling ---");
    let random_proper_noun = dict.choose_filtered::<Noun, _>(|n| n.is_proper(), &mut rng);
    println!("Randomly selected proper noun: {}", random_proper_noun.unwrap());
}