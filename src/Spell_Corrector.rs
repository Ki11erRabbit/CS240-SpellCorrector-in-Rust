mod Trie;


use std::fs;

use std::collections::HashSet;

pub struct SpellCorrector {
    dictionary: Trie::Trie,
}

impl SpellCorrector {

    pub fn new() -> Self {
        Self {dictionary: Trie::Trie::new()}
    }
    pub fn use_dictionary(&mut self, dictionary_file_name: String) {
        let file = fs::read_to_string(dictionary_file_name);
        
        for line in file.expect("Unable to find dictionary").lines() {
            
            for word in line.split(' ') {
                let mut lower_word = word.to_lowercase();
                lower_word = lower_word.trim_end().to_string();
                self.dictionary.add(lower_word.to_string());
            }
            
        }
    }

    pub fn suggest_similar_word(&mut self, input_word: String) -> Result<String, String> {
        let lower_word = input_word.to_lowercase();

        match self.dictionary.find(&lower_word) {
            Err(_x) => {},
            Ok(_v) => return Ok(input_word)
        }

        let edit_dist1 = self.gen_edit_dist1(lower_word);
        //println!("{:?}", self.edit_dist1);
        println!("edit dist 1 size: {}",edit_dist1.len());
        println!("Finding Matches in Edit Distance 1");
        let mut matches = Vec::new();
        for word in edit_dist1.iter() {
            match self.dictionary.find(&word.to_string()) {
                Err(_x) => {},
                Ok(_v) => matches.push(word)
            }
        }

        let mut output : (Option<String>, u32) = (None, 0);
        println!("finding Highest Freq in matches");
        for matched_word in matches.iter() {
            let pair;
            match self.dictionary.find(&matched_word.to_string()) {
                Err(_x) => continue,
                Ok(node) => pair = (Some(matched_word.to_string().clone()), node.get_freq())
            }

            if output.0 < pair.0 {
                output = pair;
            }
            
        }
        //println!("{:?}", matches); 
        if output.0.is_some() {
            return Ok(output.0.unwrap().to_string());
        }

        let mut matches2 = Vec::new();
        
        let edit_dist2 = self.gen_edit_dist2(&edit_dist1);
        println!("edit dist 2 size: {}",edit_dist2.len());
        println!("Finding Matches in Edit Distance 2");
        for word in edit_dist2.iter() {
            //println!("{}",word);
            match self.dictionary.find(&word.to_string()) {
                Err(_x) => {},
                Ok(_v) => matches2.push(word.to_string())
            }
        }
        
        //println!("{:?}", self.edit_dist2);
        println!("finding Highest Freq in matches");
        
        for matched_word in matches2.iter() {
            let pair;
            match self.dictionary.find(&matched_word.to_string()) {
                Err(_x) => continue,
                Ok(node) => pair = (Some(matched_word.to_string().clone()), node.get_freq())
            }

            if output.0 < pair.0 {
                output = pair;
            }
            
        }
        
        //println!("{:?}", matches); 
        if output.0.is_some() {
            return Ok(output.0.unwrap().to_string());
        }
        
        Err("Unable to find word \"".to_string() + &input_word + &"\"".to_string())
    }

    fn delete_char(&mut self,word: & String) -> Vec<String> {
        let mut words = Vec::new();
        //println!("length of word {}",word.chars().count());
        for i in 0..(word.chars().count()-1) {
            let mut new_word = word.clone();
            new_word.drain(i..i+1);
            
            words.push(new_word);
        }
        words
    }

    fn transpose_char(&mut self,word: &String) -> Vec<String> {
        let mut words = Vec::new();
        for i in 0..(word.chars().count()-1) {
            let char1 = word.chars().nth(i).unwrap();
            for j in 1..(word.chars().count() -1) {
                let char2 = word.chars().nth(j).unwrap();
                if char1 == char2 {
                    continue;
                }
                //println!("{} {}",char1.to_string(), char2.to_string());
                let mut new_word = word.clone();

                new_word.replace_range(i..i+1, &char2.to_string());
                new_word.replace_range(j..j+1, &char1.to_string());
                //println!("\t{} {}",word,new_word);
                words.push(new_word);
            }
        }
        words
    }

    fn alternate_char(&mut self,word: &String) -> Vec<String> {
        let mut words = Vec::new();
        for i in 0..(word.chars().count()-1) {
            for c in 'a'..'z' {
                let mut new_word = word.clone();
                
                new_word.replace_range(i..i+1, &c.to_string());

                words.push(new_word);
            }
        }
        words
    }

    fn insert_char(&mut self,word: &String) -> Vec<String> {
        let mut words = Vec::new();
        for i in 0..(word.chars().count()) {
            for c in 'a'..'z' {
                let mut new_word = word.clone();

                new_word.replace_range(i..i+1, &c.to_string());

                words.push(new_word);
            }
        }

        words
    }

    fn gen_edit_dist1(&mut self, word: String) -> HashSet<String> {
        let mut edit_dist1 = HashSet::new();
        edit_dist1.extend(self.delete_char(&word));
        edit_dist1.extend(self.transpose_char(&word));
        edit_dist1.extend(self.alternate_char(&word));
        edit_dist1.extend(self.insert_char(&word));


/*
        for new_word in self.delete_char(&word) {
            edit_dist1.insert(new_word);
        }
        for new_word in self.transpose_char(&word) {
            edit_dist1.insert(new_word);
        }
        for new_word in self.alternate_char(&word) {
            edit_dist1.insert(new_word);
        }
        for new_word in self.insert_char(&word) {
            edit_dist1.insert(new_word);
        }*/
        edit_dist1
    }

    fn gen_edit_dist2(&mut self, words: &HashSet<String>) -> HashSet<String> {

        let mut edit_dist2 = HashSet::new();
        for word in words.iter() {
            edit_dist2.extend(self.delete_char(&word));
            edit_dist2.extend(self.transpose_char(&word));
            edit_dist2.extend(self.alternate_char(&word));
            edit_dist2.extend(self.insert_char(&word));
        }
        /*for word in words.iter() {
            for new_word in self.delete_char(&word) {
                edit_dist2.insert(new_word);
            }
            for new_word in self.transpose_char(&word) {
                edit_dist2.insert(new_word);
            }
            for new_word in self.alternate_char(&word) {
                edit_dist2.insert(new_word);
            }
            for new_word in self.insert_char(&word) {
                edit_dist2.insert(new_word);
            }
        }*/
        edit_dist2
    }
    
}
