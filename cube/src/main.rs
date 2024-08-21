use cube::Cube;

pub mod cube;

fn main() {
    let mut cube = Cube::new(30, 80, 10, 0.0, 100.0, 0.07, 40.0);
    cube.run();
}
