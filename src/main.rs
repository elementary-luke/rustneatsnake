use std::collections::HashMap;

use config::Config;
use network::Network;
use link::Link;
use neuron::*;
use network::Mutation;
use popmanager::PopManager;
use raylib::{prelude::*};
use grid::Grid;
use runner::Runner;

use crate::{agent::Agent, vec2::Vec2i};

mod link;
mod neuron;
mod network;
mod config;
mod popmanager;
mod vec2;
mod grid;
mod agent;
mod runner;

fn main() 
{

    let mut manager = PopManager::new();
    manager.add();
    manager.simulate_population();
    manager.sort_population_by_fitness();

    for i in 0..1000
    {
        if i % 10 == 0
        {
            println!("{}, {:?}, {}", i, manager.networks[0].fitness, manager.get_avg_num_neurons());
        }
        manager.cull_weak();
        manager.add_offspring();
        manager.simulate_population();
        manager.sort_population_by_fitness();
    }
    
    manager.networks[0].draw();
    println!("{:?}", manager.networks[0].fitness);
    println!("{:?}", manager.networks.last().unwrap().fitness);

    println!("aab{:?}", Agent::new(manager.networks[0].clone()).evaluate());
    println!("aab{:?}", Agent::new(manager.networks[0].clone()).evaluate());
    println!("aab{:?}", Agent::new(manager.networks[0].clone()).evaluate());
    println!("aab{:?}", Agent::new(manager.networks[0].clone()).evaluate());
    println!("aab{:?}", Agent::new(manager.networks[0].clone()).evaluate());
    
    
    let mut runner = Runner::new(manager.networks[0].clone());

    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("Hello, World")
        .build();
     
    rl.set_target_fps(60); // 15 is good

    while !rl.window_should_close() {
        // let desire = Vec2i::from((
        //     rl.is_key_down(KeyboardKey::KEY_D) as i16 - rl.is_key_down(KeyboardKey::KEY_A) as i16,
        //     rl.is_key_down(KeyboardKey::KEY_S) as i16 - rl.is_key_down(KeyboardKey::KEY_W) as i16
        // ));
        // gr.step(desire);
        if rl.is_key_pressed(KeyboardKey::KEY_R)
        {
            runner = Runner::new(manager.networks[0].clone());
        }

        // let p = rl.is_key_pressed(KeyboardKey::KEY_SPACE);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        runner.draw(&mut d);

        // if p
        {
            runner.step();
            // println!("{:?}", runner.grid.get_inputs());
        }
        
    }
}
