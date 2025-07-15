use raylib::prelude::RaylibDrawHandle;

use crate::agent::Agent;
use crate::grid::Grid;
use crate::network::Network;

pub struct Runner 
{
    pub agent : Agent,
    pub grid : Grid,
}

impl Runner 
{
    pub fn new(network : Network) -> Runner
    {
        return Runner {agent : Agent::new(network), grid : Grid::new()};
    }

    pub fn step(&mut self)
    {
        if !self.grid.running {
            return;
        }
        let inputs = self.grid.get_inputs();
        let desire = self.agent.get_desire_from_inputs(inputs);
        self.grid.step(desire);
    }

    pub fn draw(&mut self, d : &mut RaylibDrawHandle)
    {
        self.grid.draw(d);
    }
}