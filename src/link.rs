use crate::config::Config;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Clone, Copy)]
pub struct Link 
{
    pub id : usize,
    pub from : usize,
    pub to : usize,
    pub weight : f32,
    pub enabled : bool,
}


impl Default for Link 
{
    fn default() -> Link 
    {
        return Link {id : 0, from : 0, to : 0, weight : 0.0, enabled : true}
    }
}

impl Link
{
    pub fn set_random_weight(&mut self)
    {
        let normal = Normal::new(Config::link_mean, Config::link_sigma).unwrap();
        self.weight = normal.sample(&mut rand::rng()).clamp(Config::min_link_weight, Config::max_link_weight);
    }

    pub fn nudge_link(&mut self)
    {
        let normal = Normal::new(0.0, Config::link_mutate_power).unwrap();

        self.weight += normal.sample(&mut rand::rng());
        self.weight = self.weight.clamp(Config::min_link_weight, Config::max_link_weight);
    }
}