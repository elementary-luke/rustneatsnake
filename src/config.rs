//to use raylib, installed cmake 3.31 and most recent llvm. MAKE SURE PATH VARS ADDED IN INSTALLATION

use crate::network::*;

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
    pub const input_count : usize = 30;
    pub const output_count : usize = 4;

    //population
    pub const survival_percentage : f32 = 0.4;
    pub const population_size : usize = 100;
    pub const cull_method : i16 = 1; //0:top x% survive, 1: higher fitness means better chance to survive, 2: tournament
    pub const force_reevalutaion : bool = true; // if true all networks will be evaluated, even if they were evaluated in a previous generation
    pub const global_change_map : bool = true; // if true, the same mutation in another generation will have the same innovation_number


    pub const advanced_crossover : bool = true; //if advanced, it follows the NEAT sepcification, if not we just add everything from dominant and if theres a match then pick one
    pub const randomly_choose_matching_genes : bool = true; // if true randomly choose gene when genes match, otherwise average

    pub const cE : f32 = 1.0;
    pub const cD : f32 = 1.0;
    pub const cW : f32 = 0.4;
    pub const delta_t : f32 = 3.0;


    //INPUTS
    //0-7     : fruit
    //8-15    : walls
    //16-23   : body
    //24-27   : direction
    //28      : length
    //29      : bias


    //game
    pub const grid_width : usize = 20;
    pub const grid_height : usize = 20;

    pub const cell_width : i16 = 30; // size of cell displayed

    pub const fruit_value : f32 = 1000.0;
    pub const step_value : f32 = 1.0;

    //agent
    pub const num_simulations : usize = 5;

    //relative probabilites, don't necessarily have to add up to 1
    pub const mutation_probabilities : [(Mutation, f32); 8] = [
        (Mutation::add_link, 0.15),
        (Mutation::remove_link, 0.1),
        (Mutation::add_neuron, 0.1),
        (Mutation::remove_neuron, 0.05),
        (Mutation::reset_link, 0.1),
        (Mutation::nudge_link, 0.37),
        (Mutation::toggle_link, 0.03),
        (Mutation::none, 0.1),
    ];
}