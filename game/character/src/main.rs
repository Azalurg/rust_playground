/*
    Simple character implementations 
*/

#[derive(Debug)]
enum Class {
    Warrior,
    Mage,
    Archer,
}

struct Character {
    name: String,
    class: Class,
    max_hp: u32,
    current_hp: u32,
    level: u32,
    xp: u32,
    max_xp: u32,
    stats: Stats,
    stats_gain_per_level: Stats,
}

#[derive(Debug)]
struct Stats {
    strength: u32,
    agility: u32,
    intelligence: u32,
}

impl Character {
    fn new(name: String, class: Class) -> Character {
        let stats = match class {
            Class::Warrior => Stats {
                strength: 10,
                agility: 5,
                intelligence: 5,
            },
            Class::Mage => Stats {
                strength: 5,
                agility: 5,
                intelligence: 10,
            },
            Class::Archer => Stats {
                strength: 5,
                agility: 10,
                intelligence: 5,
            },
        };

        Character {
            name,
            class,
            max_hp: stats.strength * 10,
            current_hp: stats.strength * 10,
            level: 1,
            xp: 0,
            max_xp: 100,
            stats_gain_per_level: Stats {
                strength: stats.strength / 2,
                agility: stats.agility / 2,
                intelligence: stats.intelligence / 2,
            },
            stats
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.xp -= self.max_xp;
        self.max_xp += 100; // Increase max XP for the next level
        self.stats.strength += self.stats_gain_per_level.strength; // Increase strength
        self.stats.agility += self.stats_gain_per_level.agility; // Increase agility
        self.stats.intelligence += self.stats_gain_per_level.intelligence; // Increase intelligence
        self.max_hp += self.stats_gain_per_level.strength * 10; // Increase max HP for the next level
        self.current_hp = self.max_hp; // Restore current HP to max HP
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
        } else {
            self.current_hp -= damage;
        }
    }

    fn print_stats(&self) {
        println!("Name: {}", self.name);
        println!("Class: {:?}", self.class);
        println!("Level: {}", self.level);
        println!("XP: {}/{}", self.xp, self.max_xp);
        println!("Stats: {:?}", self.stats);
        println!("Current HP: {}/{}", self.current_hp, self.max_hp);
    } 
}

fn main() {
    let warrior = Character::new(String::from("Conan"), Class::Warrior);
    let mage = Character::new(String::from("Gandalf"), Class::Mage);
    let archer = Character::new(String::from("Legolas"), Class::Archer);

    let mut characters = vec![warrior, mage, archer];

    for character in &mut characters {
        character.gain_xp(1510); // Gain 1510 XP
        character.take_damage(20); // Take 20 damage
        character.print_stats();
        println!("---------------------");
    }
}
