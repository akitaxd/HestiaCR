pub mod trigger_bot;
mod aim_assist;
mod utils;
mod fast_place;

use crate::game::GameProcess;
use crate::module::aim_assist::AimAssist;
use crate::module::fast_place::FastPlace;
use crate::module::trigger_bot::TriggerBot;

pub struct ModuleCollection {
    pub trigger_bot: TriggerBot,
    pub aim_assist: AimAssist,
    pub fast_place: FastPlace,
}
impl ModuleCollection {
    pub fn new() -> ModuleCollection   {
        ModuleCollection {
            trigger_bot: TriggerBot { sleep: 80, enabled: false, last_clicked: 0, last_war: 0 },
            aim_assist: AimAssist { enabled: false, speed: 1, fov: 70.0, last_target: Default::default() },
            fast_place: FastPlace {enabled: false, disable_on_rod: false },
        }
    }
    pub fn tick(&mut self, game:&mut GameProcess) -> Option<()>{
        if self.trigger_bot.enabled {
            self.trigger_bot.tick(game);
        }
        if self.aim_assist.enabled {
            self.aim_assist.tick(game);
        }
        if self.fast_place.enabled {
            self.fast_place.tick(game);
        }
        Some(())
    }

}
pub trait Tick {
    fn tick(&mut self,game:&mut GameProcess) -> Option<()>;
}