#[derive(Default, Debug)]
pub struct TrieNode {
    pub next: [Option<usize>; 10],
    pub terminals: Vec<usize>,
}

#[derive(Debug)]
pub struct Trie {
    pub nodes: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            nodes: vec![TrieNode::default()],
        }
    }

    pub fn add(&mut self, word: &Vec<u8>, hint: usize) {
        let mut cur: usize = 0; // root
        for &s in word {
            if let None = self.nodes[cur].next[s as usize] {
                self.nodes[cur].next[s as usize] = Some(self.nodes.len());
                self.nodes.push(TrieNode::default());
            }
            cur = self.nodes[cur].next[s as usize].unwrap();
        }
        self.nodes[cur].terminals.push(hint)
    }
}
