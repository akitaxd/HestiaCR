use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use winapi::um::winuser::{GetAsyncKeyState, PostMessageA, SendInput, INPUT, VK_LBUTTON, MK_LBUTTON, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, WM_LBUTTONDOWN, WM_LBUTTONUP};
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;
use crate::module::Tick;
use crate::module::utils::is_visible;

pub struct TriggerBot {
    pub sleep:u64,
    pub enabled:bool,
    pub last_clicked:u128,
    pub last_war:u128,
}
impl Tick for TriggerBot {
    fn tick(&mut self, game: &mut GameProcess) -> Option<()>
    {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        if (now - self.last_clicked) > self.sleep as u128 {
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
                    if mouse_over_entity != 0 || now - self.last_war < 2000 {
                        if (mouse_over_entity == 0 || is_visible(mouse_over_entity,game)?) && GetAsyncKeyState(VK_LBUTTON) != 0 {
                            if mouse_over_entity != 0 {
                                self.last_war = now;
                            }
                            self.last_clicked = now;
                            PostMessageA(game.hWindow, WM_LBUTTONDOWN, MK_LBUTTON as usize, (100 & 0xFFFF) | ((100 & 0xFFFF) << 16));
                            PostMessageA(game.hWindow, WM_LBUTTONUP, 0, (100 & 0xFFFF) | ((100 & 0xFFFF) << 16));
                        }
                    }
                }
            }
        }
        Some(())
    }
}