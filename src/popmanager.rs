use crate::agent::Agent;
use crate::link::*;
use crate::network;
use crate::neuron::*;
use crate::network::*;
use crate::config::*;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use strum::IntoEnumIterator;
use rand::prelude::IndexedRandom;
use std::collections::HashMap;

pub struct PopManager 
{
    pub innovation_count : usize,
    pub change_map : HashMap<(Mutation, usize, usize), usize>,
    pub networks : Vec<Network>,
}

impl PopManager 
{
    pub fn new() -> PopManager
    {
        return PopManager {innovation_count : Config::input_count + Config::output_count, change_map : HashMap::new(), networks : vec![]};
    }

    pub fn add(&mut self)
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
        self.innovation_count = base_net.neurons.len();

        for _ in 0..Config::population_size
        {
            let mut net = base_net.clone();
            net.set_up_intial_links();
            self.networks.push(net);
        }
    }

    pub fn cull_weak(&mut self)
    {
        let num_to_keep = (Config::survival_percentage * Config::population_size as f32) as usize;
        self.networks.truncate(num_to_keep);
    }

    pub fn add_offspring(&mut self)
    {
        if self.networks.len() == 0
        {
            return;
        }
        let mut new_generation : Vec<Network> = vec![];

        //keep top 2 through elitism
        new_generation.push(self.networks[0].clone());
        new_generation.push(self.networks[1].clone());

        while new_generation.len() < Config::population_size
        {
            let p1 = self.networks.choose(&mut rand::rng()).unwrap();
            let p2 = self.networks.choose(&mut rand::rng()).unwrap();
            let mut offspring = p1.crossover(&p2);
            offspring.mutate(&mut self.innovation_count, &mut self.change_map);
            new_generation.push(offspring);
        }
        self.networks = new_generation;
    }


    //not being used rn
    //maybe call a bunch of times if the base networks are minimal/empty
    pub fn mutate_population(&mut self)
    {
        for i in 0..self.networks.len()
        {
            self.networks[i].mutate(&mut self.innovation_count, &mut self.change_map);
        }
    }

    pub fn simulate_population(&mut self)
    {
        //singlethreaded
        // for i in 0..self.networks.len()
        // {
        //     if self.networks[i].fitness.is_none() // if kept through elitism, don't need to rerun
        //     {
        //         let mut agent = Agent::new(self.networks[i].clone());
        //         self.networks[i].fitness = Some(agent.evaluate());
        //     }
        // }


        //multithreaded
        self.networks
            .par_iter_mut()
            .for_each(|net| {
                if net.fitness.is_none() // if kept through elitism, don't need to rerun
                {
                    let mut agent = Agent::new(net.clone());
                    net.fitness = Some(agent.evaluate());
                }
            });
    }

    pub fn sort_population_by_fitness(&mut self)
    {
        self.networks.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    pub fn get_avg_num_neurons(&self) -> f32{
        return self.networks.iter().map(|net| net.neurons.len()).sum::<usize>() as f32 / self.networks.len() as f32;
    }
}
