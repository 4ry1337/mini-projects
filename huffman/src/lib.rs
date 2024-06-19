use std::collections::{BTreeMap, BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Debug)]
pub struct HuffmanNode {
    frequency: usize,
    value: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .frequency
            .cmp(&self.frequency)
            .then(self.value.cmp(&other.value))
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl HuffmanNode {
    // create leaf with value
    pub fn leaf(value: u8, frequency: usize) -> Self {
        Self {
            value: Some(value),
            frequency,
            left: None,
            right: None,
        }
    }

    //create node with childs
    pub fn node(frequency: usize, right: HuffmanNode, left: HuffmanNode) -> Self {
        Self {
            frequency,
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn tree(data: &str) -> Option<Box<Self>> {
        let mut weights = HashMap::new();

        let mut queue = BinaryHeap::new();

        for i in data.as_bytes() {
            let freq = weights.entry(*i).or_insert(0);
            *freq += 1;
        }

        for i in weights {
            queue.push(Self::leaf(i.0, i.1))
        }

        while queue.len() > 1 {
            if let (Some(left), Some(right)) = (queue.pop(), queue.pop()) {
                queue.push(Self::node(left.frequency + right.frequency, right, left))
            }
        }

        match queue.pop() {
            None => None,
            Some(root) => Some(Box::new(root)),
        }
    }
}

#[derive(Debug)]
pub struct HuffmanTable {
    counts: BTreeMap<usize, usize>,
    symbols: Vec<u8>,
}

impl HuffmanTable {
    pub fn from_codes(codes: &Vec<HuffmanCode>) -> Self {
        let mut counts = BTreeMap::new();
        let mut symbols = vec![];

        for code in codes.iter() {
            let freq = counts.entry(code.length).or_insert(0);
            *freq += 1;
            symbols.push(code.value);
        }

        Self { counts, symbols }
    }

    pub fn describe(&self) {
        println!("Counts:  {:?}", self.counts);
        println!("Symbols: {:?}", self.symbols);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HuffmanCode {
    value: u8,
    frequency: usize,
    length: usize,
    bits: u128,
}

impl Ord for HuffmanCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length
            .cmp(&other.length)
            .then(self.value.cmp(&other.value))
    }
}

impl PartialOrd for HuffmanCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl HuffmanCode {
    pub fn from_tree(tree: &Option<Box<HuffmanNode>>) -> Vec<Self> {
        fn collect(
            output: &mut Vec<HuffmanCode>,
            node: &Option<Box<HuffmanNode>>,
            indent: usize,
            bits: u128,
        ) {
            if let Some(node) = node {
                if let Some(value) = node.value {
                    output.push(HuffmanCode {
                        value,
                        frequency: node.frequency,
                        length: indent,
                        bits,
                    });
                }

                // adds 0 to the end
                collect(output, &node.left, indent + 1, bits << 1);

                // adds 1 to the end
                collect(output, &node.right, indent + 1, bits << 1 | 0x1);
            }
        }

        let mut result = vec![];

        collect(&mut result, &tree, 0, 0);

        result
    }

    pub fn from_table(table: &mut HuffmanTable) -> Vec<Self> {
        let mut result = vec![];

        let mut bits = 0;

        for symbol in &table.symbols {
            let mut entry = table.counts.first_entry().unwrap();

            result.push(HuffmanCode {
                value: *symbol,
                frequency: *entry.get(),
                length: *entry.key(),
                bits,
            });

            *entry.get_mut() -= 1;

            bits += 1;

            if *entry.get() == 0 {
                bits <<= 1;
                entry.remove();
            }
        }

        result
    }

    pub fn as_canonical(codes: &Vec<Self>) -> Vec<Self> {
        let mut sorted: Vec<Self> = codes.iter().cloned().collect();

        sorted.sort();

        let mut bits = 0;
        let mut length = 0;

        for code in sorted.iter_mut() {
            while length < code.length {
                bits <<= 1;
                length += 1;
            }

            code.bits = bits;
            bits += 1;
        }

        sorted
    }

    pub fn describe(codes: &Vec<Self>) {
        print!("char\tutf8\tfreq\tlength\tbits\n- - - - - - - - - - - - - - - - - -\n");
        for code in codes.iter() {
            println!(
                "'{}'\t{:>3}\t{}\t{}\t{:0width$b}",
                code.value as char,
                code.value,
                code.frequency,
                code.length,
                code.bits,
                width = code.length
            )
        }
    }
}

pub fn encode(data: &str) -> (String, HuffmanTable) {
    let mut result = String::new();

    let tree = HuffmanNode::tree(data);

    let codes = HuffmanCode::from_tree(&tree);

    let canonical = HuffmanCode::as_canonical(&codes);

    // HuffmanCode::describe(&canonical);

    for i in data.as_bytes() {
        let x = canonical.iter().find(|&x| x.value == *i).unwrap();
        result += &format!("{:0width$b}", x.bits, width = x.length);
    }

    let table = HuffmanTable::from_codes(&canonical);

    (result, table)
}

pub fn decode(data: &str, mut table: HuffmanTable) -> Option<Vec<u8>> {
    let mut result = vec![];

    let codes = HuffmanCode::from_table(&mut table);

    let mut binary = String::new();

    for i in data.chars() {
        binary.push(i);
        if let Some(code) = codes
            .iter()
            .find(|&code| format!("{:0width$b}", code.bits, width = code.length) == binary)
        {
            result.push(code.value);
            binary.clear();
        }
    }

    Some(result)
}

// pub fn decode(&self, bits: u128) -> Option<u8> {
//     let mut length = 1;
//     let mut first: u128 = 0;
//     let mut bits = bits;
//
//     let mut code = 0;
//     let mut count: u128 = 0;
//     let mut offset: u128 = 0;
//
//     while length < 8 {
//         code |= bits & 0x1;
//         count = self.counts[&length] as u128;
//
//         if code < first + count {
//             return Some(self.symbols[offset as usize + (code - first) as usize]);
//         }
//
//         offset += count;
//         first += count;
//         length += 1;
//
//         first <<= 1;
//         code <<= 1;
//         bits >>= 1;
//     }
//
//     None
// }

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    #[test]
    fn compression_test() {
        let message = "the quick brown fox jumps over the lazy dog";

        println!("Data: '{}'", message);

        let (encoded, table) = encode(message);

        println!("Encoded: '{}'", encoded);

        let decoded = decode(&encoded, table);

        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), message.as_bytes());

        let len_uncompressed: f32 = message.len() as f32 * 4.0 * 8.0;
        let len_compressed: f32 = encoded.len() as f32;
        println!("Uncompressed Bitwise Length {}", len_uncompressed);
        println!("Bitwise Length {}", len_compressed);
        println!(
            "Compression Rate: {}%",
            (len_compressed / len_uncompressed) * 100.0
        );
    }
}
