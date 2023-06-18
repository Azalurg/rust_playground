use std::collections::HashMap;

fn main() {
    // Vector
   let mut inventory = vec!["book", "crystal ball", "carrot"];
   inventory.push("mana potion");
   for item in inventory.iter(){
    println!("Got : {}", item)
   }

   inventory.insert(1, "coin");
   for item in &inventory{
    println!("Got : {}", item)
   }

   inventory.remove(0);
   println!("{:?}", inventory);

// String 
   let mut spellbook = String::new();

   spellbook.push_str("fire ball, ");
   spellbook += &"ice wall, ".to_string();
   spellbook += &String::from("counterstrike");

   println!("{}", spellbook);
   for character in spellbook.chars(){
    print!("{}", character)
   }
   println!();
   println!("Spellbook len: {}", spellbook.len());

   // HashMap
   let mut enchanted_items = HashMap::new();
   enchanted_items.insert(inventory[0].to_string(), "Gold coin");
   enchanted_items.insert(inventory[1].to_string(), "Powerful weapon");
   enchanted_items.insert(inventory[2].to_string(), "Quick snack");

   for (key, value) in &enchanted_items{
    println!("{}: {}", key, value)
   }

   let carrot_description = enchanted_items.get(&String::from("carrot"));
   println!("Carrot description: {:?}", carrot_description);

   //Create value if not exists
   enchanted_items.entry(inventory[3].to_string()).or_insert("Useful drink");
   enchanted_items.entry(inventory[3].to_string()).or_insert("Something, IDK");

   // Count words in string
   let mut spells = HashMap::new();
   spellbook += &", ice wall".to_string();

   for spell in spellbook.split_terminator(", "){
        let count = spells.entry(spell).or_insert(0);
        *count +=1
   }
   println!("{:?}", spells)
}
