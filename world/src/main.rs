use std::time::Instant;
use eyre::{Result, *};

pub struct TileWorld {
    height: usize,
    width: usize,
}

fn main() {
    const WORLD: TileWorld = TileWorld {
        height: 10, // Change
        width: 15, // Change
    };

    let mut tiles = [[0; WORLD.width]; WORLD.height];

    fn available_tiles(tiles_arr: [[i32; WORLD.width]; WORLD.height], pointer: Vec<usize>) -> Vec<Vec<usize>> {
        let mut available: Vec<Vec<usize>> = vec![];

        if tiles_arr[pointer[0]][pointer[1] - 1] == 0 {
            available.push(vec![pointer[0], pointer[1] - 1]);
            println!("left");
        }

        if tiles_arr[pointer[0]][pointer[1] + 1] == 0 {
            available.push(vec![pointer[0], pointer[1] + 1]);
            println!("right");
        }
        if tiles_arr[pointer[0] - 1][pointer[1]] == 0 {
            available.push(vec![pointer[0] - 1, pointer[1]]);
            println!("up");
        }
        if tiles_arr[pointer[0] + 1][pointer[1]] == 0 {
            available.push(vec![pointer[0] + 1, pointer[1]]);
            println!("down");
        }
        available

    }

    let start = Instant::now();

    tiles[0][0] = 1;
    for tile in tiles {
        println!("{:?}", tile);
    }

    let test_pointer = vec![0, 5];

    let available_tiles = available_tiles(tiles, test_pointer);
    for tile in &available_tiles {
        tiles[tile[0]][tile[1]] = 1;
    }
    println!("{:?}", available_tiles);

    for tile in tiles {
        println!("{:?}", tile);
    }
    println!("{:?}", start.elapsed());
}
