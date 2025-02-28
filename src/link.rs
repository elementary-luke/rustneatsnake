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