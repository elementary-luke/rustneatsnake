use crate::grid::Grid;
use crate::network::Network;


pub struct Agent 
{
    pub net : Network,
    pub grid : Grid
}

impl Agent 
{
    pub fn new(network : Network) -> Agent
    {
        return Agent {net : network, grid : Grid::new()};
    }
}