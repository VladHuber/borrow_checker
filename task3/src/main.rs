struct Player {
    hp: u32,
}

fn main() {
    let mut player = Player{
        hp: 100
    };
    println!("{}", player.hp);
    heal(&mut player);
    println!("{}", player.hp);
}

fn heal(player: &mut Player) {
    player.hp += 20;
}
