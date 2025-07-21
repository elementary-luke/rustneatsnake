use crate::{config::Config, network::Network};
pub struct Specie 
{
    pub id : usize,
    pub age : u16,
    pub stagnant_counter : usize,
    pub best_fitness : f32,
    pub highest_best_fitness : f32,
    pub avg_fitness : f32,
    pub representative : Network, // clone not an index since might not exist next gen
    pub members : Vec<usize>, //indices of popmanagers netwoks vector
}

impl Specie
{
    pub fn new(rep : Network, id : usize) -> Specie
    {
        let fitness : f32 = rep.fitness.unwrap_or_default();
        return Specie {representative: rep, members: vec![], age: 0, stagnant_counter: 0, best_fitness : fitness, avg_fitness : fitness, id, highest_best_fitness : fitness};
    }

    pub fn clear_members(&mut self)
    {
        self.members.clear();
    }

    pub fn sort_members_by_fitness(&mut self, networks : &Vec<Network>)
    {
        self.members.sort_by(|a, b| networks[*b].fitness.partial_cmp(&networks[*a].fitness).unwrap());
    }

    pub fn set_fitness_stats(&mut self, networks : &Vec<Network>)
    {
        let mut best : f32 = 0.0;
        let mut total : f32 = 0.0;
        for i in &self.members
        {
            let fitness = networks[*i].fitness.unwrap_or_default();
            total += fitness;
            if fitness > best
            {
                best = fitness;
            }
        }

        if best > self.highest_best_fitness * Config::stagnation_factor
        {
            self.stagnant_counter = 0;
            self.highest_best_fitness = best;
        }

        self.best_fitness = best;
        self.avg_fitness =  if self.members.len() > 0 {
            total / self.members.len() as f32
        } else {
            0.0
        };
    }
}