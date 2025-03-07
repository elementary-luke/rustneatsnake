use crate::link;
use crate::link::*;
use crate::neuron::*;
use std::collections::HashMap;
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
pub struct Network 
{
    pub neurons : HashMap<usize, Neuron>,
    pub links : Vec<Link>
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
        println!("{:?}", neighbours);

        //do dfs on all nodes, ordered by number of neighbours
        let mut visit_order : Vec<usize>= self.neurons.keys().map(|x| *x).collect::<Vec<usize>>();
        visit_order.sort_by(|a, b| (neighbours.get(a).unwrap().len() as i32).cmp(&(neighbours.get(b).unwrap().len() as i32)));
        println!("{:?}", visit_order);

        let mut compute_order : Vec<usize> = vec![];
        let mut visited : Vec<bool> = vec![false; visit_order.len()];

        //do dfs
        for i in visit_order
        {
            println!("start {i}");
            let mut q : Vec<usize> =  vec![i];
            let mut stack : Vec<usize>= vec![];
            while !q.is_empty()
            {
                let current : usize = *q.first().unwrap();
                q.remove(0);
                if visited[current]
                {
                    continue;
                }
                println!("{current}");
                visited[current] = true;
                stack.insert(0, current);

                for j in neighbours.get(&current).unwrap()
                {
                    if !visited[*j]
                    {
                        q.insert(0, *j);
                    }
                }
            }
            compute_order.append(&mut stack);
        }
        compute_order.reverse();
        println!("{:?}", compute_order);
        return compute_order;
    }

    pub fn calculate_output(&mut self) 
    {
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

    pub fn add_link(&mut self)
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
        self.links.push(Link {from : input_id, to : output_id, weight : random_range(-1.0..=1.0), ..Default::default()}) 
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
            *innovation_count += 1;
        }
        
        self.neurons.insert(new_neuron_id, Neuron {id : new_neuron_id, activation : 0.5, kind : NeuronType::Hidden, ..Default::default()});
        println!("{:?}",self.neurons.keys());
        
        let from_id : usize = old_link.from;
        let to_id : usize = old_link.to;
        let old_weight : f32 = old_link.weight;
        self.links.push(Link {from :from_id, to : new_neuron_id, weight : 1.0, ..Default::default()});
        self.links.push(Link {from :new_neuron_id, to : to_id, weight : old_weight, ..Default::default()});
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
            if prob <= 0.0
            {
                match muta 
                {
                    Mutation::add_link => self.add_link(),
                    Mutation::remove_link => self.remove_link(),
                    Mutation::add_neuron => self.add_hidden_neuron(innovation_count, change_map),
                    Mutation::remove_neuron => self.remove_hidden_neuron(),
                    Mutation::reset_link => self.reset_link_weight(),
                    Mutation::nudge_link  => self.nudge_link(),
                }
                return;
            }
        }

    }

    //TODO toggle link 

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
}

impl Default for Network 
{
    fn default() -> Network 
    {
        Network {neurons : HashMap::new(), links : vec![]}
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
}