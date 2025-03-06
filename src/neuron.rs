#[derive(PartialEq, PartialOrd)]
pub enum NeuronType 
{
    Input,
    Hidden,
    Output,
}
pub struct Neuron 
{
    pub id : usize,
    pub activation : f32,
    pub bias : f32,
    pub kind : NeuronType,
}


impl Default for Neuron 
{
    fn default() -> Neuron 
    {
        return Neuron {id : 0, activation : 0.0, bias : 0.0, kind : NeuronType::Hidden}
    }
}