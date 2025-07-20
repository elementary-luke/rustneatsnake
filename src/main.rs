use popmanager::PopManager;
use raylib::prelude::*;
use runner::Runner;

use crate::agent::Agent;

use config::Config;
use network::Network;
use link::Link;
use neuron::*;
use network::Mutation;
use crate::vec2::Vec2i;

mod link;
mod neuron;
mod network;
mod config;
mod popmanager;
mod vec2;
mod grid;
mod agent;
mod runner;
mod specie;

fn main() 
{

    let mut manager = PopManager::new();
    manager.initialise_base_population();

    for i in 0..1000
    {
        if i % 10 == 0
        {
            manager.print_generation_statistics();
        }
        manager.next_generation();
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
     
    rl.set_target_fps(Config::fps);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_R)
        {
            runner = Runner::new(manager.networks[0].clone());
        }

        let space = rl.is_key_pressed(KeyboardKey::KEY_SPACE);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        runner.draw(&mut d);

        if Config::autorun || space
        {
            runner.step();
            // println!("{:?}", runner.grid.get_inputs());
        }
        
    }
}
