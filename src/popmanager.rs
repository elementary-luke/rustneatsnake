use crate::agent::Agent;
use crate::config;
use crate::neuron::*;
use crate::network::*;
use crate::config::*;
use crate::specie;
use crate::specie::Specie;
use rand::seq::SliceRandom;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rand::prelude::IndexedRandom;
use std::collections::HashMap;
use rand::random_range;
use tabled::{
    Tabled, Table, assert::assert_table,
    builder::Builder,
    settings::{Style, Alignment, object::Columns},
};
use std::iter::once;

pub struct PopManager 
{
    pub innovation_count : usize,
    pub change_map : HashMap<(Mutation, usize, usize), usize>,
    pub net_count : usize, //used for id
    pub specie_count : usize, //used for id
    pub species : Vec<Specie>,
    pub networks : Vec<Network>,
    pub generation : usize,
    pub delta_t : f32,
}

impl PopManager 
{
    pub fn new() -> PopManager
    {
        return PopManager {innovation_count : 0, change_map : HashMap::new(), net_count : 0, specie_count : 0, species : vec![], networks : vec![], generation : 0, delta_t : Config::base_delta_t};
    }

    pub fn print_generation_statistics(&self)
    {
        if Config::use_species
        {
            let mut builder = Builder::default();
            builder.push_record(["id", "avg fitness", "size", "best fitness", "age", "stag_count", "representative id"]);

            let mut data : Vec<(usize, f32, usize, f32, u16, usize, usize)> = vec![];


            for s in &self.species 
            {
                data.push((s.id, s.avg_fitness, s.members.len(), s.best_fitness, s.age, s.stagnant_counter, s.representative.id));
            }
            data.sort_by(|a, b| b.1.partial_cmp(&a.1).expect("ERR"));

            data.iter().map(|(a, b, c, d, e, f, g)| vec![a.to_string(), b.to_string(), c.to_string(), d.to_string(), e.to_string(), f.to_string(), g.to_string()])
                .for_each(|a| builder.push_record(a));

            let mut table = builder.build();
            table.with(Style::rounded());

            println!("{table}");
        }

        println!("gen:{}", self.generation);
        println!("best fitness:{}", self.networks[0].fitness.unwrap_or_default());
        println!("avg fitness:{}", self.get_avg_fitness());
        println!("avg neuron count: {}", self.get_avg_num_neurons());
        println!("avg link count: {}", self.get_avg_num_links());
        
        if Config::use_species
        {
            println!("{}", self.delta_t);
            println!("num species: {}", self.species.len());
        }

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

        for _ in 0..Config::population_size
        {
            let mut net = base_net.clone();

            for _ in 0..Config::num_start_links
            {
                net.add_link(&mut self.innovation_count, &mut self.change_map);
            }
            
            net.id = self.net_count;
            self.net_count += 1;
            self.networks.push(net);
        }
    }

    pub fn adjust_delta_t(&mut self)
    {
        let diff = (self.species.len() as f32 - Config::target_num_species as f32).abs();
        let adjustment = Config::delta_t_adjustment * (diff / Config::target_num_species as f32);
        if self.species.len() > Config::target_num_species {
            self.delta_t += adjustment;
        } else if self.species.len() < Config::target_num_species {
            self.delta_t -= adjustment;
        }
        self.delta_t = self.delta_t.clamp(0.5, 10.0);
    }

    pub fn initialise_base_population(&mut self)
    {
        self.add();
        self.simulate_population();
        self.sort_population_by_fitness();
    }

    pub fn next_generation(&mut self)
    {
        if Config::use_species
        {
            self.handle_age_stagnation();
            self.speciate_population();
            self.adjust_delta_t();
        }
        else
        {
            self.cull_weak();
        }
        self.add_offspring();
        self.simulate_population();
        self.sort_population_by_fitness();
        if Config::use_species
        {
            self.set_species_representatives();
        }
        self.generation += 1;
    }

    pub fn handle_age_stagnation(&mut self )
    {
        for specie in &mut self.species 
        {
            specie.stagnant_counter += 1;
            specie.age += 1;
        }

        self.species.retain(|s| s.stagnant_counter < Config::stagnation_threshold);
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
        let num_to_keep = (Config::survival_percentage * self.networks.len() as f32)as usize ;
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
        //TODO
    }

    pub fn add_offspring(&mut self)
    {
        if Config::use_species
        {
            self.species_add_offspring();
        }
        else
        {
            self.simple_add_offspring();
        }
    }

    pub fn set_species_representatives(&mut self)
    {
        //TODO ADD OPTION FOR RANDOM!!
        for specie in  &mut self.species
        {
            specie.sort_members_by_fitness(&self.networks);//JUST CHOOSE MAX!!
            specie.representative = self.networks[specie.members[0]].clone();
        }
    }

    pub fn set_species_stats(&mut self)
    {
        for specie in  &mut self.species
        {
            specie.set_fitness_stats(&self.networks);
        }
    }

    pub fn speciate_population(&mut self)
    {
        for specie in &mut self.species
        {
            specie.clear_members();
        }

        for i in 0..self.networks.len()
        {
            if Config::shuffle_species_order
            {
                self.species.shuffle(&mut rand::rng());
            }
            let mut species_found : bool = false;
            for specie in &mut self.species
            {
                if self.networks[i].get_genetic_distance(&specie.representative) <= self.delta_t
                {
                    specie.members.push(i);
                    species_found = true;
                    break;
                }
            }

            //create new specie with it as representative if none currently match the network
            if !species_found
            {
                let mut new_specie = Specie::new(self.networks[i].clone(), self.specie_count);
                self.specie_count += 1;
                new_specie.members.push(i);
                self.species.push(new_specie);
            }
        }

        //remove speices with 0 members
        self.species.retain(|s| !s.members.is_empty());
    }

    pub fn species_add_offspring(&mut self)
    {
        if self.networks.len() == 0
        {
            return;
        }

        let mut new_generation : Vec<Network> = vec![];
        //keep top through elitism
        for i in 0..Config::elitistm_num
        {
            let mut offspring = self.networks[i].clone();
            if Config::mutate_elites
            {
                offspring.mutate(&mut self.innovation_count, &mut self.change_map);
                offspring.fitness = None;
            }
            new_generation.push(offspring);
        }

        self.set_species_stats();
        let proportions : Vec<usize> = self.get_species_proportions();

        for (i, quota) in proportions.iter().enumerate()
        {
            for _ in 0..*quota
            {
                let p1 = self.species[i].members.choose(&mut rand::rng()).unwrap();
                let p2 = self.species[i].members.choose(&mut rand::rng()).unwrap();
                let mut offspring = if random_range(0.0..=1.0) > Config::mut_not_cross_prob {
                    self.networks[*p1].crossover(&self.networks[*p2], &mut self.net_count)
                } else {
                    self.networks[*p1].crossover(&self.networks[*p1], &mut self.net_count) // clone
                };
                offspring.mutate(&mut self.innovation_count, &mut self.change_map);
                offspring.fitness = None;
                new_generation.push(offspring);
            }
        }


        self.networks = new_generation;
    }

    pub fn get_species_proportions(&mut self) -> Vec<usize>
    {
        let total_avg_fitness : f32 = self.species.iter().map(|x| x.avg_fitness).sum();
        let pop_size = Config::population_size - Config::elitistm_num;

        //qs for each s
        let raw_quotas: Vec<f32> = self.species.iter().map(|s| (s.avg_fitness / total_avg_fitness) * (pop_size as f32)).collect();

        //floor(qs) for each s, basically how many of the population have currently been assigned to the species
        let mut proportions: Vec<usize> = raw_quotas.iter().map(|qs| qs.floor() as usize).collect();

        //pop_size - sum(rs), basically how many of the population need to be assigned to species
        let leftovers = pop_size - proportions.iter().sum::<usize>();

        //rs = qs - floor(qs)
        //sort species in descending rs, so that highest rs is given leftovers first
        let mut remainders: Vec<(usize, f32)> = raw_quotas.iter().enumerate().map(|(i, qs)| (i, qs - qs.floor())).collect();

        remainders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());


        //give leftovers 1 by 1 giving leftovers first to species with a higher rs
        for i in 0..leftovers {
            let specie_index = remainders[i].0;
            proportions[specie_index] += 1;
        }

        if proportions.iter().sum::<usize>() != pop_size
        {
            println!("PROBLEM IN PROPORTIONS sum(proportions): {}  popsize: {}", proportions.iter().sum::<usize>(), pop_size)
        }

        return proportions;
    }

    pub fn simple_add_offspring(&mut self)
    {
        if self.networks.len() == 0
        {
            return;
        }
        let mut new_generation : Vec<Network> = vec![];

        //keep through elitism
        for i in 0..Config::elitistm_num
        {
            new_generation.push(self.networks[i].clone());
        }

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
        if Config::use_multithreading
        {
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
        else 
        {
            for i in 0..self.networks.len()
            {
                if self.networks[i].fitness.is_none() // if kept through elitism, don't need to rerun
                {
                    let mut agent = Agent::new(self.networks[i].clone());
                    self.networks[i].fitness = Some(agent.evaluate());
                }
            }
        }
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
