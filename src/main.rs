use std::collections::HashMap;

use config::Config;
use network::Network;
use link::Link;
use neuron::*;
use network::Mutation;
use popmanager::PopManager;
use raylib::{ffi::SetTargetFPS, prelude::*};
use grids::Grid;

mod link;
mod neuron;
mod network;
mod config;
mod popmanager;
mod vec2;
mod grids;

fn main() 
{
    let mut change_map : HashMap<(Mutation, usize, usize), usize> = HashMap::new();
    let mut innovation_count : usize = Config::input_count + Config::output_count;

    let mut manager = PopManager{..Default::default()};

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
         
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
