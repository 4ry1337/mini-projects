pub fn encoding(data: &str) -> String {
    let mut encoded = String::new();
    let mut i = 0;

    while i < data.len() {
        let mut count = 1;
        while i < data.len() && data.chars().nth(i) == data.chars().nth(i + 1) {
            count += 1;
            i += 1;
        }
        encoded.push_str(&count.to_string());
        encoded.push(data.chars().nth(i).unwrap());
        i += 1;
    }

    encoded
}

pub fn decoding(data: &str) -> String {
    let mut decoded = String::new();
    let mut current_num = String::new();

    for c in data.chars() {
        if c.is_numeric() {
            current_num.push(c);
        } else {
            let count = current_num.parse::<usize>().unwrap();
            decoded.push_str(&c.to_string().repeat(count));
            current_num = String::new();
        }
    }

    decoded
}

#[cfg(test)]
mod tests {
    use crate::{decoding, encoding};

    #[test]
    fn encoding_test() {
        let message = "wwwwddr";
        println!("Message: {message}");
        let result = encoding(message);
        println!("Encoded Message: {result}");
        assert_eq!(result, "4w2d1r");
    }
    #[test]
    fn decoding_test() {
        let encoded_message = "4w2d2r";
        println!("Encoded Message:: {encoded_message}");
        let result = decoding(encoded_message);
        println!("Decoded Message: {result}");
        assert_eq!(result, "wwwwddrr");
    }
}
