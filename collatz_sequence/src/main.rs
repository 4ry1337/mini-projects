/// https://en.wikipedia.org/wiki/Collatz_conjecture
///
/// The Collatz Sequence is defined as follows, for an arbitrary n_1 greater than zero:
/// If `n_i == 1`, then the sequence terminates at `n_i`.
/// If `n_i` is even, then `n_i+1 = n_i / 2`.
/// If `n_i` is odd, then `n_i+1 = 3 * n_i  + 1`.
///
/// For example, beginning with n_1 = 3:
/// 3 is odd, so n_2 = 3 * 3 + 1 = 10;
/// 10 is even, so n_3 = 10 / 2 = 5;
/// 5 is odd, so n_4 = 3 * 5 + 1 = 16;
/// 16 is even, so n_5 = 16 / 2 = 8;
/// 8 is even, so n_6 = 8 / 2 = 4;
/// 4 is even, so n_7 = 4 / 2 = 2;
/// 2 is even, so n_8 = 1; and the sequence terminates.

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut l = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        l += 1;
    }
    l
}

fn main() {
    let collatz = collatz_length(11);
    println!("Length: {}", collatz); // should be 15
    assert!(collatz == 15);
}
