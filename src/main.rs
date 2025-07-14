use std::collections::HashMap;

use config::Config;
use network::Network;
use link::Link;
use neuron::*;
use network::Mutation;
use popmanager::PopManager;
use raylib::{prelude::*};
use grid::Grid;

use crate::vec2::Vec2i;

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
    manager.add(100);
    manager.simulate_population();
    manager.networks[0].draw();
    let mut gr = Grid::new();

    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("Hello, World")
        .build();
     
    rl.set_target_fps(5);

    while !rl.window_should_close() {
        // let desire = Vec2i::from((
        //     rl.is_key_down(KeyboardKey::KEY_D) as i16 - rl.is_key_down(KeyboardKey::KEY_A) as i16,
        //     rl.is_key_down(KeyboardKey::KEY_S) as i16 - rl.is_key_down(KeyboardKey::KEY_W) as i16
        // ));
        // gr.step(desire);

        

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        gr.draw(&mut d);
    }
}
