mod Spell_Corrector;
fn main() {
    println!("Hello, world!");


    let mut dictionary = Trie::Trie::new();

    dictionary.add("aba".to_string());
    dictionary.add("aca".to_string());

    println!("{}",dictionary.to_string());
}
