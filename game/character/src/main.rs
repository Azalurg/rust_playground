/*
    Simple character implementations 
*/

#[derive(Debug)]
enum Classes {
    Warrior,
    Mage,
    Archer,
}

impl Classes {
    fn get_stats(&self) -> Stats {
        match self {
            Classes::Warrior => Stats {
                strength: 12,
                agility: 5,
                intelligence: 3,
            },
            Classes::Mage => Stats {
                strength: 2,
                agility: 3,
                intelligence: 15,
            },
            Classes::Archer => Stats {
                strength: 5,
                agility: 10,
                intelligence: 5,
            },
        }
    }
}

#[derive(Debug)]
enum Races {
    Elf,
    Dwarf,
    Human
}

impl Races {
    fn get_stats(&self) -> Stats {
        match self {
            Races::Elf => Stats {
                strength: 1,
                agility: 2,
                intelligence: 1,
            },
            Races::Dwarf => Stats {
                strength: 2,
                agility: 1,
                intelligence: 1,
            },
            Races::Human => Stats {
                strength: 1,
                agility: 1,
                intelligence: 1,
            },
        }
    }
}

struct Character {
    name: String,
    class: Classes,
    race: Races,
    max_hp: u32,
    current_hp: u32,
    level: u32,
    xp: u32,
    max_xp: u32,
    stats: Stats,
    stats_gain_per_level: Stats,
    free_stats_points: u32,
}

#[derive(Debug)]
struct Stats {
    strength: u32,
    agility: u32,
    intelligence: u32,
}

impl Character {
    fn new(name: String, class: Classes, race: Races) -> Character {
        let class_stats = class.get_stats();
        let race_stats = race.get_stats();

        let character = Character {
            name,
            class,
            race,
            max_hp: class_stats.strength * race_stats.strength * 10,
            current_hp: class_stats.strength * race_stats.strength * 10,
            level: 1,
            xp: 0,
            max_xp: 100,
            stats_gain_per_level: race_stats,
            stats: class_stats,
            free_stats_points: 0
        };

        // character.print_stats();
        character
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.xp -= self.max_xp;
        self.max_xp += (self.max_xp as f32 * 0.2) as u32; // Increase max XP for the next level
        self.stats.strength += self.stats_gain_per_level.strength; // Increase strength
        self.stats.agility += self.stats_gain_per_level.agility; // Increase agility
        self.stats.intelligence += self.stats_gain_per_level.intelligence; // Increase intelligence
        self.max_hp += self.stats_gain_per_level.strength * 10; // Increase max HP for the next level
        self.current_hp = self.max_hp; // Restore current HP to max HP
        
        if self.level % 10 == 0 {
            self.free_stats_points += 10;
        } else {
            self.free_stats_points += 3;
        }

        
    }

    fn gain_xp(&mut self, xp: u32) {
        self.xp += xp;
        loop {
            if self.max_xp > self.xp{
                break;
            }
            self.level_up()
        }
    }

    fn take_damage(&mut self, damage: u32) {
        if damage >= self.current_hp {
            self.current_hp = 0;
            println!("Character {} {} is dead.", self.name, self.level)
        } else {
            self.current_hp -= damage;
        }
    }

    fn print_stats(&self) {
        println!("---");
        println!("Name: {}", self.name);
        println!("Race: {:?}", self.race);
        println!("Class: {:?}", self.class);
        println!("Level: {}", self.level);
        println!("XP: {}/{}", self.xp, self.max_xp);
        println!("Stats: {:?}", self.stats);
        println!("Current HP: {}/{}", self.current_hp, self.max_hp);
        println!("---");
    } 

    fn print(&self) {
        println!("{} {}/{}", self.name, self.current_hp, self.max_hp);
        println!("{:?} {:?} {} {}/{} {}",self.race, self.class, self.level, self.xp, self.max_xp, self.free_stats_points);
        println!("Stats: {:?}", self.stats);
    }

    fn attack(&self, who: &mut Mob) {
        who.take_damage(self.stats.strength + self.stats.agility + self.stats.intelligence);
    }

    fn add_point(&mut self, stats: Stats) {
        if stats.strength + stats.agility + stats.intelligence > self.free_stats_points{
            println!("Not enough points");
        } else {
            self.stats.strength += stats.strength;
            self.stats.agility += stats.agility;
            self.stats.intelligence += stats.intelligence;
        }
    }
}

struct Mob{
    name: String,
    max_hp: u32,
    current_hp: u32,
    level: u32,
    stats: Stats
}

impl Mob {
    fn new(name: String, level: u32, stats_gain_per_level: Stats) -> Mob{
        let stats = Stats { strength: level * stats_gain_per_level.strength, agility: level * stats_gain_per_level.agility, intelligence: level * stats_gain_per_level.intelligence };
        
        Mob{
            name,
            max_hp: level * stats_gain_per_level.strength * 5,
            current_hp: level * stats_gain_per_level.strength * 5,
            level,
            stats
        }
    }

    fn attack(&self, who: &mut Character) {
        who.take_damage(self.stats.strength + self.stats.agility + self.stats.intelligence);
    }

    fn take_damage(&mut self, damage: u32) {
        if damage >= self.current_hp {
            self.current_hp = 0;
            println!("Mob {} {} is dead.", self.name, self.level)
        } else {
            self.current_hp -= damage;
        }
    }
}

fn main() {
    let warrior = Character::new(String::from("Gimli"), Classes::Warrior, Races::Dwarf);
    let mage = Character::new(String::from("Gandalf"), Classes::Mage, Races::Human);
    let archer = Character::new(String::from("Legolas"), Classes::Archer, Races::Elf);

    let mut characters = vec![warrior, mage, archer];

    for character in &mut characters {
        character.gain_xp(81000); // Gain 81000 XP
        character.take_damage(20); // Take 20 damage
        character.print();
        println!("-------------");
    }

    let mut hero = Character::new(String::from("Lisa"), Classes::Warrior, Races::Elf); 
    hero.gain_xp(20000);
    hero.add_point(Stats { strength: 50, agility: 20, intelligence: 4 });
    let mut zombie = Mob::new(String::from("Zombie"), 20, Stats { strength: 4, agility: 1, intelligence: 1 });
    println!("--- Fight ---");
    hero.print();

    loop{
        println!("-------------");
        println!("Character: {} | Mob: {}",hero.current_hp, zombie.current_hp);
        println!("-------------");

        hero.attack(&mut zombie);
        if zombie.current_hp == 0 {
            break;
        }
        
        zombie.attack(&mut hero);
        if hero.current_hp == 0 {
            break
        };
    }
}
