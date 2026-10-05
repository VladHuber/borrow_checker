fn main() {
    let inventory = vec![
        String::from("Меч"),
        String::from("Лук"),
    ];
    match find_item(&inventory, "Меч") {
        Some(item) => println!("{}", item),
        None => println!("Ничего")
    }
    
}
fn find_item<'a>(inventory: &'a Vec<String>, search_item: &str) -> Option<&'a String>{
    for item in inventory  {
        if search_item.contains(item) {
            return Some(item);
        }
    };
    return None;
}