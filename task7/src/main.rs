#[derive(Debug)]
struct Item{
    name: String
}
struct Player{
    name: String
}

fn main() {
    let mut inventory: Vec<Item> = vec![
        Item{
            name: "Sword".to_string()
        },        
        Item{
            name: "Shield".to_string()
        }
    ];

let sword = inventory[0].name.clone();


inventory.push(Item { name: "Poison".to_string() });
println!("{}", sword);
}
