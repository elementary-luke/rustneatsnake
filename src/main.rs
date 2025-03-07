use std::collections::HashMap;

use config::Config;
use network::Network;
use link::Link;
use neuron::*;
use network::Mutation;
use popmanager::PopManager;

mod link;
mod neuron;
mod network;
mod config;
mod popmanager;

fn main() 
{
    let mut change_map : HashMap<(Mutation, usize, usize), usize> = HashMap::new();
    let mut innovation_count : usize = Config::input_count + Config::output_count;

    let mut manager = PopManager{..Default::default()};

    manager.add(2);
    manager.networks[0].draw();
    manager.networks[1].draw();
    // for i in 0..100
    // {
    //     println!("{i}");
    //     manager.mutate_population(&mut innovation_count, &mut change_map);
    // }
    manager.add_offspring();
    manager.networks[2].draw();
}
