use std::collections::HashMap;

use config::Config;
use network::Network;
use link::Link;
use neuron::*;
use network::Mutation;
use popmanager::PopManager;
use raylib::{prelude::*};
use grid::Grid;

mod link;
mod neuron;
mod network;
mod config;
mod popmanager;
mod vec2;
mod grid;
mod agent;

fn main() 
{
    let mut change_map : HashMap<(Mutation, usize, usize), usize> = HashMap::new();
    let mut innovation_count : usize = Config::input_count + Config::output_count;

    let mut manager = PopManager{..Default::default()};
    let mut gr = Grid::new();
    gr.print_grid();

    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("Hello, World")
        .build();
     
    rl.set_target_fps(5);

    while !rl.window_should_close() {
        gr.step(&rl);
        

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        gr.draw(&mut d);
    }
}
