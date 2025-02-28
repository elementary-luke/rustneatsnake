use crate::link;
use crate::link::*;
use crate::neuron::*;
use std::collections::HashMap;

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

        //do dfs on all nodes, ordered by 
        let mut visit_order : Vec<usize>= self.neurons.keys().map(|x| *x).collect::<Vec<usize>>();
        visit_order.sort_by(|a, b| (neighbours.get(a).unwrap().len() as i32).cmp(&(neighbours.get(b).unwrap().len() as i32)));
        println!("{:?}", visit_order);

        let mut compute_order : Vec<usize> = vec![];
        let mut visited : Vec<bool> = vec![false; visit_order.len()];

        //do 
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
            if self.neurons.get(&i).unwrap().kind != NeuronType::Input
            {
                let weighted_sum : f32 = self.links.iter().filter(|x| x.to == i).fold(0.0, |acc, x| acc + x.weight * self.neurons.get_mut(&x.from).unwrap().activation);
                self.neurons.get_mut(&i).unwrap().activation = sigmoid(weighted_sum);
            }
        }
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