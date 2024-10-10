use std::collections::HashMap;

use utils::write_file;

const HEADER_END: &str = "HEADER_END";
const HEADER_END_BYTES: &[u8] = HEADER_END.as_bytes();

type CodeMap = HashMap<char, String>;
type ReverseCodeMap<'a> = HashMap<&'a String, &'a char>;

#[derive(Debug)]
pub struct HuffmanCompression {
    text: String,
    code_map: CodeMap,
    encoded_bytes: Vec<u8>,
}

impl HuffmanCompression {
    pub fn encode(text: &str) -> HuffmanCompression {
        let tree = HuffmanTree::new(text);
        let code_map = tree.get_code_map();
        let encoded_bytes = text
            .chars()
            .map(|character| code_map.get(&character).unwrap().to_owned())
            .collect::<String>()
            .as_bytes()
            .chunks(8)
            .map(|chunk| chunk.iter().fold(0_u8, |acc, &b| (acc << 1) | (b - b'0')))
            .collect::<Vec<_>>();
        HuffmanCompression {
            text: text.to_string(),
            code_map,
            encoded_bytes,
        }
    }

    fn encode_header(&self) -> Vec<u8> {
        let header_str = self
            .code_map
            .iter()
            .map(|(character, code)| format!("{}:{}", *character as u8, code))
            .collect::<Vec<String>>()
            .join("\n")
            + HEADER_END;
        header_str.as_bytes().to_vec()
    }

    pub fn export_encoded(&self, file_path: &str) {
        let mut bytes_vec = self.encode_header();
        {
            bytes_vec.extend(&self.encoded_bytes);
        }
        write_file(file_path, &bytes_vec);
    }

    pub fn decode_header(header: &[u8]) -> CodeMap {
        header
            .split(|&b| b == b'\n')
            .map(|line| {
                let line_str = String::from_utf8(line.to_vec()).unwrap();
                let (character, code) = line_str.split_once(":").unwrap();
                println!("character before: {}", character);
                println!("code before: {}", code);
                (char::from_u32(character.trim().parse::<u32>().unwrap()).unwrap(), code.trim().to_string())
            })
            .inspect(|(character, code)| {
                println!("character after: {}", character);
                println!("code after: {}", code);
            })
            .collect()
    }

    pub fn decode(encoded_bytes: &Vec<u8>) -> HuffmanCompression {
        let header_end_index = encoded_bytes
            .windows(HEADER_END_BYTES.len())
            .position(|window| window == HEADER_END_BYTES)
            .unwrap();
        let (header, encoded_body) = encoded_bytes.split_at(header_end_index);

        let code_map = Self::decode_header(header);

        // create a reverse code map
        let reverse_code_map = code_map
            .iter()
            .map(|(character, code)| (code, character))
            .collect::<ReverseCodeMap>();

        let mut sorted_codes = reverse_code_map.keys().collect::<Vec<_>>();

        sorted_codes.sort_by(|a, b| b.len().cmp(&a.len()));
        println!("sorted_codes: {:?}", sorted_codes);

        let encoded_bytes = encoded_body.to_vec();

        let mut text = encoded_bytes
            .iter()
            // .take(100)
            .map(|byte| format!("{:08b}", byte))
            .collect::<String>();

        for code in sorted_codes {
            let character = reverse_code_map.get(code).unwrap();
            text = text.replace(*code, &character.to_string());
        }

        HuffmanCompression {
            text,
            code_map,
            encoded_bytes,
        }
    }

    pub fn export_decoded(&self, file_path: &str) {
        write_file(file_path, &self.text.as_bytes().to_vec());
    }
}

#[derive(Debug)]
struct HuffmanTree {
    root: HuffmanNode,
}

impl HuffmanTree {
    fn get_frequency_map(text: &str) -> HashMap<char, u32> {
        let mut frequency_map = HashMap::new();
        for character in text.chars() {
            *frequency_map.entry(character).or_insert(0 as u32) += 1;
        }
        frequency_map
    }

    fn new(text: &str) -> HuffmanTree {
        let frequency_map = HuffmanTree::get_frequency_map(text);

        let mut priority_queue = PriorityQueue::from_frequency_map(&frequency_map);
        let root = Self::build_tree(&mut priority_queue);
        HuffmanTree { root }
    }

    fn build_tree(priority_queue: &mut PriorityQueue) -> HuffmanNode {
        while priority_queue.nodes.len() > 1 {
            let left = priority_queue.pop();
            let right = priority_queue.pop();
            let mut new_node = HuffmanNode::new(None, left.frequency + right.frequency);
            new_node.left = Some(Box::new(left));
            new_node.right = Some(Box::new(right));
            priority_queue.push(new_node);
        }
        let root = priority_queue.pop();
        root
    }

    fn get_code_map(&self) -> CodeMap {
        let mut code_map = HashMap::new();
        self.root.get_code(&mut code_map, String::new());
        code_map
    }
}

#[derive(Debug)]
struct HuffmanNode {
    character: Option<char>,
    frequency: u32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(character: Option<char>, frequency: u32) -> HuffmanNode {
        HuffmanNode {
            character,
            frequency,
            left: None,
            right: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.character.is_some()
    }

    fn get_code(&self, code_map: &mut CodeMap, code: String) {
        if let Some(left) = &self.left {
            left.get_code(code_map, code.clone() + "0");
        }

        if let Some(right) = &self.right {
            right.get_code(code_map, code.clone() + "1");
        }

        if self.is_leaf() {
            code_map.insert(self.character.unwrap(), code.clone());
        }
    }
}

struct PriorityQueue {
    nodes: Vec<HuffmanNode>,
}

impl PriorityQueue {
    fn from_frequency_map(frequency_map: &HashMap<char, u32>) -> Self {
        let mut priority_queue = Self { nodes: Vec::new() };
        for (character, frequency) in frequency_map {
            priority_queue.push(HuffmanNode::new(Some(*character), *frequency));
        }
        priority_queue
    }

    fn push(&mut self, node: HuffmanNode) {
        self.nodes.push(node);
        self.nodes.sort_by_key(|node| node.frequency);
    }

    fn pop(&mut self) -> HuffmanNode {
        self.nodes.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, fs::read_to_string};
    fn get_testing_string() -> String {
        let file_path = "test.txt";
        let file_contents = read_to_string(file_path).expect("Failed to read file");
        file_contents
    }

    #[test]
    fn test_get_frequency_map() {
        let testing_string = get_testing_string();
        let frequency_map = HuffmanTree::get_frequency_map(&testing_string);
        assert_eq!(frequency_map.get(&'X'), Some(&333));
        assert_eq!(frequency_map.get(&'t'), Some(&223000));
    }

    #[test]
    fn test_create_priority_queue() {
        let frequency_map = HashMap::from([
            ('C', 32),
            ('D', 42),
            ('E', 120),
            ('K', 7),
            ('L', 42),
            ('M', 24),
            ('U', 37),
            ('Z', 2),
        ]);
        let priority_queue = PriorityQueue::from_frequency_map(&frequency_map);
        assert_eq!(priority_queue.nodes.len(), 8);
        assert_eq!(priority_queue.nodes[0].frequency, 2);
        assert_eq!(priority_queue.nodes[7].frequency, 120);
    }

    #[test]
    fn test_build_tree() {
        let mut priority_queue = PriorityQueue {
            nodes: vec![
                HuffmanNode::new(Some('Z'), 2),
                HuffmanNode::new(Some('K'), 7),
                HuffmanNode::new(Some('M'), 24),
                HuffmanNode::new(Some('C'), 32),
                HuffmanNode::new(Some('U'), 37),
                HuffmanNode::new(Some('D'), 42),
                HuffmanNode::new(Some('L'), 42),
                HuffmanNode::new(Some('E'), 120),
            ],
        };
        let root = HuffmanTree::build_tree(&mut priority_queue);
        assert_eq!(root.frequency, 306);
        let left = root.left.unwrap();
        assert_eq!(left.frequency, 120);
        assert_eq!(left.character, Some('E'));
        let right = root.right.unwrap();
        assert_eq!(right.frequency, 186);
        assert_eq!(right.character, None);
    }

    #[test]
    fn test_get_code_map() {
        let mut priority_queue = PriorityQueue {
            nodes: vec![
                HuffmanNode::new(Some('Z'), 2),
                HuffmanNode::new(Some('K'), 7),
                HuffmanNode::new(Some('M'), 24),
                HuffmanNode::new(Some('C'), 32),
                HuffmanNode::new(Some('U'), 37),
                HuffmanNode::new(Some('D'), 42),
                HuffmanNode::new(Some('L'), 42),
                HuffmanNode::new(Some('E'), 120),
            ],
        };
        let root = HuffmanTree::build_tree(&mut priority_queue);
        let mut code_map = HashMap::new();
        root.get_code(&mut code_map, String::new());
        assert_eq!(code_map.get(&'C'), Some(&"1110".to_string()));
        assert_eq!(code_map.get(&'D'), Some(&"101".to_string()));
        assert_eq!(code_map.get(&'E'), Some(&"0".to_string()));
        assert_eq!(code_map.get(&'K'), Some(&"111101".to_string()));
        assert_eq!(code_map.get(&'L'), Some(&"110".to_string()));
        assert_eq!(code_map.get(&'M'), Some(&"11111".to_string()));
        assert_eq!(code_map.get(&'U'), Some(&"100".to_string()));
        assert_eq!(code_map.get(&'Z'), Some(&"111100".to_string()));
    }
}
