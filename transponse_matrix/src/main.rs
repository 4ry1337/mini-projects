fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {}
    }
}

fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    println!("Original:");
    matrix.iter().for_each(|row| {
        println!("{row:?}");
    });

    let transposed = transpose(matrix);
    println!("Transposed:");
    transposed.iter().for_each(|row| {
        println!("{row:?}");
    });
}
