use winapi::um::winnt::LONG;
use winapi::um::winuser::SendInput;
use winapi::um::winuser::MOUSEEVENTF_MOVE;
use winapi::um::winuser::INPUT_MOUSE;
use std::f64::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};
use winapi::um::winuser::INPUT;
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;
use winapi::um::winuser::LPINPUT;
pub struct Position {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}
impl Position {
    pub fn distance_to(&self,other:&Position) -> f64 {
        let f1 = self.x - other.x;
        let f2 = self.y - other.y;
        let f3 = self.z - other.z;
        let v = f1 * f1 + f2 * f2 + f3 * f3;
        v.sqrt()
    }
    pub fn rotation_to(&self, other: &Position) -> [f32; 2] {
        let dx = other.x - self.x;
        let dy = other.y - (self.y + 0.4);
        let dz = other.z - self.z;
        let dist = (dx * dx + dz * dz).sqrt();
        let mut yaw = (dz.atan2(dx) * 180.0 / PI) - 90.0;
        while yaw > 180.0 {
            yaw -= 360.0;
        }
        while yaw < -180.0 {
            yaw += 360.0;
        }
        let mut pitch = -(dy.atan2(dist) * 180.0 / PI);
        [yaw as f32, pitch as f32]
    }
}
pub struct LastTarget {
    pub target:u64,
    pub time:u128,
}
impl Default for LastTarget {
    fn default() -> Self {
        LastTarget { target: 0, time: 0 }
    }
}
impl LastTarget {
    pub fn new(target:u64) -> LastTarget {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        LastTarget {
            target,
            time,
        }
    }
    pub fn time_since(&self) -> u128 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        now - self.time
    }
}
pub fn rotations(entity:u64, game_process: &mut GameProcess) -> Option<[f32; 2]> {
    let entity_klass = game_process.find_class("com/craftrise/m9")?;
    let yaw_id = game_process.get_field_id(entity_klass,"bL","F")?;
    let pitch_id = game_process.get_field_id(entity_klass,"N","F")?;
    let mut yaw:f32 = game_process.get_value_field(entity,yaw_id)?;
    let mut pitch:f32 = game_process.get_value_field(entity,pitch_id)?;
    while yaw > 180.0 {
        yaw -= 360.0;
    }
    while yaw < -180.0 {
        yaw += 360.0;
    }
    Some([yaw,pitch])
}
pub fn get_entity_position(entity:u64,game_process: &mut GameProcess) -> Option<Position>
{
    let entity_klass = game_process.find_class("com/craftrise/m9")?;
    let pos_x_id = game_process.get_field_id(entity_klass,"bE","D")?;
    let pos_y_id = game_process.get_field_id(entity_klass,"aY","D")?;
    let pos_z_id = game_process.get_field_id(entity_klass,"bH","D")?;
    let pos_x:f64 = game_process.get_value_field(entity,pos_x_id)?;
    let pos_y:f64 = game_process.get_value_field(entity,pos_y_id)?;
    let pos_z:f64 = game_process.get_value_field(entity,pos_z_id)?;
    Some(Position {
        x: pos_x,
        y: pos_y,
        z: pos_z,
    })
}
pub fn is_visible(entity:u64,game_process:&mut GameProcess) -> Option<bool>
{
    let byte_klass = game_process.find_class("java/lang/Byte")?;
    let entity_klass = game_process.find_class("com/craftrise/m9")?;
    let data_watcher = game_process.find_class("com/craftrise/qN")?;
    let watchable_klass = game_process.find_class("com/craftrise/qN$a")?;
    let byte_value_id = game_process.get_field_id(byte_klass,"value","B")?;
    let values_id = game_process.get_field_id(data_watcher,"c","[Lcom/craftrise/qN$a;")?;
    let data_watcher_id = game_process.get_field_id(entity_klass,"bA","Lcom/craftrise/qN;")?;
    let watchable_byte_value_id = game_process.get_field_id(watchable_klass,"a", "Ljava/lang/Byte;")?;
    let data_watcher_obj = game_process.get_object_field(entity,data_watcher_id)?;
    let values = game_process.get_object_field(data_watcher_obj,values_id)?;
    let flags:Vec<u64> = game_process.get_array_elements(values,1)?;
    let flag_byte_base = *flags.get(0)?;
    let watchable_byte_value = game_process.get_object_field(flag_byte_base,watchable_byte_value_id)?;
    let flag:i8 = game_process.get_value_field(watchable_byte_value,byte_value_id)?;
    let visible = (flag & 1 << 5) == 0;
    Some(visible)
}
pub fn mouse_move(x:i64,y:i64)
{

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };

    let mouse_input = unsafe {
        std::mem::transmute::<_, *mut winapi::um::winuser::MOUSEINPUT>(&mut input.u)
    };

    unsafe {
        (*mouse_input).dx = x as LONG;
        (*mouse_input).dy = y as LONG;
        (*mouse_input).mouseData = 0;
        (*mouse_input).dwFlags = MOUSEEVENTF_MOVE;
        (*mouse_input).time = 0;
        (*mouse_input).dwExtraInfo = 0;
    }

    unsafe {
        SendInput(1, &mut input as *mut _ as LPINPUT, std::mem::size_of::<INPUT>() as i32);
    }
}
pub struct AimAssist {
    pub enabled:bool,
    pub speed:i64,
    pub fov:f32,
    pub last_target: LastTarget,
}