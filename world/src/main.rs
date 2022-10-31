use std::time::Instant;

pub struct TileWorld {
    height: usize,
    width: usize,
}

impl TileWorld {}

fn main() {
    const WORLD: TileWorld = TileWorld {
        height: 10, //Change
        width: 15, //Change
    };
    let mut tiles = [[0; WORLD.width]; WORLD.height];

    let start = Instant::now();

    tiles[0][0] = 1;
    for tile in tiles {
        println!("{:?}", tile);
    }

    println!("{:?}", start.elapsed());
}
