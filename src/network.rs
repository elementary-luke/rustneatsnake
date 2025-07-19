use crate::link::*;
use crate::neuron;
use crate::neuron::*;
use std::cmp::max;
use std::collections::HashMap;
use std::net;
use std::thread::current;
use crate::config::Config;
use rand::random_range;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;
use rand::prelude::IndexedMutRandom;
use std::collections::HashSet;
use rand_distr::{Distribution, Normal};
use petgraph::prelude::Graph;
use petgraph::prelude::NodeIndex;
use petgraph::dot::Dot;

#[derive(Debug, Clone)]
pub struct Network 
{
    pub id : usize,
    pub neurons : HashMap<usize, Neuron>,
    pub links : Vec<Link>,
    pub fitness : Option<f32>
}

impl Network 
{
    pub fn topological_sort(&self) -> Vec<usize>
    {
        let mut neighbours : HashMap<usize, Vec<usize>> = HashMap::new();
        for i in self.neurons.keys()
        {
            neighbours.insert(*i, vec![]);
        }
        for link in self.links.iter()
        {
            neighbours.get_mut(&link.from).unwrap().push(link.to);
        }
        // println!("{:?}", neighbours);

        //do dfs on all nodes, ordered by number of neighbours
        let mut visit_order : Vec<usize>= self.neurons.keys().map(|x| *x).collect::<Vec<usize>>();
        visit_order.sort_by(|a, b| (neighbours.get(a).unwrap().len() as i32).cmp(&(neighbours.get(b).unwrap().len() as i32)));
        // println!("{:?}", visit_order);

        let mut compute_order : Vec<usize> = vec![];
        let mut visited : HashSet<usize> = HashSet::new();
        
        //do dfs
        for i in visit_order
        {
            // println!("start {i}");
            let mut q : Vec<usize> =  vec![i];
            let mut stack : Vec<usize>= vec![];
            while !q.is_empty()
            {
                let current : usize = *q.first().unwrap();
                q.remove(0);
                if visited.contains(&current)
                {
                    continue;
                }
                // println!("{current}");
                visited.insert(current);
                stack.insert(0, current);

                for j in neighbours.get(&current).unwrap()
                {
                    if !visited.contains(j)
                    {
                        q.insert(0, *j);
                    }
                }
            }
            compute_order.append(&mut stack);
        }
        compute_order.reverse();
        // println!("{:?}", compute_order);
        return compute_order;
    }

    pub fn calculate_output(&mut self) 
    {
        for neuron in self.neurons.values_mut().filter(|neuron| neuron.kind == NeuronType::Output)
        {
            neuron.activation = 0.0;
        }
        let compute_order : Vec<usize> = self.topological_sort();
        for i in compute_order
        {
            let neuron = self.neurons.get(&i).unwrap();
            if neuron.kind != NeuronType::Input
            {
                let weighted_sum : f32 = self.links.iter().filter(|x| x.to == i && x.enabled).fold(0.0, |acc, x| acc + x.weight * self.get_neuron(x.from).activation);
                self.get_neuron_mut(i).activation = sigmoid(weighted_sum + neuron.bias);
            }
        }
    }

    pub fn add_link(&mut self, innovation_count : &mut usize, change_map : &mut HashMap<(Mutation, usize, usize), usize>)
    {
        //pick an input and output
        let possible_inputs : Vec<usize> = self.neurons.iter()
            .filter(|(id, neuron)| [NeuronType::Input, NeuronType::Hidden].contains(&neuron.kind))
            .map(|(id, neuron)| *id)
            .collect::<Vec<usize>>();

        if possible_inputs.len() == 0
        {
            return;
        }
        
        let input_id : usize = *possible_inputs.choose(&mut rand::rng()).unwrap();

        let possible_outputs : Vec<usize> = self.neurons.iter()
            .filter(|(id, neuron)| [NeuronType::Output, NeuronType::Hidden].contains(&neuron.kind))
            .map(|(id, neuron)| *id)
            .collect::<Vec<usize>>();

        if possible_outputs.len() == 0
        {
            return;
        }
        
        let output_id : usize= *possible_outputs.choose(&mut rand::rng()).unwrap();

        //if a link between these two already exists, terminate
        if self.links.iter().any(|link| link.from == input_id && link.from == output_id)
        {
            self.links.iter_mut().find(|link| link.from == input_id && link.from == output_id).unwrap().enabled = true;
            return;
        }

        if self.cycle(input_id, output_id)
        {
            return;
        }

        let new_link_id : usize;
        if change_map.contains_key(&(Mutation::add_link, input_id, output_id))
        {
            new_link_id = *change_map.get(&(Mutation::add_link, input_id, output_id)).unwrap();
        }
        else
        {
            new_link_id = *innovation_count;
            change_map.insert((Mutation::add_link, input_id, output_id), *innovation_count);
            *innovation_count += 1;
        }

        let mut link = Link {from : input_id, to : output_id, id : new_link_id, ..Default::default()};
        link.set_random_weight();
        self.links.push(link);
    }

    pub fn cycle(&self, from_id : usize, to_id : usize) -> bool
    {
        //start from the links to point and try to make it to the links from point through dfs, if you can there's a loop
        let mut q : Vec<usize> = self.links.iter().filter(|link| link.from == to_id).map(|link| link.to).collect::<Vec<usize>>();
        let mut visited : HashSet<usize> = HashSet::new();
        while !q.is_empty()
        {
            let current : usize = *q.first().unwrap();
            q.remove(0);
            if current == from_id
            {
                return true;
            }
            //add nodes you can go to from the current node to the front of the queue
            q.append(&mut self.links.iter().filter(|link| link.from == current && !visited.contains(&link.to)).map(|link| link.to).collect::<Vec<usize>>());
            visited.insert(current);
        }
        return false;
    }

    pub fn remove_link(&mut self)
    {
        if self.links.len() == 0
        {
            return;
        }
        self.links.remove(random_range(0..self.links.len()));
    }
    
    pub fn add_hidden_neuron(&mut self, innovation_count : &mut usize, change_map : &mut HashMap<(Mutation, usize, usize), usize>)
    {
        let mut possible_links = self.links.iter_mut().filter(|link| link.enabled).collect::<Vec<&mut Link>>();
        if possible_links.len() == 0
        {
            return;
        }


        //disable old link
        let old_link =  possible_links.choose_mut(&mut rand::rng()).unwrap();
        old_link.enabled = false;
        

        //create new neuron and 2 new links
        let new_neuron_id : usize;
        if change_map.contains_key(&(Mutation::add_neuron, old_link.from, old_link.to))
        {
            new_neuron_id = *change_map.get(&(Mutation::add_neuron, old_link.from, old_link.to)).unwrap();
        }
        else
        {
            new_neuron_id = *innovation_count;
            change_map.insert((Mutation::add_neuron, old_link.from, old_link.to), *innovation_count);
            *innovation_count += 3;
        }
        
        self.neurons.insert(new_neuron_id, Neuron {id : new_neuron_id, activation : 0.5, kind : NeuronType::Hidden, ..Default::default()});
        
        //first link has weight of 1 and second has old weight to preserve how the network works
        let from_id : usize = old_link.from;
        let to_id : usize = old_link.to;
        let old_weight : f32 = old_link.weight;
        self.links.push(Link {from :from_id, to : new_neuron_id, weight : 1.0, id : new_neuron_id + 1, ..Default::default()});
        self.links.push(Link {from :new_neuron_id, to : to_id, weight : old_weight, id : new_neuron_id + 2, ..Default::default()});
    }

    pub fn remove_hidden_neuron(&mut self)
    {
        //remove neuron itself
        let possible_inputs : Vec<usize> = self.neurons.iter()
            .filter(|(id, neuron)| neuron.kind == NeuronType::Hidden)
            .map(|(id, neuron)| *id)
            .collect::<Vec<usize>>();

        if possible_inputs.len() == 0
        {
            return;
        }
        
        let id : usize = *possible_inputs.choose(&mut rand::rng()).unwrap();
        self.neurons.remove(&id);

        //remove all associated links
        self.links.retain(|link| link.from != id && link.to != id);
    }


    pub fn get_neuron(&self, i : usize) -> &Neuron
    {
        return self.neurons.get(&i).unwrap();
    }

    pub fn get_neuron_mut(&mut self, i : usize) -> &mut Neuron
    {
        if self.neurons.get_mut(&i).is_none()
        {
            println!("PROBLEM ON {i}");
            self.draw();
        }
        return self.neurons.get_mut(&i).unwrap();
    }

    pub fn reset_link_weight(&mut self)
    {
        if self.links.len() == 0
        {
            return;
        }
        let link = self.links.choose_mut(&mut rand::rng()).unwrap();
        link.set_random_weight();
    }

    pub fn nudge_link(&mut self)
    {
        if self.links.len() == 0
        {
            return;
        }
        let link = self.links.choose_mut(&mut rand::rng()).unwrap();
        link.nudge_link();
    }

    pub fn mutate(&mut self, innovation_count : &mut usize, change_map : &mut HashMap<(Mutation, usize, usize), usize>)
    {
        let prob_sum : f32 = Config::mutation_probabilities.iter().map(|(muta, prob)| prob).sum::<f32>();
        let mut pick : f32 = random_range(0.0..prob_sum);

        for (muta, prob) in Config::mutation_probabilities
        {
            pick -= prob;
            if pick <= 0.0
            {
                match muta 
                {
                    Mutation::add_link => self.add_link(innovation_count, change_map),
                    Mutation::remove_link => self.remove_link(),
                    Mutation::add_neuron => self.add_hidden_neuron(innovation_count, change_map),
                    Mutation::remove_neuron => self.remove_hidden_neuron(),
                    Mutation::reset_link => self.reset_link_weight(),
                    Mutation::nudge_link  => self.nudge_link(),
                    Mutation::toggle_link => self.toggle_link(),
                    Mutation::none  => (),
                }
                return;
            }
        }

    }

    pub fn toggle_link(&mut self)
    {
        if self.links.len() == 0
        {
            return;
        }
        let link = self.links.choose_mut(&mut rand::rng()).unwrap();
        link.enabled = !link.enabled
    }


    pub fn set_up_intial_links(&mut self)
    {
        let mut innovation_count = self.neurons.len();
        for i in self.neurons.iter().filter(|(id, neuron)| neuron.kind == NeuronType::Input).map(|(id, neuron)| id)
        {
            for j in self.neurons.iter().filter(|(id, neuron)| neuron.kind == NeuronType::Output).map(|(id, neuron)| id)
            {
                let link = Link {from : *i, to : *j, id : innovation_count, ..Default::default()};
                innovation_count += 1;
                self.links.push(link);
            }
        }
    }

    pub fn randomise_all_link_weights(&mut self)
    {
        for i in 0..self.links.len()
        {
            self.links[i].set_random_weight();
        }
    }

    pub fn draw(&self)
    {
        let mut g : Graph<String, String>= Graph::new();
        let mut nodemap : HashMap<usize, NodeIndex> = HashMap::new();
        for (k,v) in &self.neurons
        {
            let id : usize = *k;
            let idstr = id.to_string();
            let node : NodeIndex = g.add_node(idstr);
            nodemap.insert(id, node);
        }

        for link in &self.links
        {
            let from : NodeIndex = *nodemap.get(&link.from).unwrap();
            let to : NodeIndex = *nodemap.get(&link.to).unwrap();
            if link.enabled
            {
                g.add_edge(from, to, link.weight.to_string());
            }
            else
            {
                g.add_edge(from, to, link.weight.to_string() + " disabled");
            }
        }


        println!("{}", Dot::new(&g));
    }

    // pub fn crossover(&self, other : &Network, net_count : &mut usize) -> Network
    // {
    //     let mut offspring : Network = Network::new(*net_count);
    //     let (dominant, recessive) : (&Network, &Network)= if self.fitness.unwrap_or_default() > other.fitness.unwrap_or_default()  {
    //         (self, other)
    //     } else {
    //         (other, self)
    //     };

    //     for neuron_id in dominant.neurons.keys()
    //     {
    //         let dominant_neuron = dominant.get_neuron(*neuron_id);
    //         if recessive.neurons.contains_key(neuron_id)
    //         {
    //             let recessive_neuron = dominant.get_neuron(*neuron_id);
    //             let mut offspring_neuron = dominant_neuron.clone();
    //             offspring_neuron.activation = 0.0;
    //             offspring_neuron.bias = *vec![dominant_neuron.bias, recessive_neuron.bias].choose(&mut rand::rng()).unwrap();
    //             offspring.neurons.insert(*neuron_id, offspring_neuron);
    //             //TODO choose random activation function
    //         }
    //         else
    //         {
    //             let mut offspring_neuron = dominant_neuron.clone();
    //             offspring_neuron.activation = 0.0;
    //             offspring.neurons.insert(*neuron_id, offspring_neuron);
    //         }
    //     }

    //     for dominant_link in dominant.links.clone()
    //     {
    //         let recessive_link = recessive.links.iter().find(|link| link.from == dominant_link.from && link.to == dominant_link.to);
    //         if recessive_link.is_some()
    //         {
    //             let mut offspring_link = dominant_link.clone();
    //             offspring_link.weight = *vec![dominant_link.weight, recessive_link.unwrap().weight].choose(&mut rand::rng()).unwrap();
    //             offspring_link.enabled = *vec![dominant_link.enabled, recessive_link.unwrap().enabled].choose(&mut rand::rng()).unwrap();
    //             offspring.links.push(offspring_link);
    //         }
    //         else
    //         {
    //             offspring.links.push(dominant_link.clone());
    //         }
    //     }

    //     return offspring;
    // }

    pub fn crossover(&self, other : &Network, net_count :&mut usize) -> Network
    {
        if self.id == other.id
        {
            return self.clone();
        }

        let (dominant, recessive) : (&Network, &Network)= if self.fitness.unwrap_or_default() > other.fitness.unwrap_or_default()  {
            (self, other)
        } else {
            (other, self)
        };

        let mut offspring : Network = Network::new(*net_count);
        *net_count += 1;

        let mut neurons_used : HashSet<usize> = HashSet::new();

        //TODO maybe use hashsets instead and instersections like in get_genetic_distance
        let all_innovation_numbers: Vec<usize> = dominant.links
            .iter()
            .chain(recessive.links.iter())
            .map(|link| link.id)
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect::<Vec<usize>>();   

        for id in all_innovation_numbers
        {
            match (dominant.get_link(id), recessive.get_link(id))
            {
                (Some(d), Some(r)) => {
                    let mut new_link = if random_range(0..=1) == 0 {
                        d.clone()
                    } else {
                        r.clone()
                    };
                    
                    if d.enabled != r.enabled
                    {
                        new_link.enabled = random_range(0.0..=1.0) <= 0.25;
                    }

                    if !Config::randomly_choose_matching_genes
                    {
                        new_link.weight = (d.weight + r.weight) / 2.0
                    }
                    
                    offspring.links.push(new_link);
                    neurons_used.insert(new_link.from);
                    neurons_used.insert(new_link.to);

                },

                (Some(d), None) => {
                    offspring.links.push(d.clone());
                    neurons_used.insert(d.from);
                    neurons_used.insert(d.to);
                },

                (None, Some(r)) => {
                    if dominant.fitness.unwrap_or_default().round() == recessive.fitness.unwrap_or_default().round()
                    {
                        offspring.links.push(r.clone());
                        neurons_used.insert(r.from);
                        neurons_used.insert(r.to);
                    }
                },

                (None, None) => ()
            }
        }


        //make sure input and output nodes always end up in the offspring even if they have no links to or from them
        for i in 0..(Config::input_count + Config::output_count)
        {
            neurons_used.insert(i);
        }

        
        for i in neurons_used
        {
            let neuron = if dominant.neurons.contains_key(&i) {
                dominant.get_neuron(i).clone()
            } else {
                recessive.get_neuron(i).clone()
            };
            
            offspring.neurons.insert(i, neuron);
        }
        return offspring;
    }

    pub fn get_genetic_distance(&self, other : &Network) -> f32
    {
        let self_genes : HashSet<usize> = self.links
            .iter()
            .map(|link| link.id)
            .collect::<HashSet<usize>>();

        let other_genes : HashSet<usize> = other.links
            .iter()
            .map(|link| link.id)
            .collect::<HashSet<usize>>();
        
        let shared_genes : HashSet<usize>= self_genes.intersection(&other_genes).copied().collect();

        let max_self  = self_genes.iter().max().copied().unwrap_or(0);
        let max_other = other_genes.iter().max().copied().unwrap_or(0);
        let cutoff    = max_self.min(max_other);

        let all_genes : HashSet<usize>= self_genes.union(&other_genes).copied().collect();

        let mut E : f32 = 0.0;
        let mut D : f32 = 0.0;
        let mut total_weight_diff : f32 = 0.0;
        
        for id in all_genes
        {
            if shared_genes.contains(&id)
            {
                total_weight_diff += (self.get_link(id).unwrap().weight - other.get_link(id).unwrap().weight).abs();
            }
            else
            {
                if id <= cutoff
                {
                    D += 1.0;
                }
                else
                {
                    E += 1.0;
                }
            }
        }

        let N : f32= max(max(self_genes.len(), other_genes.len()), 1) as f32;

        let avg_weight_diff = if shared_genes.is_empty() {
            0.0
        } else {
            total_weight_diff / shared_genes.len() as f32
        };

        let delta = Config::cE * E / N + Config::cD * D / N + Config::cW * avg_weight_diff;

        return delta;
    }

    pub fn get_link(&self, id : usize) -> Option<&Link>
    {
        return self.links.iter().find(|x| x.id == id);
    }

    pub fn set_inputs(&mut self, inputs : Vec<f32>)
    {
        for i in 0..Config::input_count
        {
            self.get_neuron_mut(i).activation = inputs[i];
        }
    }

    pub fn get_outputs(&mut self) -> Vec<f32>
    {
        let mut outputs : Vec<f32> = vec![];
        for i in Config::input_count..(Config::input_count + Config::output_count)
        {
            outputs.push(self.get_neuron_mut(i).activation);
        }

        return outputs;
    }

    pub fn new(id : usize) -> Network
    {
        return Network {neurons : HashMap::new(), links : vec![], fitness : None, id : id}
    }
}

pub fn sigmoid(x : f32) -> f32
{
    return 1.0 / (1.0 + (-x).exp());
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Mutation 
{
    add_neuron,
    remove_neuron,
    add_link,
    remove_link,
    reset_link,
    nudge_link,
    toggle_link,
    none,
}