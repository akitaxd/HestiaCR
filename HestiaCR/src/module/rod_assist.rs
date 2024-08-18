use crate::module::utils::get_predicted_entity_position;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use winapi::um::winuser::{GetAsyncKeyState, PostMessageA, MK_LBUTTON, VK_RBUTTON,WM_RBUTTONDOWN, WM_RBUTTONUP};
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;
use crate::module::Tick;
use crate::module::utils::{get_entity_position, get_held_item, get_stack_count, is_rod, is_visible, max_rc_delay, reset_rc_delay, rotations, switch, w_rotations, LastTarget, Position};

pub struct RodAssist {
    pub pullback_enabled:bool,
    pub aimbot_enabled:bool,
    pub pullback_delay:u64,
    pub trigger_point: i32,
    pub pullback_point:i32,
    pub switch_on_pullback:bool,
    pub switch_delay:u64,
    pub last_target: LastTarget,
    pub last_pullback:u128,
    pub last_tick_clicked:bool,
    pub last_held_item:u64,
}
impl Tick for RodAssist {
    fn tick(&mut self, game: &mut GameProcess) -> Option<()> {
        let mut rod:bool = false;
        'pullback:{
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let klass = game.find_class("com/craftrise/client/S")?;
            let klass_2 = game.find_class("cr/launcher/main/a")?;
            let field_id = game.get_field_id(klass, "cj", "Lcom/craftrise/client/S;")?;
            let field_id_2 = game.get_field_id(klass_2, "q", "Lcom/craftrise/client/fa;")?;
            let field_id_3 = game.get_field_id(klass, "cB", "I")?;
            let minecraft = game.get_static_object_field(klass, field_id)?;
            let the_player = game.get_static_object_field(klass_2, field_id_2)?;
            let held_item = get_held_item(the_player, game)?;
            let mut count = get_stack_count(held_item, game)?;
            if count == 1 {
                let delay: i32 = game.get_value_field(minecraft, field_id_3)?;
                if delay == 0 {
                    rod = true;
                }
                if delay == self.trigger_point && now - self.last_pullback > self.pullback_delay as u128 {
                    if self.pullback_enabled {
                        game.write_field(minecraft, field_id_3, self.pullback_point);
                        if self.switch_on_pullback {
                            switch(the_player, self.switch_delay, game)?;
                        }
                    }
                    self.last_pullback = now;
                }
            }
        }
        if self.aimbot_enabled {
            let klass = game.find_class("com/craftrise/client/S")?;
            let klass_2 = game.find_class("com/craftrise/pV")?;
            let klass_3 = game.find_class("cr/launcher/main/a")?;
            let field_id_5 = game.get_field_id(klass, "bw", "Lcom/craftrise/client/dG;")?;
            let field_id_4 = game.get_field_id(klass_3,"q","Lcom/craftrise/client/fa;")?;
            let field_id_3 = game.get_field_id(klass_2, "a", "Lcom/craftrise/m9;")?;
            let field_id_2 = game.get_field_id(klass, "H", "Lcom/craftrise/pV;")?;
            let field_id = game.get_field_id(klass, "cj", "Lcom/craftrise/client/S;")?;
            let minecraft = game.get_static_object_field(klass, field_id)?;
            if minecraft == 0 {
                return None;
            }
            let mouse_over_object = game.get_object_field(minecraft, field_id_2)?;
            let gui = game.get_object_field(minecraft, field_id_5)?;
            if gui != 0 {
                return None;
            }
            let mut mouse_over_entity:u64  =  0;
            if mouse_over_object != 0 {mouse_over_entity = game.get_object_field(mouse_over_object, field_id_3)?;}
            let the_player = game.get_static_object_field(klass_3 , field_id_4)?;
            let my_position = get_entity_position(the_player,game)?;
            let mut position = Position { x: 0.0, y: 0.0, z: 0.0 };
            let mut distance = 99f64;
            if self.last_target.target != 0 {
                position = get_entity_position(self.last_target.target,game)?;
                distance = position.distance_to(&my_position);
            }
            if mouse_over_entity != 0 {
                if self.last_target.target == 0 || distance > 15.0 {
                    self.last_target = LastTarget::new(mouse_over_entity);
                }
            }
            unsafe {
                if self.last_target.target != 0 && is_visible(self.last_target.target,game)? {
                    let amp = {
                        distance * 1.15
                    };
                    position = get_predicted_entity_position(self.last_target.target,amp,game)?;
                    distance = position.distance_to(&my_position);
                    if distance > 1.0 && distance < 10.0 {
                        let held_item = get_held_item(the_player,game)?;
                        if  held_item != self.last_held_item && is_rod(held_item,game)? {
                            let rotations = my_position.rotation_to(&position);
                            {
                                w_rotations(the_player,rotations,game);
                            }
                        }
                        self.last_held_item = held_item;
                    }
                }
            }
        }
        Some(())

    }
}