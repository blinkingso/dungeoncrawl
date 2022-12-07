use crate::prelude::*;

/// 地砖 地图的方块数
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/// Pre-defined set of tile types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

/// transform (x,y) to vector index
/// let index = (y * WIDTH) + x
/// let x = index % WIDTH
/// let y = index / WIDTH
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    /// Create a new `map`
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// Render the map. The map needs to be able to draw itself to the screen.
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        //TODO means what?
        ctx.set_active_console(0);

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                    }
                }
            }
        }
    }

    /// Check point(player) x/y coordinate pair is within the bounds of the map.
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Players can walk on floors but no through walls.
    /// Only `TileType::Floor` can walk through
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// Test if a map coordinate is valid
    /// return `None` if not else `Some(index)`
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}