use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>,
    value: Option<i32>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
                chs: HashMap::new(),
                value: None,
            },
        }
    }

    fn add_string(&mut self, string: String, value: i32) {
        let mut current_node = &mut self.root;
        for c in string.chars() {
            current_node = current_node.chs
                .entry(c)
                .or_insert(TrieNode {
                    chs: HashMap::new(),
                    value: None,
                });
        }
        current_node.value = Some(value);
    }

    // This function returns the number of strings stored in the trie, which is the number of nodes that have a value.
    fn length(&self) -> usize {
        fn count_nodes(node: &TrieNode) -> usize {
            let mut count = if node.value.is_some() { 1 } else { 0 };
            for child in node.chs.values() {
                count += count_nodes(child);
            }
            count
        }
        count_nodes(&self.root)
    }

    // This function returns an iterator over the characters in the trie, along with their associated values. 
    fn iter(&self) -> Vec<(char, Option<i32>)> {
        fn walk(node: &TrieNode, out: &mut Vec<(char, Option<i32>)>) {
            for (ch, child) in &node.chs {
                out.push((*ch, child.value));
                walk(child, out);
            }
        }   
        let mut result = Vec::new();
        walk(&self.root, &mut result);
        result
    }

    // This function searches the Trie for a key and returns a reference to that key's corresponding node if it exists.
    fn find(&self, key: &String) -> Option<&TrieNode> {
        let mut current_node = &self.root;
        for c in key.chars() {
            current_node = current_node.chs.get(&c)?;
        }
        Some(current_node)
    }

    // This function deletes a key from the Trie and returns the value associated with that key if it exists.
    fn delete(&mut self, key: &String) -> Option<i32> {
        let mut current_node = &mut self.root;
        for c in key.chars() {
            current_node = current_node.chs.get_mut(&c)?;
        }
        current_node.value.take()
    }

}

fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);
    println!("{:#?}", trie);
}