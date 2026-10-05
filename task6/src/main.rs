#[derive(Debug)]
struct Npc {
    name: String,
}

fn main() {
    let mut npc = Npc{ name: "Hello".to_string()};
let r1 = &npc;
println!("{}", r1.name);
println!("{:?}", r1);
let r2 = &npc;
println!("{:?} {:?}", r1, r2);
}

fn greet(npc: Npc){
    println!("Привет NPC: {}", npc.name);
}
// fn greet_link(npc: &Npc){
//     println!("Привет NPC: {}", npc.name);
// }
fn greet_mut_link(npc: &mut Npc) {
    npc.name.push_str(" Великий");
}