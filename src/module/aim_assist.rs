use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;
use crate::module::Tick;
use crate::module::utils::{get_entity_position, is_visible, mouse_move, rotations, LastTarget, Position};

pub struct AimAssist {
    pub enabled:bool,
    pub speed:i64,
    pub fov:f32,
    pub last_target: LastTarget,
}
impl Tick for AimAssist {
    fn tick(&mut self, game: &mut GameProcess) -> Option<()>{
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
        if distance > 15.0 {
            self.last_target = LastTarget::new(0);
        }
        if mouse_over_entity != 0 {
            if self.last_target.target == 0 || distance > 5.0 {
                self.last_target = LastTarget::new(mouse_over_entity);
            }
        }
        position = get_entity_position(self.last_target.target,game)?;
        distance = position.distance_to(&my_position);
        unsafe {
            if self.last_target.target != 0 && is_visible(self.last_target.target,game)? && GetAsyncKeyState(VK_LBUTTON) != 0 {
                if distance > 1.0 && distance < 6.0 {
                    let current_rotations = rotations(the_player,game)?;
                    let rotations = my_position.rotation_to(&position);
                    let diff = (current_rotations[0] - rotations[0]).abs();
                    let diff_2 = (current_rotations[1] - rotations[1]).abs();
                    if diff > self.speed as f32 && diff < self.fov && diff_2 < self.fov {
                        if current_rotations[0] > rotations[0] + self.speed as f32 {
                            for _ in 0..self.speed {
                                mouse_move(-1,0);
                            }
                        }
                        if current_rotations[0] < rotations[0] - self.speed as f32  {
                            for _ in 0..self.speed {
                                mouse_move(1,0);
                            }
                        }
                    }

                }
            }
        }
        Some(())
    }
}