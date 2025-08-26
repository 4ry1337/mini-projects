use game_of_life::conway::Conway;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "FULL");
    //let mut conway = Conway::new(1337, 30 * 141, 60, 141);
    let seed = 1337;
    let population = 15 * 5;
    let height = 15;
    let width = 20;
    let mut conway = Conway::new(seed, population, height, width);
    conway.random_generation();
    //conway.set(1, 0, true);
    //conway.set(1, 1, true);
    //conway.set(1, 2, true);
    conway.run();
}
