pub mod trigger_bot;

use crate::game::GameProcess;
use crate::module::trigger_bot::TriggerBot;

pub struct ModuleCollection {
    pub trigger_bot: TriggerBot,
}
impl ModuleCollection {
    pub fn new() -> ModuleCollection   {
        ModuleCollection {
            trigger_bot: TriggerBot { sleep: 80, enabled: false },
        }
    }
    pub fn tick(&self,game:&GameProcess) {
        if self.trigger_bot.enabled {
            self.trigger_bot.tick(game);
        }
    }

}
pub trait Tick {
    fn tick(&self,game:&GameProcess);
}