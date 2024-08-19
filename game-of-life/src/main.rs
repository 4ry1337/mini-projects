use game_of_life::conway::Conway;

fn main() {
    let conway = Conway::new(0, 0, 32, 32);
    println!("{}", conway.neighbour_count(0, 0));
}
