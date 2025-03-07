use crate::link::*;
use crate::network;
use crate::neuron::*;
use crate::network::*;
use crate::config::*;
use strum::IntoEnumIterator;
use std::collections::HashMap;

pub struct PopManager 
{
    pub networks : Vec<Network>,
}

impl Default for PopManager 
{
    fn default() -> PopManager 
    {
        return PopManager {networks : vec![]};
    }
}

impl PopManager 
{
    pub fn add(&mut self, n : usize)
    {
        let mut base_net = Network {..Default::default()};
        for i in 0..Config::input_count
        {
            base_net.neurons.insert(i, Neuron {id: i, activation: 0.0, kind: NeuronType::Input, ..Default::default()});
        }
        for i in Config::input_count..(Config::input_count + Config::output_count)
        {
            base_net.neurons.insert(i, Neuron {id: i, activation: 0.0, kind: NeuronType::Output, ..Default::default()});
        }

        for _ in 0..n
        {
            let mut net = base_net.clone();
            net.set_up_intial_links();
            self.networks.push(net);
        }
    }

    pub fn mutate_population(&mut self, innovation_count : &mut usize, change_map : &mut HashMap<(Mutation, usize, usize), usize>)
    {
        for i in 0..self.networks.len()
        {
            self.networks[i].mutate(innovation_count, change_map);
        }
    }
}
