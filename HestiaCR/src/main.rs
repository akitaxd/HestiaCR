use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;

mod memory;
mod game;

fn main() {
    let game = GameProcess::craftrise().unwrap();
    let klass = game.find_class("cr/launcher/Config");
    println!("{klass}")
}
