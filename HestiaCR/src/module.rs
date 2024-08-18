pub mod trigger_bot;
mod aim_assist;
mod utils;
mod fast_place;
mod rod_assist;

use crate::game::GameProcess;
use crate::module::aim_assist::AimAssist;
use crate::module::fast_place::FastPlace;
use crate::module::rod_assist::RodAssist;
use crate::module::trigger_bot::TriggerBot;

pub struct ModuleCollection {
    pub trigger_bot: TriggerBot,
    pub aim_assist: AimAssist,
    pub fast_place: FastPlace,
    pub rod_assist: RodAssist,
}
impl ModuleCollection {
    pub fn new() -> ModuleCollection   {
        ModuleCollection {
            trigger_bot: TriggerBot { sleep: 80, enabled: false, last_clicked: 0, last_war: 0 },
            aim_assist: AimAssist { enabled: false, speed: 1, fov: 70.0, last_target: Default::default() },
            fast_place: FastPlace {enabled: false, disable_on_rod: false, last_tick_item_count: 1 },
            rod_assist: RodAssist { pullback_enabled: false, pullback_delay: 300, trigger_point: 2, pullback_point: 0, switch_on_pullback: true, switch_delay: 25, last_target: Default::default(), last_pullback: 0, aimbot_enabled: false, last_tick_clicked: false, last_held_item: 0 },
        }
    }
    pub fn tick(&mut self, game:&mut GameProcess) -> Option<()>{
        self.rod_assist.tick(game);
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