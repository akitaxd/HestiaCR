pub mod trigger_bot;
mod aim_assist;

use crate::game::GameProcess;
use crate::module::aim_assist::AimAssist;
use crate::module::trigger_bot::TriggerBot;

pub struct ModuleCollection {
    pub trigger_bot: TriggerBot,
    pub aim_assist: AimAssist,
}
impl ModuleCollection {
    pub fn new() -> ModuleCollection   {
        ModuleCollection {
            trigger_bot: TriggerBot { sleep: 80, enabled: false, last_clicked: 0, last_war: 0 },
            aim_assist: AimAssist { enabled: false, speed: 1, fov: 70.0, last_target: Default::default() },
        }
    }
    pub fn tick(&mut self, game:&GameProcess) -> Option<()>{
        if self.trigger_bot.enabled {
            self.trigger_bot.tick(game);
        }
        if self.aim_assist.enabled {
            self.aim_assist.tick(game);
        }
        Some(())
    }

}
pub trait Tick {
    fn tick(&mut self,game:&GameProcess) -> Option<()>;
}