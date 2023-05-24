use std::collections::LinkedList;

pub struct Entity {
    name: String,
    health: i32,
    mana: i32,
    light_attack_dmg: i32,
    heavy_attack_dmg: i32,
    att_buff: f32,
    def_buff: f32,
    speed: i32,
    spells: LinkedList<Spell>,
}


impl Entity {
    pub fn new(name: String) -> Entity {
        Entity {
            name,
            health: 100,
            mana: 100,
            light_attack_dmg: 10,
            heavy_attack_dmg: 20,
            att_buff: 1.0,
            def_buff: 1.0,
            speed: 10,
            spells: LinkedList::new(),
        }
    }
    pub fn hit_entity(self: &Self, enemy: Entity) -> Entity {
        Entity {
            health: enemy.health - self.light_attack_dmg,
            ..enemy
        }
    }
    pub fn print_stats(self: &Self) {
        println!("");
        println!("{}", self.name);
        println!("Health: {}", self.health);
        println!("Mana: {}", self.mana);
        println!("Att Buff: {}", self.att_buff);
        println!("Def Buff: {}", self.def_buff);
        println!("Speed: {}", self.speed);
    }
}

pub struct Spell {
    name: String,
    dmg: i32,
    cost: i32,
    att_buff: f32,
    def_buff: f32,
}
