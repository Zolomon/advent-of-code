use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, std::cmp::PartialEq, Copy)]
enum Tile {
    Edge,
    On,
    Off,
}

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn neighbours(grid: &Vec<Tile>, (x, y): (i32, i32)) -> usize {
    OFFSETS
        .iter()
        .map(|(dx, dy)| grid[((x + dx) + ((y + dy) * SIZE)) as usize])
        .filter(|x| *x == Tile::On)
        .count()
}

fn next(grid: &mut std::vec::Vec<Tile>) -> Vec<Tile> {
    let mut new_grid = vec![Tile::Edge; USIZE * USIZE];

    for y in 1..USIZE - 1 {
        for x in 1..USIZE - 1 {
            let count = neighbours(grid, (x as i32, y as i32));
            let pos = x + (y * USIZE);
            let c = grid[pos];
            new_grid[pos] = Tile::Off;
            if c == Tile::On && (count == 2 || count == 3) {
                new_grid[pos] = Tile::On;
            } else if c == Tile::Off && count == 3 {
                new_grid[pos] = Tile::On;
            }
        }
    }
    new_grid
}
fn parse(line: String) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];
    for c in line.chars() {
        tiles.push(match c {
            '.' => Tile::Off,
            '#' => Tile::On,
            _ => panic!(),
        });
    }
    tiles
}

fn solve(grid: &mut Vec<Tile>, part2: bool) -> usize {
    let mut i = 0;
    while i < ITERATIONS {
        let new_grid = next(grid);
        *grid = new_grid;
        if part2 {
            let end = SIZE - 2 - 1;
            grid[(0 + 1) + ((0 + 1) * SIZE) as usize] = Tile::On;
            grid[((end + 1) + ((0 + 1) * SIZE)) as usize] = Tile::On;
            grid[((end + 1) + ((end + 1) * SIZE)) as usize] = Tile::On;
            grid[(0 + 1) + ((end + 1) * SIZE) as usize] = Tile::On;
        }
        i += 1;
    }
    grid.iter().filter(|x| **x == Tile::On).count()
}

const ITERATIONS: i32 = 100;
const EDGE_OFFSET: i32 = 2;
const SIZE: i32 = 100 + EDGE_OFFSET;
const USIZE: usize = SIZE as usize;
fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let tiles = buffer
        .lines()
        .map(|x| x.unwrap())
        .map(parse)
        .flatten()
        .collect::<Vec<_>>();

    let mut grid = vec![Tile::Edge; USIZE * USIZE];

    for y in 0..(SIZE - EDGE_OFFSET) {
        for x in 0..(SIZE - EDGE_OFFSET) {
            grid[((x + 1) + ((y + 1) * SIZE)) as usize] =
                tiles[(x + (y * (SIZE - 2))) as usize].clone();
        }
    }

    let mut part2_grid = grid.clone();
    part2_grid[(0 + 1) + ((0 + 1) * SIZE) as usize] = Tile::On;
    part2_grid[((SIZE - 3 + 1) + ((0 + 1) * SIZE)) as usize] = Tile::On;
    part2_grid[((SIZE - 3 + 1) + ((SIZE - 3 + 1) * SIZE)) as usize] = Tile::On;
    part2_grid[(0 + 1) + ((SIZE - 3 + 1) * SIZE) as usize] = Tile::On;
    let part1 = solve(&mut grid, false);
    let part2 = solve(&mut part2_grid, true);

    println!("part1: {part1}");
    println!("part2: {part2}");
}
