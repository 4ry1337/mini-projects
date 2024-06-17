use std::collections::HashMap;

fn frequency(data: &str) -> HashMap<char, i32> {
    let mut weights = HashMap::new();
    for i in data.chars() {
        let freq = weights.entry(i).or_insert(0);
        *freq += 1;
    }
    weights
}

#[derive(PartialEq, Eq, Debug)]
pub struct Node {
    frequency: i32,
    value: Option<i32>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(frequency: i32, value: Option<i32>) -> Self {
        Self {
            value,
            frequency,
            left: None,
            right: None,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .frequency
            .cmp(&self.frequency)
            .then(self.value.cmp(&other.value))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn compression(data: &str) -> String {
    let weights = frequency(data);
    println!("{:#?}", weights);
    return String::new();
}

// pub fn decoding(data: &str) -> String {
// }
//
#[cfg(test)]
mod tests {
    use crate::frequency;

    #[test]
    fn frequency_test() {
        let weights = frequency("HELLO WORLD");
        assert_eq!(weights.get(&'O'), Some(&2));
        assert_eq!(weights.get(&'L'), Some(&3));
    }
    // #[test]
    // fn encoding_test() {
    //     let compressed = compression("A DEAD DAD CEDED A BAD BABE A BEADED ABACA BED");
    //     assert_eq!(compressed, "00100010101110110101111101101100")
    // }
    // #[test]
    // fn decoding_test() {
    // }
}
