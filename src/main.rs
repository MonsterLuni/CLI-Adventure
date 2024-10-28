mod enemy;
mod player;
mod point;

use text_io::{read};
use crate::enemy::{Enemy};
use crate::player::Player;
use crate::point::Point;

struct Map{
    width:u8,
    height:u8
}
fn main(){
    let mut player = Player{lives: 3,position: Point{x:0,y:0}};
    let mut running:bool = true;

    let mut enemies:Vec<Enemy> = vec![Enemy { alive: true, damage: 1, position: Point { x: 8, y: 5} },Enemy { alive: true, damage: 1, position: Point { x: 7, y: 2 } }];
    while running {
        let map = Map { width: 10, height: 10 };
        let mut i = 0;

        while i < enemies.len(){
            if enemies[i].alive {
                enemies[i].update(&map);
            }else{
                enemies.remove(i);
            }
            i += 1;
        }
        update_map(&map,&mut player,&enemies);
        update_infos(&map,&player,&enemies);
        let input:String = read!();
        for enemy in &enemies{
            if player.position.x == enemy.position.x && player.position.y == enemy.position.y {
                if player.lives - enemy.damage <= 0{
                    println!("You dead")
                }else{
                    player.lives -= enemy.damage
                }
            }
        }
        match input.as_str() {
            "exit" => {running = false},
            "w" => player.position.move_entity(input,&map),
            "a" => player.position.move_entity(input,&map),
            "s" => player.position.move_entity(input,&map),
            "d" => player.position.move_entity(input,&map),
            _ => println!("{}", input)
        }
    };
}
fn update_map(map: &Map, player: &mut Player, enemies:&[Enemy]){
    let mut x = 0;
    let mut y = 0;
    let mut enemy_drawn:bool;
    while x < map.height {
        println!(" ");
        while y < map.width{
            enemy_drawn = false;
            for enemy in enemies{
                if enemy.alive {
                    if x == enemy.position.x && y == enemy.position.y {
                        print!(" x ");
                        enemy_drawn = true;
                    }
                }
            }
            if !enemy_drawn {
                if x == player.position.x && y == player.position.y {
                    print!(" o ");
                }else{
                    print!(" - ");
                }
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }
}
fn update_infos(map: &Map, player: &Player, enemies:&Vec<Enemy>){
    let mut i = 0;
    println!(" ");
    while i < map.width * 3{
        print!("-");
        i += 1;
    }
    println!(" ");
    println!("You have {:?} live points left!",player.lives);
    println!("There are currently {:?} Enemies alive!",enemies.len());
}