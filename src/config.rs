use std::{collections::HashMap, sync::atomic::{AtomicI32, AtomicUsize}};
use crate::network::*;
use strum_macros::EnumIter;

pub struct Config
{

}

impl Config
{
    pub const link_has_id :bool = false;
    pub const link_mean : f32 = 0.0;
    pub const link_sigma : f32 = 1.0;
    pub const min_link_weight : f32 = -1.0;
    pub const max_link_weight : f32 = 1.0;
    pub const link_mutate_power : f32 = 1.2;
    pub const input_count : usize = 3;
    pub const output_count : usize = 3;

    //relative probabilites, don't necessarily have to add up to 1
    pub const mutation_probabilities : [(Mutation, f32); 6] = [
        (Mutation::add_link, 0.15),
        (Mutation::remove_link, 0.1),
        (Mutation::add_neuron, 0.1),
        (Mutation::remove_neuron, 0.05),
        (Mutation::reset_link, 0.4),
        (Mutation::nudge_link, 0.1),
    ];
}

#[derive(Debug, EnumIter)]

pub enum InputType
{
    A,
    B,
    C
}
#[derive(Debug, EnumIter)]
pub enum OutputType
{
    C,
    D,
    E
}