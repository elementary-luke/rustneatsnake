use crate::agent::Agent;
use crate::neuron::*;
use crate::network::*;
use crate::config::*;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rand::prelude::IndexedRandom;
use std::collections::HashMap;
use rand::random_range;

pub struct PopManager 
{
    pub innovation_count : usize,
    pub change_map : HashMap<(Mutation, usize, usize), usize>,
    pub net_count : usize,
    pub networks : Vec<Network>,
    pub generation : usize,
}

impl PopManager 
{
    pub fn new() -> PopManager
    {
        return PopManager {innovation_count : Config::input_count + Config::output_count, change_map : HashMap::new(), net_count : 0, networks : vec![], generation : 0};
    }

    pub fn print_generation_statistics(&self)
    {
        println!("gen:{}", self.generation);
        println!("best fitness:{}", self.networks[0].fitness.unwrap_or_default());
        println!("avg fitness:{}", self.get_avg_fitness());
        println!("avg neuron count: {}", self.get_avg_num_neurons());
        println!("avg link count: {}", self.get_avg_num_links());
        println!("---------------------");
    }

    pub fn add(&mut self)
    {
        let mut base_net = Network::new(0);
        for i in 0..Config::input_count
        {
            base_net.neurons.insert(i, Neuron {id: i, activation: 0.0, kind: NeuronType::Input, ..Default::default()});
        }
        for i in Config::input_count..(Config::input_count + Config::output_count)
        {
            base_net.neurons.insert(i, Neuron {id: i, activation: 0.0, kind: NeuronType::Output, ..Default::default()});
        }
        self.innovation_count = Config::input_count + Config::output_count + Config::input_count * Config::output_count;
        base_net.set_up_intial_links();

        for _ in 0..Config::population_size
        {
            let mut net = base_net.clone();
            net.randomise_all_link_weights();
            net.id = self.net_count;
            self.net_count += 1;
            self.networks.push(net);
        }
    }

    pub fn initialise_base_population(&mut self)
    {
        self.add();
        self.simulate_population();
        self.sort_population_by_fitness();
    }

    pub fn next_generation(&mut self)
    {
        self.cull_weak();
        self.add_offspring();
        self.simulate_population();
        self.sort_population_by_fitness();
        self.generation += 1;
    }

    pub fn cull_weak(&mut self)
    {
        match Config::cull_method
        {
            0 => {
                self.cutoff_cull();
            },
            1 => {
                let num_to_keep = (Config::survival_percentage * Config::population_size as f32) as usize;
                self.networks.truncate(num_to_keep);
            },
            2 => {
                let num_to_keep = (Config::survival_percentage * Config::population_size as f32) as usize;
                self.networks.truncate(num_to_keep);
            },

            _ => ()
        }
    }

    pub fn cutoff_cull(&mut self)
    {
        let num_to_keep = Config::survival_percentage  as usize * self.networks.len();
        self.networks.truncate(num_to_keep);
    }

    pub fn cull_stochastically(&mut self)
    {
        let num_to_keep = (Config::survival_percentage as f32 * self.networks.len() as f32) as usize;
        let mut total_fitness = self.networks.iter().map(|a| a.fitness.unwrap_or_default()).sum::<f32>();
        let mut to_keep : Vec<Network> = vec![];

        //keep best through elitism
        let best = self.networks.remove(0);
        to_keep.push(best);
        
        while to_keep.len() < num_to_keep
        {
            let mut selected_index = 0;
            let mut r = random_range(0.0..=total_fitness);
            for i in 0..self.networks.len()
            {
                let net_fitness = self.networks[i].fitness.unwrap_or_default();
                r -= net_fitness;
                if r <= 0.0
                {
                    selected_index = i;
                    break;
                }
            }

            let selected = self.networks.remove(selected_index);
            total_fitness -= selected.fitness.unwrap_or_default();
            to_keep.push(selected);
        }
        self.networks = to_keep;
        self.sort_population_by_fitness();
    }

    pub fn cull_tournament()
    {

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
            let mut offspring = p1.crossover(&p2, &mut self.net_count);
            offspring.mutate(&mut self.innovation_count, &mut self.change_map);
            new_generation.push(offspring);
        }
        self.networks = new_generation;
    }


    //not being used rn
    //maybe call a bunch of times if the base networks are minimal/empty
    pub fn mutate_population(&mut self)
    {
        if !Config::global_change_map
        {
            self.change_map.clear();
        }
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
                if Config::force_reevalutaion || net.fitness.is_none() // if kept through elitism, don't need to rerun
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

    pub fn get_avg_num_neurons(&self) -> f32
    {
        return self.networks.iter().map(|net| net.neurons.len()).sum::<usize>() as f32 / self.networks.len() as f32;
    }

    pub fn get_avg_num_links(&self) -> f32
    {
        return self.networks.iter().map(|net| net.links.len()).sum::<usize>() as f32 / self.networks.len() as f32;
    }

    pub fn get_avg_fitness(&self) -> f32
    {
        return self.networks.iter().map(|net| net.fitness.unwrap_or_default()).sum::<f32>() / self.networks.len() as f32;
    }
}
