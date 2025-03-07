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

    manager.add(100);
    manager.networks[0].draw();
    for i in 0..10000
    {
        println!("{i}");
        manager.mutate_population(&mut innovation_count, &mut change_map);
    }
    manager.networks[0].draw();
    


    // let mut net = Network::default();
    // net.neurons.insert(0, Neuron {id : 0, activation : 0.5, kind : NeuronType::Input, ..Default::default()});
    // net.neurons.insert(1, Neuron {id : 1, activation : 0.8, kind : NeuronType::Input, ..Default::default()});
    // net.neurons.insert(2, Neuron {id : 2, activation : 1.0, kind : NeuronType::Hidden, ..Default::default()});
    // net.neurons.insert(3, Neuron {id : 3, activation : 1.0, kind : NeuronType::Hidden, ..Default::default()});
    // net.neurons.insert(4, Neuron {id : 4, activation : 1.0, kind : NeuronType::Hidden, ..Default::default()});
    // net.neurons.insert(5, Neuron {id : 5, activation : 1.0, kind : NeuronType::Hidden, ..Default::default()});

    // let connections : Vec<(usize, usize)> = vec![(0, 2), (1, 2)];

    // net.links.push(Link {from : 0, to : 2, weight : 0.2, ..Default::default()});
    // net.links.push(Link {from : 1, to : 2, weight : 0.4, ..Default::default()});
    // net.links.push(Link {from : 2, to : 3, weight : 0.4, ..Default::default()});
    // net.links.push(Link {from : 3, to : 4, weight : 0.4, ..Default::default()});
    // net.links.push(Link {from : 4, to : 5, weight : 0.4, ..Default::default()});
    // net.links.push(Link {from : 2, to : 4, weight : 0.4, ..Default::default()});

    // net.add_hidden_neuron(&mut innovation_count, &mut change_map);
    // println!("{}, {:?}", innovation_count, change_map);

    // net.draw();
    
    // for (from, to) in connections
    // {
    //     net.links.push(Link {from, to, weight : 0.3, ..Default::default()});
    // }

    //net.calculate_output();
    //println!("{}", net.neurons.get(&2).unwrap().activation);
}
