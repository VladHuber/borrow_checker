fn main() {
    let inventory = vec![String::from("Меч"), String::from("Лук")];
    for item in inventory.iter() {
        println!("{}", item);
    }
    println!("{:?}", inventory);
}
