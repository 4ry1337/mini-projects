use game_of_life::conway::Conway;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "FULL");
    let mut conway = Conway::new(1337, 16 * 16, 32, 32);
    conway.random_generation();
    //conway.set(1, 0, true);
    //conway.set(1, 1, true);
    //conway.set(1, 2, true);
    conway.run();
}
