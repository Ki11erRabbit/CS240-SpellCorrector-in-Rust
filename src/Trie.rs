pub struct Node {
    data: char,
    freq: u32,
    children: Vec<Option<Box<Node>>>,
}

impl Node {
    pub fn new(data: char) -> Self {
        Self {data: data, freq: 0, children: vec![None;26]}
    }
    pub fn get_value(&self) -> char {
        self.data
    }

    pub fn get_freq(&self) -> u32 {
        self.freq
    }
    pub fn get_children(&mut self) -> &mut Vec<Option<Box<Node>>> {
        &mut self.children
    }
    pub fn increment_freq(&mut self) {
        self.freq += 1;
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {data: '\0', freq: 0, children: vec![None;26]}
    }
}
impl Clone for Node {
    fn clone(&self) -> Self {
        Self {data: self.data, freq: self.freq, children: self.children.to_owned() }
    }
}

pub struct Trie {
    root: Box<Node>,
    num_nodes: u32,
    num_words: u32
}

impl Default for Trie {
    fn default() -> Self {
        Self {root: Default::default(), num_nodes: 1, num_words: 0}
    }
}
impl Trie {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add(&mut self, word: String) {
        let lower_word = word.to_lowercase();
        let mut curr_node = &mut self.root;// self.root;
       
        let mut i = 0;
        for letter in lower_word.as_str().chars() {
            let index :usize = letter.to_digit(36).unwrap() as usize - 10;

            if curr_node.get_children()[index].is_some() {
                curr_node = curr_node.get_children()[index].as_mut().unwrap();
            }
            else {
                    curr_node.get_children()[index] = Some(Box::new(Node::new(letter)));
                    curr_node = curr_node.get_children()[index].as_mut().unwrap(); 
                    self.num_nodes += 1;
            }
            if curr_node.get_freq() < 1 && i == lower_word.len() -1 {
                self.num_words += 1;
                curr_node.increment_freq();
            }
            else if curr_node.get_freq() > 0 && i == lower_word.len() -1 {
                curr_node.increment_freq();
            }
            i += 1;
        }
    }

    pub fn find(&self, word: String) -> Result<Node,String> {
        let lower_word = word.as_str().to_lowercase();
        let mut curr_node = self.root.clone();

        for letter in lower_word.chars() {
            let index :usize = letter.to_digit(36).unwrap() as usize - 10;
            match &curr_node.get_children()[index] {
                Some(next_node) => {
                    *curr_node = *next_node.clone(); 
                }
                None => {
                    return Err(word + &" not found".to_string());
                }
            }
        }

        if curr_node.get_freq() >= 1 {
            return Ok(*curr_node);
        }

        return Err(word + &" not found".to_string());
    }

    pub fn get_word_count(&self) -> u32 {
        self.num_words
    }
    pub fn get_node_count(&self) -> u32 {
        self.num_nodes
    }
    fn to_string_helper(mut curr_node: Node, holder: &mut String, out: &mut String) {
        for i in 0..25 {
            
            match &curr_node.get_children()[i] {
                Some(next_node) => {
                    holder.push(next_node.get_value());
                    if next_node.get_freq() > 0 {
                        let output = holder.as_str().to_owned() + "\n";
                        *out += output.as_str();
                        //print!("{}",out);
                    }
                    Trie::to_string_helper(*next_node.clone(), holder, out);
                    holder.pop();
                }
                None => {
                    continue;
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut holder = String::new();
        
        Self::to_string_helper(*self.root.clone(),&mut holder,&mut out);
        out.pop(); 

        out
    }
    
    pub fn hash_code(&mut self) -> i32 {
        let mut sum :i32 = 0;
        for i in 0..25 {
            if self.root.get_children()[i].is_some() {
                sum += i as i32;
            }
        }

        sum | self.num_nodes as i32 & self.num_words as i32
    }
    fn compare(mut base_node: Node, mut test_node: Node) -> bool {
        if base_node.get_freq() != test_node.get_freq() {
            return false;
        }

        for i in 0..25 {
            if base_node.get_children()[i].is_some() && test_node.get_children()[i].is_some() {

                if base_node.get_freq() != test_node.get_freq() {
                    return false;
                }
                else if !Trie::compare(*base_node.get_children()[i].as_ref().unwrap().clone(), *test_node.get_children()[i].as_ref().unwrap().clone()) {
                    return false;
                }
            }
        }
        true
    }
}
impl Eq for Trie {}

impl PartialEq for Trie {
    fn eq(&self, other: &Self) -> bool {
        if self.num_words != other.num_words {
            return false;
        }
        if self.num_nodes != other.num_nodes {
            return false;
        }

        Trie::compare(*self.root.clone(), *other.root.clone())
    }
}
