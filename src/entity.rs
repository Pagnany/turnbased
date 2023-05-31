use rusqlite::Connection;

pub fn get_spells_from_db() -> Vec<Spell> {
    let conn = Connection::open("./db/turnbased.db").unwrap();
    let mut stmt = conn.prepare("select * from spells;").unwrap();

    let mut vec_spells = Vec::<Spell>::new();

    let spells = stmt
        .query_map([], |row| {
            Ok(Spell {
                name: row.get(1).unwrap(),
                dmg: row.get(2).unwrap(),
                cost: row.get(3).unwrap(),
                att_buff: row.get(4).unwrap(),
                def_buff: row.get(5).unwrap(),
            })
        })
        .unwrap();
    for spe in spells {
        vec_spells.push(spe.unwrap())
    }
    vec_spells
}

pub struct Entity {
    name: String,
    health: i32,
    mana: i32,
    light_attack_dmg: i32,
    heavy_attack_dmg: i32,
    att_buff: f32,
    def_buff: f32,
    speed: i32,
    spells: Vec<Spell>,
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
            spells: get_spells_from_db(),
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

    pub fn print_spells(self: &Self) {
        println!("Spells:");
        let mut i = 1;
        for spell in &self.spells {
            println!(" {} {}", i, spell.name);
            i += 1;
        }
    }
}
#[derive(Debug)]
pub struct Spell {
    name: String,
    dmg: i32,
    cost: i32,
    att_buff: f32,
    def_buff: f32,
}
