use crate::{Map};
use crate::point::Point;
pub struct Enemy{
    pub(crate) alive:bool,
    pub(crate) damage:u8,
    pub(crate) position:Point
}
impl Enemy{
    pub fn update(&mut self,map: &Map){
        if self.position.check_move_entity("w".to_string(), map) {
            self.position.move_entity("w".to_string(), map);
        }else{
            self.alive = false;
        }
    }
}