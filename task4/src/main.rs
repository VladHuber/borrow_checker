fn main() {
    let mut inventory = vec![
        String::from("Меч"), String::from("Лук")
        ];
    let first = &inventory[0].clone();
    inventory.push(String::from("Щит"));
    println!("{}",first);

}
