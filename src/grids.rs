use rand::random_range;

use crate::{config::Config, vec2::Vec2i};

pub struct Grid 
{
    data : Vec<Vec<Object>>,
    segments : Vec<Vec2i>,
    dir : Vec2i,
}

impl Grid 
{
    pub fn new() -> Grid
    {
        let mut data : Vec<Vec<Object>> = vec![vec![Object::empty; Config::grid_width]; Config::grid_height];
        for y in 0..Config::grid_height
        {
            for x in 0..Config::grid_width
            {
                if x == 0 || x == Config::grid_width - 1 || y == 0 || y == Config::grid_width - 1
                {
                    data[y][x] = Object::wall;
                }
            }
        }
        let dir : Vec2i = match random_range(0..=3) {
            0 => Vec2i::from((1, 0)),
            1 => Vec2i::from((-1, 0)),
            2 => Vec2i::from((0, 1)),
            3 => Vec2i::from((0, -1)),
            _ => Vec2i::from((0, 0))
        };
        return Grid {data : data, segments : vec![], dir : dir};
    }
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Object
{
    head,
    body,
    wall,
    fruit,
    empty,
}