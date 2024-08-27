use cube::Cube;

pub mod cube;

fn main() {
    let mut cube = Cube::new(30, 80, 15);
    cube.random_rotation();
    cube.run();
}
