use std::fmt::Debug;
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{GetAsyncKeyState, SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, VK_LBUTTON};
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;

mod memory;
mod game;

fn main() {
    let game = GameProcess::craftrise().unwrap();
    let klass = game.find_class("com/craftrise/client/S");
    let klass_2 = game.find_class("com/craftrise/pV");
    let field_id_3 = game.get_field_id(klass_2,"a","Lcom/craftrise/m9;");
    let field_id_2 = game.get_field_id(klass,"H","Lcom/craftrise/pV;");
    let field_id = game.get_field_id(klass,"cj","Lcom/craftrise/client/S;");
    let minecraft = game.get_static_object_field(klass,field_id);
    loop {
        unsafe {
            if GetAsyncKeyState(VK_LBUTTON) != 0 {
                let mouse_over_object = game.get_object_field(minecraft,field_id_2);
                let mouse_over_entity = game.get_object_field(mouse_over_object,field_id_3);
                if mouse_over_entity != 0 {
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
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

}
