use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;

mod memory;
mod game;

fn main() {
    let game = GameProcess::craftrise().unwrap();
    let klass = game.find_class("cr/launcher/Config");
    let field_id = game.get_field_id(klass,"newRelease","Ljava/lang/String;");
}
