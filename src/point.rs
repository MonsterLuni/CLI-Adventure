use crate::Map;

pub struct Point {
    pub(crate) x:u8,
    pub(crate) y:u8
}
impl Point{
    pub(crate) fn check_move_entity(&mut self, input:String, map:&Map) -> bool{
        match input.as_str(){
            "w" => self.check_if_move_is_possible("x",-1,&map),
            "a" => self.check_if_move_is_possible("y",-1,&map),
            "s" => self.check_if_move_is_possible("x",1,&map),
            "d" => self.check_if_move_is_possible("y",1,&map),
            _ => false
        }
    }
    pub(crate) fn move_entity(&mut self, input:String, map:&Map){
        match input.as_str(){
            "w" => if self.check_if_move_is_possible("x",-1,&map) {self.x -= 1},
            "a" => if self.check_if_move_is_possible("y",-1,&map) {self.y -= 1},
            "s" => if self.check_if_move_is_possible("x",1,&map) {self.x += 1},
            "d" => if self.check_if_move_is_possible("y",1,&map) {self.y += 1},
            _ => {}
        }
    }
    pub(crate) fn check_if_move_is_possible(&mut self,direction:&str,movement:i8,map:&Map) -> bool{
        if direction == "x"{
            if self.x as i8 + movement >= 0 && self.x as i8 + movement <= (map.width - 1) as i8 {
                true
            }else{
                false
            }
        }else{
            if self.y as i8 + movement >= 0 && self.y as i8 + movement <= (map.height - 1) as i8 {
                true
            }else{
                false
            }
        }
    }
}