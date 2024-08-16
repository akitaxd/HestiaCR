use std::f64::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};
use winapi::um::winnt::LONG;
use winapi::um::winuser::{SendInput, INPUT, INPUT_MOUSE, LPINPUT, MOUSEEVENTF_MOVE};
use crate::game::GameProcess;
use crate::game::jvm::JVM_Control;
use crate::module::Tick;

pub struct Position {
    x:f64,
    y:f64,
    z:f64,
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
        let dy = other.y - (self.y + 0.4); // Assuming eye height as 1.8 meters
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
    fn new(target:u64) -> LastTarget {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        LastTarget {
            target,
            time,
        }
    }
    fn time_since(&self) -> u128 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        now - self.time
    }
}
pub fn rotations(entity:u64,game_process: &GameProcess) -> Option<[f32; 2]> {
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
pub fn get_entity_position(entity:u64,game_process: &GameProcess) -> Option<Position>
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
    pub last_target: LastTarget,
}
impl Tick for AimAssist {
    fn tick(&mut self, game: &GameProcess) -> Option<()>{
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
        if self.last_target.time_since() > 1000 && mouse_over_entity != 0 {
            if game.is_instance_of(mouse_over_entity,game.find_class("com/craftrise/mg")?) {
                self.last_target = LastTarget::new(mouse_over_entity);
            }
        }
        if self.last_target.time_since() < 5000 && self.last_target.target != 0 {
            let the_player = game.get_static_object_field(klass_3 , field_id_4)?;
            let my_position = get_entity_position(the_player,&game)?;
            let position = get_entity_position(self.last_target.target,&game)?;
            let distance = position.distance_to(&my_position);
            if distance > 1.0 && distance < 4.7 {
                let current_rotations = rotations(the_player,&game)?;
                let rotations = my_position.rotation_to(&position);
                if (current_rotations[0] - rotations[0]).abs() > self.speed as f32 * 2.0 || mouse_over_entity != self.last_target.target {
                    if current_rotations[0] > rotations[0] + self.speed as f32 * 2.0 {
                        for _ in 0..self.speed {
                            mouse_move(-1,0);
                        }
                    }
                    if current_rotations[0] < rotations[0] - self.speed as f32* 2.0{
                        for _ in 0..self.speed {
                            mouse_move(1,0);
                        }
                    }
                }

            }
        }
        Some(())
    }
}