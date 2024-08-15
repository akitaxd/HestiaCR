use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;

mod memory;
mod game;

fn main() {
    let game = GameProcess::custom("java.exe").unwrap();
    game.find_class("java/lang/String");
}
