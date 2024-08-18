use crate::game::{GameProcess, Readable};
use crate::game::jvm::JVM_Control;
use crate::module::Tick;
use crate::module::utils::{get_held_item, get_stack_count};

pub struct FastPlace {
    pub enabled:bool,
    pub disable_on_rod:bool,
    pub last_tick_item_count:i32,
}
impl Tick for FastPlace {
    fn tick(&mut self, game: &mut GameProcess) -> Option<()> {
        let klass = game.find_class("com/craftrise/client/S")?;
        let klass_2 = game.find_class("cr/launcher/main/a")?;
        let field_id = game.get_field_id(klass, "cj", "Lcom/craftrise/client/S;")?;
        let field_id_2 = game.get_field_id(klass_2,"q","Lcom/craftrise/client/fa;")?;
        let field_id_3 = game.get_field_id(klass,"cB","I")?;
        let minecraft = game.get_static_object_field(klass, field_id)?;
        let the_player = game.get_static_object_field(klass_2 , field_id_2)?;
        let held_item = get_held_item(the_player,game)?;
        let mut count = get_stack_count(held_item,game)?;
        if !self.disable_on_rod {
            game.write_field(minecraft,field_id_3,0i32);
            return Some(())
        }
        if count != 1 {
            game.write_field(minecraft,field_id_3,0i32);
        }
        Some(())
    }
}