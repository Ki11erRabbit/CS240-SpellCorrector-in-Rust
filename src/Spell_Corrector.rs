mod Trie;


use std::fs;

use std::collections::HashSet;

pub struct Spell_Corrector {
    dictionary: Trie::Trie,
    edit_dist1: HashSet<String>,
    edit_dist2: HashSet<String>,
}

impl Spell_Corrector {

    pub fn new() -> Self {
        Self {dictionary: Trie::Trie::new(), edit_dist1: HashSet::new(), edit_dist2: HashSet::new() }
    }
    pub fn use_dictionary(&mut self, dictionary_file_name: String) {
        let file = fs::read_to_string(dictionary_file_name);
        
        for line in file.expect("Unable to find dictionary").lines() {
            
            for word in line.split(' ') {
                word = &word.to_lowercase();
                word = word.trim_end();
                self.dictionary.add(word.to_string());
            }
            
        }
    }

    pub fn suggest_similar_word(&mut self, input_word: String) -> Result<String, String> {
        input_word = input_word.to_lowercase();

        match self.dictionary.find(input_word) {
            Err(x) => {},
            Ok(v) => return Ok(input_word)
        }

        self.gen_edit_dist1(input_word);
        let mut matches = Vec::new();
        for word in self.edit_dist1 {
            match self.dictionary.find(word) {
                Err(x) => {},
                Ok(v) => matches.push(word)
            }
        }

        let mut output : (Option<String>, u32) = (None, 0);

        for matched_word in matches {
            let mut pair;
            match self.dictionary.find(matched_word) {
                Err(x) => continue,
                Ok(node) => pair = (Some(matched_word), node.get_freq())
            }

            if output.0 < pair.0 {
                output = pair;
            }
            
        }
        
        if output.0.is_some() {
            return Ok(output.0.unwrap());
        }

        matches.clear();
        for word in self.edit_dist1 {
            self.gen_edit_dist2(word);
        }


        for matched_word in matches {
            let mut pair;
            match self.dictionary.find(matched_word) {
                Err(x) => continue,
                Ok(node) => pair = (Some(matched_word), node.get_freq())
            }

            if output.0 < pair.0 {
                output = pair;
            }
            
        }
        
        if output.0.is_some() {
            return Ok(output.0.unwrap());
        }
        
        Err("Unable to find word \"".to_string() + &input_word + &"\"".to_string())
    }

    fn delete_char(&mut self,word: String) -> Vec<String> {
        let mut words = Vec::new();
        for i in 0..(word.chars().count()-1) {
            let mut new_word = word;
            new_word.drain(i..i+1);
            
            words.push(new_word);
        }
        words
    }

    fn transpose_char(&mut self,word: String) -> Vec<String> {
        let mut words = Vec::new();
        for i in 0..(word.chars().count()-1) {
            let char1 = word.chars().nth(i).unwrap();
            for j in 1..(word.chars().count() -1) {
                let char2 = word.chars().nth(j).unwrap();
                let mut new_word = word;

                new_word.replace_range(i..i+1, &char2.to_string());
                new_word.replace_range(j..j+1, &char1.to_string());

                words.push(new_word);
            }
        }
        words
    }

    fn alternate_char(&mut self,word: String) -> Vec<String> {
        let mut words = Vec::new();
        for i in 0..(word.chars().count()-1) {
            for c in 'a'..'z' {
                let mut new_word = word;
                
                new_word.replace_range(i..i+1, &c.to_string());

                words.push(new_word);
            }
        }
        words
    }

    fn insert_char(&mut self,word: String) -> Vec<String> {
        let mut words = Vec::new();
        for i in 0..(word.chars().count()) {
            for c in 'a'..'z' {
                let mut new_word = word;

                new_word.replace_range(i..i+1, &c.to_string());

                words.push(new_word);
            }
        }

        words
    }

    fn gen_edit_dist1(&mut self, word: String) {
        for new_word in self.delete_char(word) {
            self.edit_dist1.insert(new_word);
        }
        for new_word in self.transpose_char(word) {
            self.edit_dist1.insert(new_word);
        }
        for new_word in self.alternate_char(word) {
            self.edit_dist1.insert(new_word);
        }
        for new_word in self.insert_char(word) {
            self.edit_dist1.insert(new_word);
        }
    }

    fn gen_edit_dist2(&mut self, word: String) {

        for new_word in self.delete_char(word) {
            self.edit_dist2.insert(new_word);
        }
        for new_word in self.transpose_char(word) {
            self.edit_dist2.insert(new_word);
        }
        for new_word in self.alternate_char(word) {
            self.edit_dist2.insert(new_word);
        }
        for new_word in self.insert_char(word) {
            self.edit_dist2.insert(new_word);
        }
    }
    
}
