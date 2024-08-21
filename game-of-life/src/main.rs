use game_of_life::conway::Conway;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "FULL");
    //let mut conway = Conway::new(1337, 30 * 141, 60, 141);
    let mut conway = Conway::new(1337, 15 * 85, 30, 85);
    conway.random_generation();
    //conway.set(1, 0, true);
    //conway.set(1, 1, true);
    //conway.set(1, 2, true);
    conway.run();
}
