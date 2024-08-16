use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use winapi::um::winuser::{SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT};
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;
use crate::module::Tick;

pub struct TriggerBot {
    pub sleep:u64,
    pub enabled:bool,
    pub last_clicked:u128,
}
impl Tick for TriggerBot {
    fn tick(&mut self, game: &GameProcess) -> Option<()>
    {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        if (now - self.last_clicked) > self.sleep as u128 {
            self.last_clicked = now;
            let klass = game.find_class("com/craftrise/client/S")?;
            let klass_2 = game.find_class("com/craftrise/pV")?;
            let field_id_4 = game.get_field_id(klass, "bw", "Lcom/craftrise/client/dG;")?;
            let field_id_3 = game.get_field_id(klass_2, "a", "Lcom/craftrise/m9;")?;
            let field_id_2 = game.get_field_id(klass, "H", "Lcom/craftrise/pV;")?;
            let field_id = game.get_field_id(klass, "cj", "Lcom/craftrise/client/S;")?;
            unsafe {
                let minecraft = game.get_static_object_field(klass, field_id)?;
                if minecraft == 0 {
                    return None;
                }
                let gui = game.get_object_field(minecraft, field_id_4)?;
                if gui == 0 {
                    let mouse_over_object = game.get_object_field(minecraft, field_id_2)?;
                    if mouse_over_object == 0 {
                        return None;
                    }
                    let mouse_over_entity = game.get_object_field(mouse_over_object, field_id_3)?;

                    if mouse_over_entity != 0 {
                        if game.is_instance_of(mouse_over_entity,game.find_class("com/craftrise/mg")?) {
                            let mut input = INPUT {
                                type_: INPUT_MOUSE,
                                u: std::mem::zeroed(),
                            };
                            *input.u.mi_mut() = MOUSEINPUT {
                                dx: 0,
                                dy: 0,
                                mouseData: 0,
                                dwFlags: MOUSEEVENTF_LEFTDOWN,
                                time: 0,
                                dwExtraInfo: 0,
                            };
                            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
                            input.u.mi_mut().dwFlags = MOUSEEVENTF_LEFTUP;
                            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
                        }
                    }
                }
            }
        }
        Some(())
    }
}