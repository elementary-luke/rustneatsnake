use std::{collections::HashMap, sync::atomic::{AtomicI32, AtomicUsize}};
use crate::network::Mutation;

pub struct Config
{

}

impl Config
{
    pub const link_has_id :bool = false;
    pub const id_count : AtomicUsize = AtomicUsize::new(0);
    pub const link_mean : f32 = 0.0;
    pub const link_sigma : f32 = 1.0;
    pub const min_link_weight : f32 = 1.0;
    pub const max_link_weight : f32 = 1.0;
    pub const link_mutate_power : f32 = 1.2;

    pub const mutation_probabilities : [(Mutation, f32); 6] = [
        (Mutation::add_link, 0.3),
        (Mutation::remove_link, 0.3),
        (Mutation::add_neuron, 0.3),
        (Mutation::remove_neuron, 0.3),
        (Mutation::reset_link, 0.3),
        (Mutation::nudge_link, 0.3),
    ];
}