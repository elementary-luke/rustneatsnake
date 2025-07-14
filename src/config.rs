//to use raylib, installed cmake 3.31 and most recent llvm. MAKE SURE PATH VARS ADDED IN INSTALLATION

use crate::network::*;
use strum_macros::EnumIter;

pub struct Config
{

}

impl Config
{
    //network
    pub const link_has_id :bool = false;
    pub const link_mean : f32 = 0.0;
    pub const link_sigma : f32 = 1.0;
    pub const min_link_weight : f32 = -1.0;
    pub const max_link_weight : f32 = 1.0;
    pub const link_mutate_power : f32 = 1.2;
    pub const input_count : usize = 8;
    pub const output_count : usize = 4;

    //  up_fruit_index = 0;
    //  down_fruit_index = 1;
    //  left_fruit_index = 2;
    //  right_fruit_index = 3;
    //  up_wall_index = 4;
    //  down_wall_index = 5;
    //  left_wall_index = 6;
    //  right_wall_index = 7;

    //game
    pub const grid_width : usize = 20;
    pub const grid_height : usize = 20;

    pub const cell_width : i16 = 30; // size of cell displayed

    pub const fruit_value : f32 = 1000.0;
    pub const step_value : f32 = 1.0;

    //agent
    pub const num_simulations : usize= 5;

    //relative probabilites, don't necessarily have to add up to 1
    pub const mutation_probabilities : [(Mutation, f32); 7] = [
        (Mutation::add_link, 0.15),
        (Mutation::remove_link, 0.1),
        (Mutation::add_neuron, 0.1),
        (Mutation::remove_neuron, 0.05),
        (Mutation::reset_link, 0.1),
        (Mutation::nudge_link, 0.4),
        (Mutation::none, 0.1),
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