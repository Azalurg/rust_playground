#[derive(Debug)]
struct Monster {
    name: String,
    hp: i32,
    level: i32,
    angry: bool,
}

impl Monster {
    fn hit(&self, mut m: Monster){
        m.hp -= self.level;
        println!("{} hit {} with {} dmg.", self.name, m.name, self.level)
    }
}

impl Monster{
    fn basic(name: String) -> Monster{
        Monster { name, hp: 100, level: 1, angry: false }
    }
}

fn main() {
    let mut monster1 = Monster{
        name: String::from("Bob"),
        hp: 100,
        level: 1,
        angry: false
    };

    monster1.level = 2;
    println!("{:#?}", monster1);

    let monster2 = build_monster(String::from("Alice"), 120);
    let monster3 = copy_monster(monster2);
    monster1.hit(monster3);

    let monster4 = Monster::basic(String::from("Rob"));
}

fn build_monster(name: String, hp: i32) -> Monster{
    Monster { name, hp, level: 1, angry: false }
}

fn copy_monster(m: Monster) -> Monster {
    Monster { level: 1, angry: false, ..m }
}
