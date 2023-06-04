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
                att_buff_self: row.get(4).unwrap(),
                def_buff_self: row.get(5).unwrap(),
                att_buff_enemy: row.get(6).unwrap(),
                def_buff_enemy: row.get(7).unwrap(),
                heal: row.get(8).unwrap(),
            })
        })
        .unwrap();
    for spell in spells {
        vec_spells.push(spell.unwrap())
    }
    vec_spells
}

pub struct Entity {
    pub name: String,
    pub health: i32,
    mana: i32,
    light_attack_dmg: i32,
    heavy_attack_dmg: i32,
    att_buff: f32,
    def_buff: f32,
    speed: i32,
    pub spells: Vec<Spell>,
    skip_turns: i32,
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
            def_buff: 1.5,
            speed: 10,
            spells: get_spells_from_db(),
            skip_turns: 0,
        }
    }

    pub fn lower_att_buff(self: &mut Self, att_buff: f32) {
        self.att_buff -= att_buff;
        if self.att_buff < 0.1 {
            self.att_buff = 0.1;
        }
        // show only 2 decimal places
        self.att_buff = (self.att_buff * 100.0).round() / 100.0;
    }

    pub fn hit_entity_light(self: &Self, enemy: Entity) -> Entity {
        Entity {
            health: enemy.health
                - ((self.light_attack_dmg as f32 * self.att_buff) / enemy.def_buff as f32) as i32,
            ..enemy
        }
    }

    pub fn hit_entity_heavy(self: &Self, enemy: Entity) -> Entity {
        Entity {
            health: enemy.health
                - ((self.heavy_attack_dmg as f32 * self.att_buff) / enemy.def_buff as f32) as i32,
            ..enemy
        }
    }

    pub fn use_spell(self: &mut Self, enemy: Entity, spell_index: usize) -> Entity {
        let spell = &self.spells[spell_index];

        let temp = Entity {
            health: enemy.health
                - ((spell.dmg as f32 * self.att_buff) / enemy.def_buff as f32) as i32,
            att_buff: enemy.att_buff * spell.att_buff_enemy,
            def_buff: enemy.def_buff * spell.def_buff_enemy,
            ..enemy
        };

        self.mana -= spell.cost;
        self.att_buff *= spell.att_buff_self;
        self.def_buff *= spell.def_buff_self;
        self.health += spell.heal as i32;

        return temp;
    }

    pub fn print_stats(self: &Self) {
        println!("");
        println!("{}", self.name);
        println!(" Health: {}", self.health);
        println!(" Mana: {}", self.mana);
        println!(" Att Buff: {}", self.att_buff);
        println!(" Def Buff: {}", self.def_buff);
        println!(" Speed: {}", self.speed);
    }

    pub fn print_spells(self: &Self) {
        for (i, spell) in self.spells.iter().enumerate() {
            println!(" {} {}  (Mana: {})", i + 1, spell.name, spell.cost);
            println!("    Dmg: {}", spell.dmg);
            println!("    Att Buff Self: {}", spell.att_buff_self);
            println!("    Def Buff Self: {}", spell.def_buff_self);
            println!("    Att Buff Enemy: {}", spell.att_buff_enemy);
            println!("    Def Buff Enemy: {}", spell.def_buff_enemy);
            println!("    Heal: {}", spell.heal);
        }
    }
}

pub struct Spell {
    name: String,
    dmg: i32,
    cost: i32,
    att_buff_self: f32,
    def_buff_self: f32,
    att_buff_enemy: f32,
    def_buff_enemy: f32,
    heal: f32,
}
