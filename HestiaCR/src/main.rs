use crate::game::GameProcess;

mod memory;
mod game;
mod jvm;

fn main() {
    let game = GameProcess::craftrise().unwrap();
    //let qwe:u8 = game.read(game.jvm_ptr);
}
