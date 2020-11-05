mod draw;
use draw::*;

const MAPWIDTH: usize = 30;
const MAPHEIGHT: usize = 10;
const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

#[derive(Default)]
pub struct GameState {
    pub map: Map,
}

fn main() {
    let map = Map {
        tiles: vec![TileType::Wall; MAPCOUNT],
        width: MAPWIDTH as i32,
        height: MAPHEIGHT as i32,
    };
    let state = GameState { map };

    render(state);
}
