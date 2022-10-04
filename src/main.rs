mod Spell_Corrector;

use std::env;

fn main() {


    /*let mut dictionary = Trie::Trie::new();

    dictionary.add("aba".to_string());
    dictionary.add("aca".to_string());

    println!("{}",dictionary.to_string());*/
    
    let args: Vec<String> = env::args().collect();
    let dictionary_name = &args[1];
    let input_word = &args[2];
    let mut corrector = Spell_Corrector::SpellCorrector::new();

    corrector.use_dictionary(dictionary_name.to_string());
    let suggestion = corrector.suggest_similar_word(input_word.to_string());
    match suggestion {
        Err(_x) => println!("No similar word found"),
        Ok(word) => println!("Suggestion is: {}",word)
    }

}
