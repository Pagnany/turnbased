pub struct Entity{
    name: String,
    health: i32,
    light_attack_dmg: i32,
    heavy_attack_dmg: i32,
    speed: i32,
}

impl Entity {
    pub fn new (name: String) -> Entity{
        Entity { 
            name: name, 
            health: 100, 
            light_attack_dmg: 10, 
            heavy_attack_dmg: 20, 
            speed: 10 
        }
    }
    pub fn get_name (self: &Self) -> String {
        self.name.clone().to_string()
    }
    
    pub fn hit_entity (self: &Self, enemy: Entity) -> Entity{
        Entity { 
            health: enemy.health - self.light_attack_dmg, 
            ..enemy
         }
    }
    pub fn get_health (self: &Self) -> i32 {
        self.health
    }
}