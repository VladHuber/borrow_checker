fn main() {
    let mut sword = String::from("Железный меч");
    sell(sword);
    // println!("{}", sword);
}

fn sell(item: String) {
    println!("Продан {}", item);
}