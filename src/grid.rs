use std::cmp::min;

use rand::random_range;
use raylib::prelude::*;

use crate::config::Config;
use crate::vec2::Vec2i;

pub struct Grid 
{
    data : Vec<Vec<Object>>,
    segments : Vec<Vec2i>,
    dir : Vec2i,
    fruit_pos : Vec2i,
    pub running : bool,
    fruits_eaten : u16,
    steps_taken : u16,
    steps_without_fruit : u16,
}

impl Grid 
{
    pub fn new() -> Grid
    {
        let mut data : Vec<Vec<Object>> = vec![vec![Object::Empty; Config::grid_width]; Config::grid_height];
        for y in 0..Config::grid_height
        {
            for x in 0..Config::grid_width
            {
                if x == 0 || x == Config::grid_width - 1 || y == 0 || y == Config::grid_height - 1
                {
                    data[y][x] = Object::Wall;
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

        let head_pos : Vec2i = Vec2i::from((Config::grid_width as i16 / 2 , Config::grid_height as i16 / 2));
        data[head_pos.y as usize][head_pos.x as usize] = Object::Head;

        let mut grid = Grid {data : data, segments : vec![head_pos], dir : dir, fruit_pos : Vec2i::from((0, 0)), running : true, fruits_eaten : 0, steps_taken : 0, steps_without_fruit : 0};

        grid.spawn_fruit();

        return grid;
    }

    pub fn calculate_fitness(&self) -> f32 {
        return self.fruits_eaten as f32 * Config::fruit_value + self.steps_taken as f32 * Config::step_value
    }

    

    //make sure direction can only change according to the rules of snake i.e. cant go from moving right to left
    fn change_direction(&mut self, desire : Vec2i)
    {
        let moving_vertically = self.dir.x == 0;

        //make sure direction can only change according to the rules of snake i.e. cant go from moving right to left
        if moving_vertically
        {
            if desire.x != 0
            {
                self.dir.x = desire.x;
                self.dir.y = 0;
            }
        }
        else if desire.y != 0
        {
            self.dir.x = 0;
            self.dir.y = desire.y;
        }
    }

    pub fn step(&mut self, desire : Vec2i)
    {
        if !self.running
        {
            return;
        }

        let timeout = ((Config::grid_height * Config::grid_width) as u16) + (self.segments.len() as u16 * 50);
        
        if self.steps_without_fruit >= timeout
        {
            self.running = false;
            return;
        }
        
        self.change_direction(desire);

        let old_head = self.segments[0];
        let new_head = old_head + self.dir;
        
        let growing = self.data[new_head.y as usize][new_head.x as usize] == Object::Fruit;

        //remove last tail segment if a fruit was not eaten so that you can go in a circle
        if !growing
        {
            let tailest = self.segments.pop().unwrap();
            self.data[tailest.y as usize][tailest.x as usize] = Object::Empty;
        }

        let going_into = self.data[new_head.y as usize][new_head.x as usize];

        //leave early if going into body or wall
        if going_into == Object::Body || going_into == Object::Wall 
        {
            //put back head if run into wall
            self.data[old_head.y as usize][old_head.x as usize] = Object::Head;
            self.segments.push(old_head);

            self.running = false;
            return;
        }
        
        //make old head becomes body in grid
        if !self.segments.is_empty()
        {
            self.data[old_head.y as usize][old_head.x as usize] = Object::Body;
        }

        //add new head
        self.segments.insert(0, new_head);
        self.data[new_head.y as usize][new_head.x as usize] = Object::Head;

        //spawn new fruit if eaten
        if growing
        {
            self.steps_without_fruit = 0;
            self.fruits_eaten += 1;
            self.spawn_fruit();
        }
        self.steps_taken = min(500, self.steps_taken + 1);
        self.steps_without_fruit += 1;
    }


    pub fn spawn_fruit(&mut self) 
    {
        let mut x : usize = 0;
        let mut y : usize = 0;
        while self.data[y][x] != Object::Empty
        {
            x = random_range(1..=Config::grid_width - 1);
            y = random_range(1..=Config::grid_height - 1);
        }
        self.fruit_pos.x = x as i16;
        self.fruit_pos.y = y as i16;
        self.data[y][x] = Object::Fruit;
    }

    pub fn print_grid(&self)
    {
        for i in self.data.clone()
        {
            println!("{:?}", i.iter().map(|x| {
                match x {
                    Object::Head => "H",
                    Object::Body => "B",
                    Object::Wall => "W",
                    Object::Fruit => "O",
                    Object::Empty => "E",
                }
            }).collect::<Vec<&str>>());
        }
    }

    pub fn draw(&mut self, d : &mut RaylibDrawHandle) 
    {
        
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
         for y in 0..Config::grid_height
        {
            for x in 0..Config::grid_width
            {
                let xpos : f32= (x as i16 * Config::cell_width)  as f32;
                let ypos : f32= (y as i16 * Config::cell_width) as f32;
                let col : Color = match self.data[y][x] {
                    Object::Head => Color {r: 30, g: 90, b: 60, a: 255},
                    Object::Body => Color {r: 50, g: 150, b: 100, a: 255},
                    Object::Wall => Color {r: 70, g: 70, b: 120, a: 255},
                    Object::Fruit => Color {r: 200, g: 60, b: 60, a: 255},
                    Object::Empty => Color {r: 240, g: 240, b: 240, a: 255},
                };
                
                let width : f32=  Config::cell_width as f32;
                
                d.draw_rectangle_rounded(Rectangle{x : xpos, y : ypos, width, height : width}, 0.5, 10, col);
            }
        }
    }

    pub fn get_inputs(&self) -> Vec<f32> {
        let mut inputs : Vec<f32> = vec![0.0; Config::input_count];

        let dirs = vec![Vec2i::from((0, -1)), Vec2i::from((0, 1)), Vec2i::from((-1, 0)), Vec2i::from((1, 0))];

        //normalise vectors by max possible distance
        let max_hori = (Config::grid_width - 1) as f32;
        let max_verti = (Config::grid_height - 1) as f32;
        let max_hypo = (((Config::grid_height - 1).pow(2) + (Config::grid_width - 1).pow(2)) as f32).sqrt();

        //fruit
        let fruit_pos_diff = self.fruit_pos - self.segments[0];

        if fruit_pos_diff.y < 0 // fruit above
        {
            inputs[0] = 1.0 - (fruit_pos_diff.y as f32 / max_verti).abs();
            inputs[1] = 0.0;
        }
        else if fruit_pos_diff.y > 0
        {
            inputs[0] = 0.0;
            inputs[1] = 1.0 - (fruit_pos_diff.y as f32 / max_verti).abs();
        }
        else
        {
            inputs[0] = 0.0;
            inputs[1] = 0.0;
        }

        if fruit_pos_diff.x < 0 // fruit left
        {
            inputs[2] = 1.0 - (fruit_pos_diff.x as f32 / max_hori).abs();
            inputs[3] = 0.0;
        }
        else if fruit_pos_diff.x > 0
        {
            inputs[2] = 0.0;
            inputs[3] = 1.0 - (fruit_pos_diff.x as f32 / max_hori).abs();
        }
        else
        {
            inputs[2] = 0.0;
            inputs[3] = 0.0;
        }

        //walls
        for (i, dir) in dirs.iter().enumerate() {
            let mut pos = self.segments[0] + *dir;
            
            loop {
                match self.data[pos.y as usize][pos.x as usize] {
                    Object::Wall => {
                        break;
                    }

                    _ => {
                        pos += *dir;
                    }
                }
            }

            let displacement = (pos - self.segments[0]).fmagnitude();

            if i == 0 || i == 1
            {
                inputs[i + 4] =  1.0 - (displacement / max_verti);
            }
            else if i == 2 || i == 3
            {
                inputs[i + 4] =  1.0 - (displacement / max_hori) ;
            }
        }


        //body
        for (i, dir) in dirs.iter().enumerate() {
            let mut pos = self.segments[0] + *dir;
            let mut found_body = false;
            
            loop {
                match self.data[pos.y as usize][pos.x as usize] {
                    Object::Wall => {
                        break;
                    }

                    Object::Body => {
                        found_body = true;
                        break;
                    }

                    _ => {
                        pos += *dir;
                    }
                }
            }

            if found_body
            {
                let displacement = (pos - self.segments[0]).fmagnitude();
                if i == 0 || i == 1
                {
                    inputs[i + 8] =  1.0 - (displacement / max_verti);
                }
                else if i == 2 || i == 3
                {
                    inputs[i + 8] =  1.0 - (displacement / max_hori);
                }
            }
            else
            {
                inputs[i + 8] = 0.0;
            }

        }

        //direction of snake
        for (i, dir) in dirs.iter().take(4).enumerate() {
            inputs[i + 12] = (self.dir == *dir) as i16 as f32
        }

        // //length of snake
        // inputs[13] = self.segments.len() as f32 / ((Config::grid_height - 2) * (Config::grid_width - 2)) as f32;
        
        //bias neuron
        inputs[16] = 1.0;

        return inputs;
    }
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Object
{
    Head,
    Body,
    Wall,
    Fruit,
    Empty,
}