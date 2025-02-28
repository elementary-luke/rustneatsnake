use network::Network;
use link::Link;
use neuron::*;

mod link;
mod neuron;
mod network;

fn main() 
{
    let mut net = Network::default();
    net.neurons.insert(0, Neuron {id : 0, activation : 0.5, kind : NeuronType::Input, ..Default::default()});
    net.neurons.insert(1, Neuron {id : 1, activation : 0.8, kind : NeuronType::Input, ..Default::default()});
    net.neurons.insert(2, Neuron {id : 2, activation : 1.0, kind : NeuronType::Output, ..Default::default()});

    let connections : Vec<(usize, usize)> = vec![(0, 2), (1, 2)];

    net.links.push(Link {from : 0, to : 2, weight : 0.2, ..Default::default()});
    net.links.push(Link {from : 1, to : 2, weight : 0.4, ..Default::default()});
    // for (from, to) in connections
    // {
    //     net.links.push(Link {from, to, weight : 0.3, ..Default::default()});
    // }

    net.calculate_output();
    println!("{}", net.neurons.get(&2).unwrap().activation);

}
