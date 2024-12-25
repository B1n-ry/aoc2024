use rayon::{iter::{IntoParallelRefIterator, ParallelIterator}, str::ParallelString};

struct Dictionary {
    root_nodes: Vec<DictionaryNode>
}
struct DictionaryNode {
    end_node: bool,
    node_value: char,
    children: Vec<DictionaryNode>,
}
impl Dictionary {
    fn new() -> Self {
        Self { root_nodes: Vec::new() }
    }
    fn insert(&mut self, input: &str) {
        match self.root_nodes.iter_mut().find(|node| input.starts_with(node.node_value)) {
            Some(node) => if input.len() == 1 { node.end_node = true; } else { node.insert(&input[1..]); },
            None => self.root_nodes.push(DictionaryNode::new(input)),
        }
    }
    fn contains(&self, pat: &str) -> bool {
        self.root_nodes.iter().any(|node| node.contains(pat))
    }
    fn contains_part_of(&self, pat: &str) -> bool {
        self.root_nodes.iter().any(|node| node.contains_part_of(pat))
    }
}
impl<'a> FromIterator<&'a str> for Dictionary {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut dict = Dictionary::new();

        iter.into_iter().for_each(|slice| dict.insert(slice));
        dict
    }
}
impl DictionaryNode {
    fn new(string: &str) -> Self {
        let c = string.chars().nth(0).expect("Tried creating a dictionary node from empty string!");
        let end_node = string.len() == 1;
        let child = if !end_node { vec![Self::new(&string[1..])] } else { Vec::new() };

        Self { end_node, node_value: c, children: child }
    }
    fn insert(&mut self, string: &str) {
        if string.len() == 0 {
            self.end_node = true;
            return;
        }
        match self.children.iter_mut().find(|node| string.starts_with(node.node_value)) {
            Some(child) => child.insert(&string[1..]),
            None => self.children.push(DictionaryNode::new(string)),
        }
    }
    fn contains(&self, pat: &str) -> bool {
        let Some(first) = pat.chars().nth(0) else {
            panic!("Tried to check if empty string is contained in dictionary");
        };

        let node_match = first == self.node_value;
        if pat.len() == 1 {
            return node_match && self.end_node;
        }
        
        node_match && self.children.iter().any(|node| node.contains(&pat[1..]))
    }
    fn contains_part_of(&self, pat: &str) -> bool {
        let Some(first) = pat.chars().nth(0) else {
            panic!("Tried to check if empty string is partly contained in dictionary");
        };

        let node_match = first == self.node_value;
        if pat.len() == 1 {
            return node_match;
        }
        
        node_match && self.children.iter().any(|node| node.contains_part_of(&pat[1..]))
    }
}

pub fn run(file_input: &str) {
    let (towel_types, requested_patterns) = file_input.split_once("\r\n\r\n").expect("Did not contain separating empty line");
    let towel_types: Dictionary = towel_types.split(", ").collect();

    let can_be_built: Vec<usize> = requested_patterns.par_lines().filter_map(|pattern| {
        Some(ways_to_craft_pattern(pattern.to_string(), &towel_types)).filter(|&i| i != 0)
    }).collect();

    println!("Problem 1: {}", can_be_built.len());
    println!("Problem 2: {}", can_be_built.par_iter().sum::<usize>());
}

#[memoize::memoize(Ignore: towel_types)]
fn ways_to_craft_pattern(pattern: String, towel_types: &Dictionary) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    let mut combinations = 0;
    for i in 1..=pattern.len() {
        let slice = &pattern[..i];
        if !towel_types.contains_part_of(slice) {
            break;
        }
        if towel_types.contains(slice) {
            combinations += ways_to_craft_pattern(pattern[i..].to_string(), towel_types);
        }
    }
    combinations
}
