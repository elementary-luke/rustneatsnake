//to use raylib, installed cmake 3.31 and most recent llvm. MAKE SURE PATH VARS ADDED IN INSTALLATION

use crate::network::*;

pub struct Config
{

}

impl Config
{
    //run settings
    pub const autorun : bool = true; // if false, you will have to press space to see the snake move 1 frame
    pub const fps : u32 = 30;

    //species
    pub const use_species : bool = true;
    pub const shuffle_species_order : bool = true; // whether the order of the species are shuffled every generation to stop species higher up from alsways getting networks that couldve also been in other species
    pub const best_as_representative : bool = true; //if false representative is randomly picked for the next generation
    pub const target_num_species : usize = 20;
    pub const stagnation_threshold : usize = 25; // if a species is stagnant for over x generations remove it
    pub const stagnation_factor : f32 = 1.00; // how much better the fitness has to be compared to the current species best to reset stagnation counter
    pub const species_elitistm_num : usize = 1;
    pub const global_elitistm_num : usize = 0; //how many of the best per generation to keep unchanged
    pub const mutate_elites : bool = false; // whether elites should be mutated or should just go into the next generation untouched
    pub const mut_not_cross_prob : f32 = 0.25; // probability that an offspring is a mutated clone not a crossover

    //network
    pub const num_start_links : usize = 1;
    pub const link_mean : f32 = 0.0;
    pub const link_sigma : f32 = 1.0;
    pub const min_link_weight : f32 = -5.0;
    pub const max_link_weight : f32 = 5.0;
    pub const link_mutate_power : f32 = 0.2;
    pub const input_count : usize = 17;
    pub const output_count : usize = 4;
    pub const print_disabled : bool = false; // whether to show disabled links in the final digraph

    //population
    pub const survival_percentage : f32 = 0.4; // only if speciation is off
    pub const population_size : usize = 100;
    pub const cull_method : i16 = 1; // only inportant when species off 0:top x% survive, 1: higher fitness means better chance to survive, 2: tournament
    pub const force_reevalutaion : bool = true; // if true all networks will be evaluated, even if they were evaluated in a previous generation
    pub const global_change_map : bool = true; // if true, the same mutation in another generation will have the same innovation_number


    pub const advanced_crossover : bool = true; //if advanced, it follows the NEAT sepcification, if not we just add everything from dominant and if theres a match then pick one
    pub const randomly_choose_matching_genes : bool = true; // if true randomly choose gene when genes match, otherwise average
    pub const link_disable_probability : f32 = 0.75; // chance that if 1 at least matching gene is disabled, the gene in the offspring is too

    pub const cE : f32 = 1.0;
    pub const cD : f32 = 1.0;
    pub const cW : f32 = 0.4;
    pub const base_delta_t : f32 = 3.0; // max genetic distance for 2 networks to be in the same specie at the start
    pub const delta_t_adjustment : f32 = 0.5; // set to 0.0 to disable
    pub const min_delta_t : f32 = 0.0;
    pub const max_delta_t : f32 = 15.0;


    //INPUTS
    //0-3     : fruit
    //4-11    : walls
    //12-19   : body
    //20-23   : direction
    //24      : length
    //25      : bias

    //26 up
    //27 down
    //28 left
    //29right

    //game
    pub const grid_width : usize = 15;
    pub const grid_height : usize = 15;

    pub const cell_width : i16 = 30; // size of cell displayed

    pub const fruit_value : f32 = 1000.0;
    pub const step_value : f32 = 1.0;

    //fitness evaluation
    pub const use_multithreading : bool = true;
    pub const num_simulations : usize = 20;

    //relative probabilites, don't necessarily have to add up to 1
    // pub const mutation_probabilities : [(Mutation, f32); 8] = [
    //     (Mutation::add_link, 0.15),
    //     (Mutation::remove_link, 0.1),
    //     (Mutation::add_neuron, 0.05),
    //     (Mutation::remove_neuron, 0.05),
    //     (Mutation::reset_link, 0.1),
    //     (Mutation::nudge_link, 0.7),
    //     (Mutation::toggle_link, 0.03),
    //     (Mutation::none, 0.1),
    // ];
    pub const mutation_probabilities: [(Mutation, f32); 9] = [
        (Mutation::add_link,     0.16),
        (Mutation::remove_link,  0.00),
        (Mutation::add_neuron,   0.06),
        (Mutation::remove_neuron, 0.00),
        (Mutation::reset_link,   0.03),
        (Mutation::nudge_link,   0.80),
        (Mutation::enable_link,  0.1),
        (Mutation::disable_link,  0.02),
        (Mutation::none,         0.025),
    ];
//     pub const mutation_probabilities: [(Mutation, f32); 8] = [
//     (Mutation::add_link,     0.20), 
//     (Mutation::remove_link,  0.02),
//     (Mutation::add_neuron,   0.08),
//     (Mutation::remove_neuron, 0.01),
//     (Mutation::reset_link,   0.05),
//     (Mutation::nudge_link,   0.60),
//     (Mutation::toggle_link,  0.02),
//     (Mutation::none,         0.02),
// ];
}