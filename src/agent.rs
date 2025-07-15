use core::f32;

use crate::grid::Grid;
use crate::network::Network;
use crate::config::Config;
use crate::vec2::Vec2i;


pub struct Agent 
{
    pub net : Network,
}

impl Agent 
{
    pub fn new(network : Network) -> Agent
    {
        return Agent {net : network};
    }

    pub fn get_desire_from_inputs(&mut self, inputs : Vec<f32>) -> Vec2i
    {
        self.net.set_inputs(inputs);

        self.net.calculate_output();

        let outputs = self.net.get_outputs();

        return self.outputs_to_desire(outputs);
    }

    pub fn evaluate(&mut self) -> f32 {
        let mut fitness : f32 = 0.0;
        for i in 0..Config::num_simulations
        {
            let mut grid = Grid::new();
            while grid.running
            {
                let desire = self.get_desire_from_inputs(grid.get_inputs());
                grid.step(desire);
            }
            fitness += grid.calculate_fitness();
        }

        return fitness / Config::num_simulations as f32;
    }

    pub fn outputs_to_desire(&self, outputs : Vec<f32>) -> Vec2i
    {
        let mut winner_index : i16 = -1;
        let mut highest_output : f32 = f32::MIN;
        
        for (i, val) in outputs.iter().enumerate()
        {
            if *val > highest_output
            {
                highest_output = *val;
                winner_index = i as i16;
            }
        }
        
        return match winner_index {
            0 => Vec2i::from((0, -1)),
            1 => Vec2i::from((0, 1)),
            2 => Vec2i::from((-1, 0)),
            3 => Vec2i::from((1, 0)),
            _ => Vec2i::from((0, 0))
        }
    }
}