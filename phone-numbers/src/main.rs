use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, Write},
};

use phone_numbers::trie::Trie;

#[derive(Debug)]
struct Dictionary {
    words: Vec<String>,
    numwords: Vec<Vec<u8>>,
}

impl Dictionary {
    fn new(words: Vec<String>) -> Dictionary {
        let mut numwords: Vec<Vec<u8>> = Vec::with_capacity(words.len());
        for i in 0..words.len() {
            numwords.push(
                words[i]
                    .to_ascii_lowercase()
                    .chars()
                    .filter(|s| *s != '-' && *s != '"')
                    .map(|s| match s {
                        'e' => 0u8,
                        'j' | 'n' | 'q' => 1u8,
                        'r' | 'w' | 'x' => 2u8,
                        'd' | 's' | 'y' => 3u8,
                        'f' | 't' => 4u8,
                        'a' | 'm' => 5u8,
                        'c' | 'i' | 'v' => 6u8,
                        'b' | 'k' | 'u' => 7u8,
                        'l' | 'o' | 'p' => 8u8,
                        'g' | 'h' | 'z' => 9u8,
                        _ => panic!("unexpected symbol"),
                    })
                    .collect(),
            );
        }
        Dictionary { words, numwords }
    }
}

fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
}

fn parse_phone_number(phone_number: String) -> Vec<u8> {
    phone_number
        .chars()
        .filter(|&s| s != '-' && s != '/')
        .map(|s| s as u8 - '0' as u8)
        .collect()
}

fn encode_phone_number(phone_number: Vec<u8>, dict: &Dictionary, trie: &Trie) -> Vec<String> {
    #[derive(PartialEq, Clone, Debug)]
    enum Transition {
        Digit,
        Word { id: usize },
    }
    let len = phone_number.len();
    let mut graph: Vec<Vec<Transition>> = vec![vec![]; len + 1];

    for position in 0..len {
        if position != 0 && graph[position].is_empty() {
            continue;
        }
        // try putting a word to [position; ...]
        let mut any_word = false;
        let mut node_ind = 0 as usize;
        for i in position..phone_number.len() {
            if let None = trie.nodes[node_ind].next[phone_number[i] as usize] {
                break;
            }
            node_ind = trie.nodes[node_ind].next[phone_number[i] as usize].unwrap();
            for &word_id in &trie.nodes[node_ind].terminals {
                any_word = true;
                graph[i + 1].push(Transition::Word { id: word_id })
            }
        }

        // try putting a digit
        if !any_word {
            graph[position + 1].push(Transition::Digit)
        }
    }

    let mut path_state: Vec<&str> = vec![];
    let mut res: Vec<String> = vec![];

    struct Env<'a> {
        path_state: &'a mut Vec<&'a str>,
        res: &'a mut Vec<String>,
        graph: &'a Vec<Vec<Transition>>,
        phone_number: &'a Vec<u8>,
        dict: &'a Dictionary,
    }
    fn find_paths(env: &mut Env, position: usize, last_digit: bool) {
        if position == 0 {
            let mut enc = String::new();
            for i in (0..env.path_state.len()).rev() {
                if !enc.is_empty() {
                    enc.push(' ');
                }
                enc += env.path_state[i];
            }
            env.res.push(enc);
            return;
        }
        for tr in &env.graph[position] {
            let step;
            let is_digit;
            match tr {
                Transition::Digit => {
                    if last_digit {
                        continue;
                    }
                    let digits: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
                    step = 1;
                    env.path_state
                        .push(digits[env.phone_number[position - 1] as usize]);
                    is_digit = true;
                }
                Transition::Word { id } => {
                    step = env.dict.numwords[*id].len();
                    env.path_state.push(&env.dict.words[*id]);
                    is_digit = false;
                }
            }
            find_paths(env, position - step, is_digit);
            env.path_state.pop();
        }
    }

    find_paths(
        &mut Env {
            path_state: &mut path_state,
            res: &mut res,
            graph: &graph,
            phone_number: &phone_number,
            dict: &dict,
        },
        len,
        false,
    );

    res
}

fn main() {
    let dict = Dictionary::new(read_lines("dictionary.txt").collect());
    let mut trie = Trie::new();
    for i in 0..dict.words.len() {
        trie.add(&dict.numwords[i], i)
    }

    let output = File::create("output.txt").unwrap();
    let mutex = std::sync::Mutex::new(output);

    read_lines("input.txt")
        .par_bridge()
        .map(|x| {
            (
                x.clone(),
                encode_phone_number(parse_phone_number(x), &dict, &trie),
            )
        })
        .for_each(|result| {
            if result.1.is_empty() {
                return;
            }
            let mut of = mutex.lock().unwrap();
            for ele in result.1 {
                writeln!(of, "{}: {}", result.0, ele).unwrap();
            }
        })
}
